use bevy::prelude::*;
use bevy::time::Timer;
use godot::classes::{Area2D, CharacterBody2D};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::characters::components::stats::{Damage, Direction, Gravity, Health, Speed};
use crate::characters::enemies::ai::state::{EnemyState, PatrolInterval, PatrolTimer};
use crate::events::damage::DamageEvent;
use crate::state::GameState;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CharacterBody2D, class_name = CapeEnemy)]
#[gdbevy(
    require(speed: Speed, as = f32, default = 150.0),
    require(health: Health, as = f32, default = 50.0),
    require(damage: Damage, as = f32, default = 5.0),
    require(direction: Direction, as = f32, default = -1.0),
    require(enemy_gravity: Gravity, as = f32, default = 980.0),
    require(initialized:CapeEnemyInitialized, as = bool, default = false),
    require(patrol_interval: PatrolInterval, as = f32, default = 3.0),
)]
pub struct CapeEnemyNode;

#[derive(Component, Default)]
struct CapeEnemyInitialized(bool);

#[derive(Event, Debug, Clone)]
struct EnteredBody {
    entity: Entity,
    instance_id: InstanceId,
}

#[derive(Event, Debug, Clone)]
struct HurtboxRequest {
    entity: Entity,
    instance_id: InstanceId,
}

fn initialize_cape_enemy(
    signal_enter: GodotSignals<EnteredBody>,
    signal_hurt: GodotSignals<HurtboxRequest>,
    mut query: Query<
        (
            Entity,
            &GodotNodeHandle,
            &mut CapeEnemyInitialized,
            &PatrolInterval,
        ),
        With<CapeEnemyNode>,
    >,
    mut godot: GodotAccess,
    mut commands: Commands,
) {
    for (entity, handle, mut initialized, patrol_interval) in &mut query {
        if !initialized.0 {
            let Some(body) = godot.try_get::<CharacterBody2D>(*handle) else {
                godot_print!("Dont get body enemy");
                return;
            };

            let Some(hitbox_node) = body.get_node_or_null("hitbox") else {
                godot_print!("dont get hitbox");
                return;
            };

            let Ok(hitbox) = hitbox_node.try_cast::<Area2D>() else {
                godot_print!("cant cast hitbox");
                return;
            };

            let Some(hurtbox_node) = body.get_node_or_null("hurtbox") else {
                godot_print!("dont get hitbox");
                return;
            };

            let Ok(hurtbox) = hurtbox_node.try_cast::<Area2D>() else {
                godot_print!("cant cast hitbox");
                return;
            };

            // add signal to hitbox
            signal_enter.connect(
                hitbox.into(),
                Area2DSignals::AREA_ENTERED,
                None,
                move |args, _node_handle, _ent| {
                    let p_id = get_parent_instance_id(args)?;

                    Some(EnteredBody {
                        entity: entity,
                        instance_id: p_id,
                    })
                },
            );

            // add signal to hutbox
            signal_hurt.connect(
                hurtbox.into(),
                Area2DSignals::AREA_ENTERED,
                None,
                move |args, _node_handle, _ent| {
                    let p_id = get_parent_instance_id(args)?;

                    Some(HurtboxRequest {
                        entity,
                        instance_id: p_id,
                    })
                },
            );

            commands.entity(entity).insert((
                EnemyState::Patrol,
                PatrolTimer(Timer::from_seconds(
                    patrol_interval.0,
                    TimerMode::Repeating,
                )),
            ));

            initialized.0 = true;
        }
    }
}

fn get_parent_instance_id(args: &[Variant]) -> Option<InstanceId> {
    let area = args.get(0)?;
    let area = area.try_to::<Gd<Area2D>>().ok()?;
    let parent = area.get_parent()?;

    let body_handle = GodotNodeHandle::new(parent);

    Some(body_handle.instance_id())
}

fn on_timeout(
    time: Res<Time>,
    query: Query<(&GodotNodeHandle, &mut Direction, &mut PatrolTimer, &EnemyState), With<CapeEnemyNode>>,
    mut godot: GodotAccess,
) {

    for (handle,mut direction,mut  timer, state) in query{
        if !state.is_patrolling(){
            continue;
        }

        timer.0.tick(time.delta());
        if !timer.0.just_finished(){
            continue;
        }
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            continue;
        };
        let scale = body.get_scale();
        body.set_scale(Vector2::new(-scale.x, scale.y));
        direction.0 *= -1.0;
    }
    
}

fn on_enter_body(
    trigger: On<EnteredBody>,
    index: Res<NodeEntityIndex>,
    mut command: Commands,
    collision: Collisions,
) {
    let Some(entity_area) = index.get(trigger.event().instance_id) else {
        return;
    };
    godot_print!(
        "Area instance id = {:?}, entity = {:?}",
        trigger.event().instance_id,
        entity_area
    );

    for other in collision.colliding_with(trigger.event().entity) {
        godot_print!("other:{:?}", other);
    }

    command.trigger(DamageEvent {
        source: trigger.event().entity,
        target: entity_area,
    });
}

fn on_hurt(
    trigger: On<HurtboxRequest>,
    index: Res<NodeEntityIndex>,
    mut command: Commands,
    collision: Collisions,
) {
    let Some(entity_area) = index.get(trigger.event().instance_id) else {
        return;
    };
    godot_print!(
        "Area instance id = {:?}, entity = {:?}",
        trigger.event().instance_id,
        entity_area
    );

    for other in collision.colliding_with(trigger.event().entity) {
        godot_print!("other:{:?}", other);
    }

    command.trigger(DamageEvent {
        target: trigger.event().entity,
        source: entity_area,
    });
}

fn find_cape_enemy(
    mut query: Query<(&GodotNodeHandle, &Speed, &Direction, &Gravity), With<CapeEnemyNode>>,
    mut godot: GodotAccess,
) {
    for (handle, speed, direction, gravity) in &mut query {
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            godot_print!("dont find enemy");
            return;
        };

        let mut dir = 0.0;

        let mut pos = body.get_velocity();

        if !body.is_on_floor(){
            pos.y = gravity.0;
        }

        dir += direction.0;
        pos.x = dir * speed.0;

        body.set_velocity(pos);

        body.move_and_slide();
    }
}

fn kill_enemy(mut commands: Commands, query: Query<(Entity, &Health), With<CapeEnemyNode>>) {
    for (entity, health) in &query {
        if health.0 < 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub struct CapeEnemyPlugin;

impl Plugin for CapeEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, find_cape_enemy.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                initialize_cape_enemy.run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, kill_enemy.run_if(in_state(GameState::InGame)))
            .add_plugins(GodotSignalsPlugin::<EnteredBody>::default())
            .add_plugins(GodotSignalsPlugin::<HurtboxRequest>::default())
            .add_systems(Update, on_timeout.run_if(in_state(GameState::InGame)))
            .add_observer(on_enter_body)
            .add_observer(on_hurt);
    }
}
