# Solidity API

## Equipment

This contract enables characters to equip/unequip `ERC1155` tokens stored through the [Items](/docs/items/Items.md) implementation.

Implementation of the [IEquipment](/docs/interfaces/IEquipment.md) interface.

### civilizations

```solidity
address civilizations
```

Address of the [Civilizations](/docs/core/Civilizations.md) instance.

### experience

```solidity
address experience
```

Address of the [Experience](/docs/core/Experience.md) instance.

### items

```solidity
address items
```

Address of the [Items](/docs/items/Items.md) instance.

### character_equipments

```solidity
mapping(bytes => mapping(enum IEquipment.EquipmentSlot => struct IEquipment.ItemEquiped)) character_equipments
```

_Map to track the equipment of characters._

### slots_types

```solidity
mapping(enum IEquipment.EquipmentSlot => mapping(enum IItems.ItemType => bool)) slots_types
```

_Map to track the equipment slots and its attachable items._

### onlyAllowed

```solidity
modifier onlyAllowed(bytes _id)
```

Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
has allowance to access a composed ID.

Requirements:

| Name | Type  | Description               |
| ---- | ----- | ------------------------- |
| \_id | bytes | Composed ID of the token. |

### constructor

```solidity
constructor(address _civilizations, address _experience, address _items) public
```

Constructor.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |
| \_items         | address | The address of the [Items](/docs/items/Items.md) instance.                |

### pause

```solidity
function pause() public
```

Pauses the contract

### unpause

```solidity
function unpause() public
```

Resumes the contract

### equip

```solidity
function equip(bytes _id, enum IEquipment.EquipmentSlot _slot, uint256 _item_id) public
```

Assigns an item to a character equipment slot. If the slot already has an equiped item
it is replaced by the item being equiped.

Requirements:

| Name      | Type                          | Description                   |
| --------- | ----------------------------- | ----------------------------- |
| \_id      | bytes                         | Composed ID of the character. |
| \_slot    | enum IEquipment.EquipmentSlot | Slot to equip the item.       |
| \_item_id | uint256                       | ID of the item to equip.      |

### unequip

```solidity
function unequip(bytes _id, enum IEquipment.EquipmentSlot _slot) public
```

Removes an item from a character equipment slot.

Requirements:

| Name   | Type                          | Description                   |
| ------ | ----------------------------- | ----------------------------- |
| \_id   | bytes                         | Composed ID of the character. |
| \_slot | enum IEquipment.EquipmentSlot | Slot to equip the item.       |

### getCharacterEquipment

```solidity
function getCharacterEquipment(bytes _id) public view returns (struct IEquipment.CharacterEquipment)
```

External function to return the character slots and items attached.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### getCharacterTotalStatsModifiers

```solidity
function getCharacterTotalStatsModifiers(bytes _id) public view returns (struct IStats.BasicStats _additions, struct IStats.BasicStats _reductions)
```

External function to return the character state modifiers with additions
and reducers counting the full equipment set.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name         | Type                     | Description                     |
| ------------ | ------------------------ | ------------------------------- |
| \_additions  | struct IStats.BasicStats | The amount of points increased. |
| \_reductions | struct IStats.BasicStats | The amount of points reduced.   |

### getCharacterTotalAttributes

```solidity
function getCharacterTotalAttributes(bytes _id) public view returns (struct IItems.BaseAttributes _additions, struct IItems.BaseAttributes _reductions)
```

External function to return the character total attributes counting additions
and reducers counting the full equipment set.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name         | Type                         | Description                     |
| ------------ | ---------------------------- | ------------------------------- |
| \_additions  | struct IItems.BaseAttributes | The amount of points increased. |
| \_reductions | struct IItems.BaseAttributes | The amount of points reduced.   |
