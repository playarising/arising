# Solidity API

## IEquipment

Interface for the {Equipment} contract.

### ItemEquiped

```solidity
struct ItemEquiped {
  uint256 id;
  bool equiped;
}

```

### EquipmentSlot

```solidity
enum EquipmentSlot {
  HELMET,
  SHOULDER_GUARDS,
  ARM_GUARDS,
  HANDS,
  RING,
  NECKLACE,
  CHEST,
  LEGS,
  BELT,
  FEET,
  CAPE,
  LEFT_HAND,
  RIGHT_HAND
}

```

### CharacterEquipment

```solidity
struct CharacterEquipment {
  struct IEquipment.ItemEquiped helmet;
  struct IEquipment.ItemEquiped shoulder_guards;
  struct IEquipment.ItemEquiped arm_guards;
  struct IEquipment.ItemEquiped hands;
  struct IEquipment.ItemEquiped rings;
  struct IEquipment.ItemEquiped necklace;
  struct IEquipment.ItemEquiped chest;
  struct IEquipment.ItemEquiped legs;
  struct IEquipment.ItemEquiped belt;
  struct IEquipment.ItemEquiped feet;
  struct IEquipment.ItemEquiped cape;
  struct IEquipment.ItemEquiped left_hand;
  struct IEquipment.ItemEquiped right_hand;
}
```
