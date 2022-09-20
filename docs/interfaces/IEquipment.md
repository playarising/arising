# Solidity API

## IEquipment

Interface for the [Equipment](/docs/core/Equipment.md) contract.

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

### pause

```solidity
function pause() external
```

See [Equipment#pause](/docs/core/Equipment.md#pause)

### unpause

```solidity
function unpause() external
```

See [Equipment#unpause](/docs/core/Equipment.md#unpause)

### equip

```solidity
function equip(bytes _id, enum IEquipment.EquipmentSlot _slot, uint256 _item_id) external
```

See [Equipment#equip](/docs/core/Equipment.md#equip)

### unequip

```solidity
function unequip(bytes _id, enum IEquipment.EquipmentSlot _slot) external
```

See [Equipment#unequip](/docs/core/Equipment.md#unequip)

### getCharacterEquipment

```solidity
function getCharacterEquipment(bytes _id) external view returns (struct IEquipment.CharacterEquipment)
```

See [Equipment#getCharacterEquipment](/docs/core/Equipment.md#getCharacterEquipment)

### getCharacterTotalStatsModifiers

```solidity
function getCharacterTotalStatsModifiers(bytes _id) external view returns (struct IStats.BasicStats _modifiers)
```

See [Equipment#getCharacterTotalStatsModifiers](/docs/core/Equipment.md#getCharacterTotalStatsModifiers)

### getCharacterTotalAttributes

```solidity
function getCharacterTotalAttributes(bytes _id) external view returns (struct IItems.BaseAttributes _modifiers)
```

See [Equipment#getCharacterTotalAttributes](/docs/core/Equipment.md#getCharacterTotalAttributes)
