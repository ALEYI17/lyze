
use bevy::prelude::*;
use godot::classes::{Area2D, CharacterBody2D, Timer};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::characters::players::{Damage, Health, PlayerNode, Speed};
use crate::state::GameState;

#[derive(Component, Default)]
pub struct Direction(f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CapeEnemyNode;

#[derive(Bundle, GodotNode, Default)]
#[godot_node(base(CharacterBody2D), class_name(CapeEnemy))]
pub struct CapeEnemyGodotNode {
    pub enemy: CapeEnemyNode,

    #[export_fields(value(export_type(f32), default(150.0)))]
    pub speed: Speed,
    
    // #[export_fields(value(export_type(f32), default(980.0)))]
    // pub gravity: Gravity,
    #[export_fields(value(export_type(f32), default(50.0)))]
    pub health: Health,

    #[export_fields(value(export_type(f32), default(5.0)))]
    pub damage: Damage,
    
    #[export_fields(value(export_type(f32), default(-1.0)))]
    pub direction: Direction,
}


#[derive(Resource, Default)]
struct CapeEnemyState {
    pub initialized: bool,
}

#[derive(Event, Debug, Clone)]
struct ChangeDirectionRequested{
    entity: Entity
}

#[derive(Event, Debug, Clone)]
struct EnteredBody {
    node: GodotNodeHandle,
    entity: Entity,
}

#[derive(Event, Debug, Clone)]
struct HurtboxRequest{
    entity: Entity
}

fn connect_timeout_signal(
    signal_direction: GodotSignals<ChangeDirectionRequested>,
    signal_enter: GodotSignals<EnteredBody>,
    signal_hurt: GodotSignals<HurtboxRequest>,
    mut query: Query<(Entity,&GodotNodeHandle), With<CapeEnemyNode>>,
    mut godot: GodotAccess,
    mut state: ResMut<CapeEnemyState>,
) {
    for (entity,handle) in &mut query {
        let Some(body) = godot.try_get::<CharacterBody2D>(*handle) else {
            godot_print!("Dont get body enemy");
            return;
        };

        let Some(timer_node) = body.get_node_or_null("Timer") else {
            godot_print!("Dont get timer");
            return;
        };

        let Ok(timer) = timer_node.try_cast::<Timer>() else {
            godot_print!("Cant cast");
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
            move |_args, node_handle, _ent| {
                Some(EnteredBody {
                    entity: entity,
                    node: node_handle,
                })
            },
        );

        // add signal to hutbox
        signal_hurt.connect(
            hurtbox.into(),
            Area2DSignals::AREA_ENTERED,
             None,
            move |_args, _node_handle, _ent| {
                Some(HurtboxRequest { entity })
            },
        );

        signal_direction.connect(
            timer.into(),
            TimerSignals::TIMEOUT,
            None,
            move |_args, _node_handle, _ent| {
                Some(ChangeDirectionRequested { entity: entity })
            },
        );

        state.initialized = true;
    }
}

fn on_timeout(
    tigger: On<ChangeDirectionRequested>,
    mut query: Query<(&GodotNodeHandle, &mut Direction), With<CapeEnemyNode>>,
    mut godot: GodotAccess,
) {

    let Ok((handle, mut direction)) = query.get_mut(tigger.entity) else{
        return;
    }; 

    let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else{
        return;
    };

    let scale = body.get_scale();
    body.set_scale(Vector2::new(-scale.x, scale.y));
    direction.0 *= -1.0;
}

fn on_enter_body(
    tigger: On<EnteredBody>,
    mut queryp: Query<(&mut Health, &GodotNodeHandle), With<PlayerNode>>,
    querye: Query<&Damage, With<CapeEnemyNode>>
) {
    let Ok(damage) = querye.get(tigger.entity) else {
        return;
    };

    if let Ok((mut player_health, handle)) = queryp.single_mut() {
        godot_print!("Health before damage: {}", player_health.0);
        player_health.0 -= damage.0;
        godot_print!("Health after damage: {}", player_health.0);

        godot_print!("Handle: {:?}", handle);
    }
    godot_print!(
        "enter body kill player: {:?}, {:?}",
        tigger.entity,
        tigger.node
    );
}

fn on_hurt(
    trigger: On<HurtboxRequest>,
    mut querye: Query<(&mut Health, &GodotNodeHandle), With<CapeEnemyNode>>,
    queryp: Query<&Damage, With<PlayerNode>>
) {
    let Ok(damage) = queryp.single() else{
        return;
    };
    if let Ok((mut enemy_health, handle)) = querye.get_mut(trigger.entity) {
        godot_print!("Enemy Health before: {}", enemy_health.0);
        enemy_health.0 -= damage.0;
        godot_print!("Enemy Health after: {}", enemy_health.0);
        godot_print!("Enemy Handle: {:?}", handle);
    }
    godot_print!("Caped enemy receive damage");
}

fn is_not_initialized(state: Res<CapeEnemyState>) -> bool {
    !state.initialized
}

fn find_cape_enemy(
    mut query: Query<(&GodotNodeHandle, &Speed, &Direction), With<CapeEnemyNode>>,
    mut godot: GodotAccess,
) {
    for (handle, speed,  direction) in &mut query {
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            godot_print!("dont find enemy");
            return;
        };

        let mut dir = 0.0;

        let mut pos = body.get_velocity();

        //let direction = DIRECTION.lock().unwrap();

        dir += direction.0;
        pos.x = dir * speed.0;

        body.set_velocity(pos);

        body.move_and_slide();
    }
}

pub struct CapeEnemyPlugin;

impl Plugin for CapeEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CapeEnemyState>()
            .add_systems(Update, find_cape_enemy.run_if(in_state(GameState::InGame)))
            .add_systems(Update, connect_timeout_signal.run_if(is_not_initialized))
            .add_plugins(GodotSignalsPlugin::<ChangeDirectionRequested>::default())
            .add_plugins(GodotSignalsPlugin::<EnteredBody>::default())
            .add_plugins(GodotSignalsPlugin::<HurtboxRequest>::default())
            .add_observer(on_timeout)
            .add_observer(on_enter_body)
            .add_observer(on_hurt);
    }
}
