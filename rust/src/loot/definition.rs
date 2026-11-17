use crate::items::definition::ItemId;

#[derive(Debug, Clone, Copy)]
pub struct LootEntry {
    pub min_quantity: u32,
    pub max_quantity: u32,
    pub copies: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct LootReward {
    pub item: ItemId,
    pub ammount: u32,
}
