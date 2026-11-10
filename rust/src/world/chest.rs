use bevy::prelude::*;
use godot::{
    classes::{AnimationPlayer, Area3D, Label3D, class_macros::private::virtuals::Xrvrs::Gd},
    global::godot_print,
};
use godot_bevy::prelude::*;

use crate::{
    characters::components::state::Initialized, godot_utils::nodes::get_custom_area_3d,
    state::GameState,
};

use godot::classes::Input;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Area3D, class_name = ChestNode)]
#[gdbevy(require(initialized:Initialized, as = bool, default = false))]
#[gdbevy(require(opened:Opened, as = bool, default = false))]
pub struct ChestInteractionNode;

#[derive(Component)]
struct Opened(pub bool);

#[derive(Resource, Default)]
pub struct ChestInteraction {
    pub chest_handle: Option<GodotNodeHandle>,
    pub chest_entity: Option<Entity>,
}

fn get_animation_player(area: &Gd<Area3D>) -> Option<Gd<AnimationPlayer>> {
    let animation_handle = area.get_node_or_null("RECLAIM CACHE | Asset Studio/AnimationPlayer")?;

    let Ok(animation) = animation_handle.try_cast::<AnimationPlayer>() else {
        return None;
    };

    Some(animation)
}

fn get_interaction_label(area: &Gd<Area3D>) -> Option<Gd<Label3D>> {
    let label_handle = area.get_node_or_null("Label3D")?;

    let Ok(label) = label_handle.try_cast::<Label3D>() else {
        return None;
    };

    Some(label)
}

#[derive(Event, Clone)]
pub struct EnterChestArea {
    handle: GodotNodeHandle,
    entity: Entity,
}

#[derive(Event, Clone)]
pub struct ExitChestArea {
    handle: GodotNodeHandle,
}

fn intialized_chest_interaction_node(
    query: Query<(Entity, &GodotNodeHandle, &mut Initialized), With<ChestInteractionNode>>,
    signal_enter: GodotSignals<EnterChestArea>,
    signal_exit: GodotSignals<ExitChestArea>,
    mut godot: GodotAccess,
) {
    for (entity, handle, mut initialized) in query {
        if initialized.0 {
            continue;
        }

        let Some(area) = get_custom_area_3d(handle, &mut godot) else {
            continue;
        };

        signal_enter.connect(
            area.clone().into(),
            Area3DSignals::BODY_ENTERED,
            None,
            move |_args, node_handle, _ent| {
                Some(EnterChestArea {
                    handle: node_handle,
                    entity,
                })
            },
        );

        signal_exit.connect(
            area.into(),
            Area3DSignals::BODY_EXITED,
            None,
            move |_args, node_handle, _ent| {
                Some(ExitChestArea {
                    handle: node_handle,
                })
            },
        );
        initialized.0 = true;
    }
}

fn on_enter_chest_area(
    trigger: On<EnterChestArea>,
    mut godot: GodotAccess,
    mut chest_resource: ResMut<ChestInteraction>,
) {
    godot_print!("Get Event for enter chest");
    let Some(area) = get_custom_area_3d(&trigger.handle, &mut godot) else {
        return;
    };

    let Some(mut label) = get_interaction_label(&area) else {
        return;
    };

    label.set_visible(true);

    chest_resource.chest_handle = Some(trigger.handle);
    chest_resource.chest_entity = Some(trigger.entity);
}

fn on_exit_chest_area(
    trigger: On<ExitChestArea>,
    mut chest_resource: ResMut<ChestInteraction>,
    mut godot: GodotAccess,
) {
    godot_print!("Get Event for exit chest");
    let Some(area) = get_custom_area_3d(&trigger.handle, &mut godot) else {
        return;
    };

    let Some(mut label) = get_interaction_label(&area) else {
        return;
    };

    label.set_visible(false);

    chest_resource.chest_handle = None;
    chest_resource.chest_entity = None;
}

fn process_chest_event(
    chest_resource: ResMut<ChestInteraction>,
    mut godot: GodotAccess,
    mut query: Query<&mut Opened, With<ChestInteractionNode>>,
) {
    let Some(handle) = chest_resource.chest_handle else {
        return;
    };

    let Some(entity) = chest_resource.chest_entity else {
        return;
    };

    let input = godot.singleton::<Input>();

    if input.is_action_just_pressed("interact") {
        let Some(area) = get_custom_area_3d(&handle, &mut godot) else {
            return;
        };

        let Some(mut animation) = get_animation_player(&area) else {
            return;
        };

        let Ok(mut open) = query.get_mut(entity) else {
            return;
        };

        animation.play_ex().name("CACHE_Open_Close").done();
        open.0 = true;
    }
}

pub struct ChestNodePlugin;

impl Plugin for ChestNodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChestInteraction>()
            .add_systems(
                Update,
                intialized_chest_interaction_node.run_if(in_state(GameState::Ingame3D)),
            )
            .add_systems(
                Update,
                process_chest_event.run_if(in_state(GameState::Ingame3D)),
            )
            .add_observer(on_enter_chest_area)
            .add_observer(on_exit_chest_area);
    }
}
