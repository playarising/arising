use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResourceKind {
    Gold,
    Wood,
    Plank,
    RawLeather,
    Leather,
    BronzeOre,
    BronzeBar,
    WoodenSword,
    TrainingGreatsword,
    WoodenShield,
    LeatherArmor,
    LeatherLeggings,
    LeatherBoots,
    LeatherHelmet,
    BronzeRing,
    BronzeAmulet,
    TravelersCloak,
    HuntingQuiver,
}

impl ResourceKind {
    pub const fn seed_bytes(self) -> &'static [u8] {
        match self {
            ResourceKind::Gold => b"gold",
            ResourceKind::Wood => b"wood",
            ResourceKind::Plank => b"plank",
            ResourceKind::RawLeather => b"raw_leather",
            ResourceKind::Leather => b"leather",
            ResourceKind::BronzeOre => b"bronze_ore",
            ResourceKind::BronzeBar => b"bronze_bar",
            ResourceKind::WoodenSword => b"wooden_sword",
            ResourceKind::TrainingGreatsword => b"training_greatsword",
            ResourceKind::WoodenShield => b"wooden_shield",
            ResourceKind::LeatherArmor => b"leather_armor",
            ResourceKind::LeatherLeggings => b"leather_leggings",
            ResourceKind::LeatherBoots => b"leather_boots",
            ResourceKind::LeatherHelmet => b"leather_helmet",
            ResourceKind::BronzeRing => b"bronze_ring",
            ResourceKind::BronzeAmulet => b"bronze_amulet",
            ResourceKind::TravelersCloak => b"travelers_cloak",
            ResourceKind::HuntingQuiver => b"hunting_quiver",
        }
    }

    pub const fn decimals(self) -> u8 {
        0
    }

    pub const fn symbol(self) -> &'static str {
        match self {
            ResourceKind::Gold => "GOLD",
            ResourceKind::Wood => "WOOD",
            ResourceKind::Plank => "PLANK",
            ResourceKind::RawLeather => "RAW_LEATHER",
            ResourceKind::Leather => "LEATHER",
            ResourceKind::BronzeOre => "BRONZE_ORE",
            ResourceKind::BronzeBar => "BRONZE_BAR",
            ResourceKind::WoodenSword => "WOOD_SWORD",
            ResourceKind::TrainingGreatsword => "TRAIN_GSWORD",
            ResourceKind::WoodenShield => "WOOD_SHIELD",
            ResourceKind::LeatherArmor => "LEATHER_ARMOR",
            ResourceKind::LeatherLeggings => "LEATHER_LEGS",
            ResourceKind::LeatherBoots => "LEATHER_BOOTS",
            ResourceKind::LeatherHelmet => "LEATHER_HELM",
            ResourceKind::BronzeRing => "BRONZE_RING",
            ResourceKind::BronzeAmulet => "BRONZE_AMULET",
            ResourceKind::TravelersCloak => "TRAVEL_CLOAK",
            ResourceKind::HuntingQuiver => "HUNTING_QUIVER",
        }
    }
}
