use anchor_lang::prelude::*;

use super::base::Attributes;
use super::resources::ResourceKind;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum EquipmentSlot {
    LeftHand,
    RightHand,
    Chest,
    Legs,
    Boots,
    Helmet,
    RingOne,
    RingTwo,
    AmuletOne,
    AmuletTwo,
    Back,
}

impl EquipmentSlot {
    pub const COUNT: usize = 11;

    pub const fn is_hand(self) -> bool {
        matches!(self, EquipmentSlot::LeftHand | EquipmentSlot::RightHand)
    }

    pub const fn is_ring(self) -> bool {
        matches!(self, EquipmentSlot::RingOne | EquipmentSlot::RingTwo)
    }

    pub const fn is_amulet(self) -> bool {
        matches!(self, EquipmentSlot::AmuletOne | EquipmentSlot::AmuletTwo)
    }

    pub const fn companion_hand(self) -> Option<EquipmentSlot> {
        match self {
            EquipmentSlot::LeftHand => Some(EquipmentSlot::RightHand),
            EquipmentSlot::RightHand => Some(EquipmentSlot::LeftHand),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EquipmentCategory {
    Weapon,
    Armor,
    Accessory,
    Back,
}

#[derive(Clone, Copy)]
pub struct EquipmentDefinition {
    pub resource: ResourceKind,
    pub category: EquipmentCategory,
    pub allowed_slots: &'static [EquipmentSlot],
    pub occupies_both_hands: bool,
    pub level_required: u8,
    pub attribute_bonus: Attributes,
}

impl EquipmentDefinition {
    pub const fn new(
        resource: ResourceKind,
        category: EquipmentCategory,
        allowed_slots: &'static [EquipmentSlot],
        occupies_both_hands: bool,
        level_required: u8,
        attribute_bonus: Attributes,
    ) -> Self {
        Self {
            resource,
            category,
            allowed_slots,
            occupies_both_hands,
            level_required,
            attribute_bonus,
        }
    }

    pub fn supports_slot(&self, slot: EquipmentSlot) -> bool {
        self.allowed_slots.iter().any(|allowed| *allowed == slot)
    }
}

const HAND_SLOTS: [EquipmentSlot; 2] = [EquipmentSlot::LeftHand, EquipmentSlot::RightHand];
const LEFT_HAND_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::LeftHand];
const CHEST_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Chest];
const LEGS_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Legs];
const BOOTS_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Boots];
const HELMET_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Helmet];
const RING_SLOTS: [EquipmentSlot; 2] = [EquipmentSlot::RingOne, EquipmentSlot::RingTwo];
const AMULET_SLOTS: [EquipmentSlot; 2] = [EquipmentSlot::AmuletOne, EquipmentSlot::AmuletTwo];
const BACK_SLOT: [EquipmentSlot; 1] = [EquipmentSlot::Back];

pub const EQUIPMENT_CODEX: [EquipmentDefinition; 11] = [
    EquipmentDefinition::new(
        ResourceKind::WoodenSword,
        EquipmentCategory::Weapon,
        &HAND_SLOTS,
        false,
        1,
        Attributes::new(2, 0, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::TrainingGreatsword,
        EquipmentCategory::Weapon,
        &HAND_SLOTS,
        true,
        3,
        Attributes::new(4, 0, 0, 0, 0, -1),
    ),
    EquipmentDefinition::new(
        ResourceKind::WoodenShield,
        EquipmentCategory::Weapon,
        &LEFT_HAND_SLOT,
        false,
        2,
        Attributes::new(0, 2, 0, 1, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherArmor,
        EquipmentCategory::Armor,
        &CHEST_SLOT,
        false,
        2,
        Attributes::new(0, 2, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherLeggings,
        EquipmentCategory::Armor,
        &LEGS_SLOT,
        false,
        2,
        Attributes::new(0, 1, 0, 0, 0, 1),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherBoots,
        EquipmentCategory::Armor,
        &BOOTS_SLOT,
        false,
        2,
        Attributes::new(1, 0, 0, 0, 0, 1),
    ),
    EquipmentDefinition::new(
        ResourceKind::LeatherHelmet,
        EquipmentCategory::Armor,
        &HELMET_SLOT,
        false,
        2,
        Attributes::new(0, 1, 0, 1, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::BronzeRing,
        EquipmentCategory::Accessory,
        &RING_SLOTS,
        false,
        1,
        Attributes::new(1, 0, 0, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::BronzeAmulet,
        EquipmentCategory::Accessory,
        &AMULET_SLOTS,
        false,
        2,
        Attributes::new(0, 0, 2, 0, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::TravelersCloak,
        EquipmentCategory::Back,
        &BACK_SLOT,
        false,
        1,
        Attributes::new(0, 0, 0, 1, 0, 0),
    ),
    EquipmentDefinition::new(
        ResourceKind::HuntingQuiver,
        EquipmentCategory::Back,
        &BACK_SLOT,
        false,
        2,
        Attributes::new(0, 0, 0, 0, 1, 0),
    ),
];

pub fn equipment_definition(resource: ResourceKind) -> Option<EquipmentDefinition> {
    equipment_index_for(resource).and_then(|idx| EQUIPMENT_CODEX.get(idx).copied())
}

/// Optimized O(1) equipment lookup by index
#[inline]
pub fn equipment_definition_by_index(index: u8) -> Option<EquipmentDefinition> {
    EQUIPMENT_CODEX.get(index as usize).copied()
}

pub fn equipment_definition_index(resource: ResourceKind) -> Option<u8> {
    equipment_index_for(resource).map(|idx| idx as u8)
}

const fn equipment_index_for(resource: ResourceKind) -> Option<usize> {
    match resource {
        ResourceKind::WoodenSword => Some(0),
        ResourceKind::TrainingGreatsword => Some(1),
        ResourceKind::WoodenShield => Some(2),
        ResourceKind::LeatherArmor => Some(3),
        ResourceKind::LeatherLeggings => Some(4),
        ResourceKind::LeatherBoots => Some(5),
        ResourceKind::LeatherHelmet => Some(6),
        ResourceKind::BronzeRing => Some(7),
        ResourceKind::BronzeAmulet => Some(8),
        ResourceKind::TravelersCloak => Some(9),
        ResourceKind::HuntingQuiver => Some(10),
        _ => None,
    }
}
