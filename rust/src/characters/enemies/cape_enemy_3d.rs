use bevy::prelude::*;
use godot::{
    classes::{Area3D, CharacterBody3D},
    prelude::*,
};
use godot_bevy::{
    interop::{Area3DSignals, GodotAccess, GodotNodeHandle},
    plugins::signals::GodotSignals,
    prelude::GodotNode,
};

use crate::{
    characters::{
        components::stats::{Damage, Gravity, Health, Speed},
        player_3d::Player3DNode,
    },
    events::combat::{DiedInCombat, EnterCombatEvent},
    state::GameState,
};

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CharacterBody3D, class_name = CapeEnemy3D)]
#[gdbevy(
    require(speed: Speed, as = f32, default = 150.0),
    require(health: Health, as = f32, default = 50.0),
    require(damage: Damage, as = f32, default = 5.0),
    require(enemy_gravity: Gravity, as = f32, default = 980.0),
    require(enemy_initialized: Initialized, as = bool, default = false),
)]
pub struct CapeEnemyNode3D;

fn get_custom_character_body_3d(
    handle: &GodotNodeHandle,
    godot: &mut GodotAccess,
) -> Option<Gd<CharacterBody3D>> {
    let body = godot.try_get::<CharacterBody3D>(*handle)?;
    Some(body)
}

fn get_combat_area_node(body: &Gd<CharacterBody3D>) -> Option<Gd<Area3D>> {
    let area_handle = body.get_node_or_null("combat_area")?;

    let Ok(area) = area_handle.try_cast::<Area3D>() else {
        return None;
    };

    Some(area)
}

#[derive(Component)]
struct Initialized(bool);

#[derive(Event, Debug, Clone)]
struct EntereArea {
    entity: Option<Entity>,
}

fn initialized_cape_enemy(
    query: Query<(Entity, &GodotNodeHandle, &Initialized), With<CapeEnemyNode3D>>,
    signal_enter: GodotSignals<EntereArea>,
    mut godot: GodotAccess,
) {
    for (entity, handle, initialized) in query {
        if initialized.0 {
            return;
        }

        let Some(body) = get_custom_character_body_3d(handle, &mut godot) else {
            return;
        };

        let Some(hitbox) = get_combat_area_node(&body) else {
            return;
        };

        signal_enter.connect(
            hitbox.into(),
            Area3DSignals::BODY_ENTERED,
            Some(entity),
            |_args, _node_handle, ent| Some(EntereArea { entity: ent }),
        );
    }
}

fn on_area_enter(
    trigger: On<EntereArea>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player3DNode>>,
) {
    let Some(entity) = trigger.event().entity else {
        return;
    };

    let Ok(entity_player) = player_query.single() else {
        return;
    };

    commands.trigger(EnterCombatEvent {
        enemy: entity,
        player: entity_player,
    });
}

fn kill_enemy_3d(mut commands: Commands, query: Query<(Entity, &Health), With<CapeEnemyNode3D>>) {
    for (entity, health) in &query {
        if health.0 <= 0.0 {
            commands.trigger(DiedInCombat{entity});
            commands.entity(entity).despawn();
        }
    }
}

pub struct CapeEnemy3DPlugin;

impl Plugin for CapeEnemy3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            initialized_cape_enemy.run_if(in_state(GameState::Ingame3D)),
        )
        .add_systems(Update, kill_enemy_3d.run_if(in_state(GameState::InCombat)))
        .add_observer(on_area_enter);
    }
}
