use bevy::prelude::*;

use crate::{
    abilities::definition::ConvictionType,
    characters::{
        components::convictions::Convictions, inventories::convictions::ConvictionPoints,
        player_3d::Player3DNode,
    },
};

#[derive(Event)]
pub struct UpgradeConvictionEvent {
    pub conviction: ConvictionType,
}

fn on_upgrade_conviction(
    trigger: On<UpgradeConvictionEvent>,
    mut query: Query<&mut Convictions, With<Player3DNode>>,
    mut conviction_points: ResMut<ConvictionPoints>,
) {
    let Ok(mut convictions) = query.single_mut() else {
        return;
    };

    match &trigger.conviction {
        ConvictionType::Ananke => {
            convictions.ananke.attack += 1.0;
            convictions.ananke.defense += 1.0;
        }
        ConvictionType::Sekhem => {
            convictions.sekhem.attack += 1.0;
            convictions.sekhem.defense += 1.0;
        }
        ConvictionType::Aeternum => {
            convictions.aeternum.attack += 1.0;
            convictions.aeternum.defense += 1.0;
        }
        ConvictionType::Ataraxia => {
            convictions.ataraxia.attack += 1.0;
            convictions.ataraxia.defense += 1.0;
        }
        ConvictionType::Alligatio => {
            convictions.alligatio.attack += 1.0;
            convictions.alligatio.defense += 1.0;
        }
    }

    conviction_points.avaible_points -= 1;
}

pub struct UpgradeConvictionPlugin;

impl Plugin for UpgradeConvictionPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_upgrade_conviction);
    }
}
