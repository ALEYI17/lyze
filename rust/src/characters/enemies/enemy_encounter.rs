use bevy::{platform::collections::HashMap, prelude::*};
use godot::{
    classes::{Area3D, Marker3D, class_macros::private::virtuals::Xrvrs::Gd},
    global::godot_print,
};
use godot_bevy::prelude::*;

use crate::{
    characters::{
        components::state::Initialized, enemies::cape_enemy_3d::CapeEnemy3D, player_3d::Player3DNode
    },
    events::combat::EnterCombatEvent,
    godot_utils::nodes::get_custom_area_3d,
};

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Area3D, class_name = EnemyEncounter)]
#[gdbevy(
    require(initialized: Initialized, as = bool, default = false),
)]
pub struct EnemyEncounterNode;

#[derive(Event, Debug, Clone)]
struct EntereEncounter {
    entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct EnemyEncounterRegistry {
    pub encounters: HashMap<Entity, Vec<Entity>>,
}

pub fn get_player_starting_position(area: &Gd<Area3D>) -> Option<Gd<Marker3D>> {
    let marker_handle = area.get_node_or_null("player_position")?;

    let Ok(marker) = marker_handle.try_cast::<Marker3D>() else {
        return None;
    };

    Some(marker)
}

fn get_childs_enemies(encounter: &Gd<Area3D>, index: &Res<NodeEntityIndex>) -> Vec<Entity> {
    let childs = encounter.get_children();

    let mut enemies: Vec<Entity> = Vec::new();

    for c in 0..childs.len() {
        let Some(child) = childs.get(c) else {
            continue;
        };

        let Some(enemy) = child.try_cast::<CapeEnemy3D>().ok() else {
            continue;
        };

        let enemy_handle = GodotNodeHandle::new(enemy);

        let id = enemy_handle.instance_id();

        let Some(final_entity) = index.get(id) else {
            continue;
        };

        enemies.push(final_entity);
    }

    enemies
}

fn initialize_enemy_encounter(
    query: Query<(Entity, &GodotNodeHandle, &mut Initialized), With<EnemyEncounterNode>>,
    mut godot: GodotAccess,
    signal_enter: GodotSignals<EntereEncounter>,
    index: Res<NodeEntityIndex>,
    mut registry: ResMut<EnemyEncounterRegistry>,
) {
    for (entity, handle, mut initialized) in query {
        if initialized.0 {
            continue;
        }

        let Some(area) = get_custom_area_3d(handle, &mut godot) else {
            continue;
        };

        let enemies = get_childs_enemies(&area, &index);

        registry.encounters.insert(entity, enemies);

        signal_enter.connect(
            area.into(),
            Area3DSignals::BODY_ENTERED,
            Some(entity),
            |_args, _node_handle, ent| Some(EntereEncounter { entity: ent }),
        );

        initialized.0 = true;
    }
}

fn on_enter_encounter(
    trigger: On<EntereEncounter>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player3DNode>>,
    registry: Res<EnemyEncounterRegistry>,
) {
    godot_print!("Enter on encounter");
    let Some(encounter) = trigger.event().entity else {
        return;
    };

    let Ok(entity_player) = player_query.single() else {
        return;
    };

    let Some(enemies) = registry.encounters.get(&encounter) else {
        return;
    };
    godot_print!("Enemies list: {:?}:", enemies);

    commands.trigger(EnterCombatEvent {
        enemy: enemies.to_vec(),
        player: entity_player,
        encounter,
    });
}

pub struct EnemyEncounterPlugin;

impl Plugin for EnemyEncounterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyEncounterRegistry>()
            .add_systems(Update, initialize_enemy_encounter)
            .add_observer(on_enter_encounter);
    }
}
