use crate::abilities::definition::ConvictionType;

#[derive(Debug, Clone)]
pub enum ItemKind {
    Prosthetic(ProstheticDefinition),
    Consumable(ConsumableEffect),
    Materials,
}

#[derive(Debug, Clone)]
pub struct ProstheticDefinition {
    pub slot: ProstheticSlot,
    pub bonuses: Vec<ConvictionsBonus>,
    pub upgrades: Vec<ProstheticUpgrade>,
    pub max_level: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum ProstheticSlot {
    LeftArm,
    RightLeg,
}

#[derive(Debug, Clone, Copy)]
pub struct ConvictionsBonus {
    pub conviction: ConvictionType,
    pub attack: f32,
    pub defense: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct UpgradeRequirements {
    pub material: ItemId,
    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct ProstheticUpgrade {
    pub requirements: Vec<UpgradeRequirements>,
    pub bonuses: Vec<ConvictionsBonus>,
}

#[derive(Debug, Clone, Copy)]
pub enum ConsumableEffect {
    Heal(f32),
    UpgrdeConviction { conviction: ConvictionType },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ItemId {
    RegenerativeSerum,
    MyofiberArm,
    NeurostrideLeg,
    Biomass,
    SekhemSerum,
}

#[derive(Debug, Clone)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: &'static str,
    pub description: &'static str,
    pub kind: ItemKind,
}
