use bevy::prelude::*;

use crate::{
    abilities::definition::ConvictionType,
    characters::{
        components::convictions::Convictions, inventories::convictions::ConvictionPoints,
        player_3d::Player3DNode,
    },
};

const PRIMARY_UPGRADE: f32 = 3.0;
const SYNTHESIS_UPGRADE: f32 = 1.0;
const UPGRADE_COST: u32 = 1;

#[derive(Event)]
pub struct UpgradeConvictionEvent {
    pub conviction: ConvictionType,
}

fn on_upgrade_conviction(
    trigger: On<UpgradeConvictionEvent>,
    mut query: Query<&mut Convictions, With<Player3DNode>>,
    mut conviction_points: ResMut<ConvictionPoints>,
) {
    if conviction_points.avaible_points == 0 {
        return;
    }

    let Ok(mut convictions) = query.single_mut() else {
        return;
    };

    let selected = trigger.conviction;

    let stats = convictions.get_mut(selected);

    stats.attack += PRIMARY_UPGRADE;
    stats.defense += PRIMARY_UPGRADE;

    for conviction in selected.synthesis() {
        let stats = convictions.get_mut(conviction);

        stats.attack += SYNTHESIS_UPGRADE;
        stats.defense += SYNTHESIS_UPGRADE;
    }

    conviction_points.avaible_points -= UPGRADE_COST;
}

pub struct UpgradeConvictionPlugin;

impl Plugin for UpgradeConvictionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_upgrade_conviction);
    }
}
