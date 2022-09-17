# Solidity API

## Equipment

This contract enables characters to equip/unequip `ERC1155` tokens stored through the [Items](/docs/items/Items.md) implementation.

Implementation of the [IEquipment](/docs/interfaces/IEquipment.md) interface.

### civilizations

```solidity
address civilizations
```

_Address of the [Civilizations](/docs/core/Civilizations.md) instance. \*_

### experience

```solidity
address experience
```

_Address of the [Experience](/docs/core/Experience.md) instance. \*_

### items

```solidity
address items
```

_Address of the [Items](/docs/items/Items.md) instance. \*_

### character_equipments

```solidity
mapping(bytes => mapping(enum IEquipment.EquipmentSlot => struct IEquipment.ItemEquiped)) character_equipments
```

_Map to track the equipment of characters. \*_

### slots_types

```solidity
mapping(enum IEquipment.EquipmentSlot => mapping(enum IItems.ItemType => bool)) slots_types
```

_Map to track the slots that can be used by an item type. \*_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

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
function equip(bytes id, enum IEquipment.EquipmentSlot item_slot, uint256 item_id) public
```

_Assigns an item to an item slot. If it is already used, it replaces it._

| Name      | Type                          | Description                        |
| --------- | ----------------------------- | ---------------------------------- |
| id        | bytes                         | Composed ID of the character.      |
| item_slot | enum IEquipment.EquipmentSlot | Slot from the equipment to remove. |
| item_id   | uint256                       | ID of the item to assign.          |

### unequip

```solidity
function unequip(bytes id, enum IEquipment.EquipmentSlot item_slot) public
```

_Removes an item from the character equipment and transfers de ERC1155 token._

| Name      | Type                          | Description                        |
| --------- | ----------------------------- | ---------------------------------- |
| id        | bytes                         | Composed ID of the character.      |
| item_slot | enum IEquipment.EquipmentSlot | Slot from the equipment to remove. |

### getCharacterEquipment

```solidity
function getCharacterEquipment(bytes id) public view returns (struct IEquipment.CharacterEquipment)
```

_Returns the full requipment of a character.
@param id Composed ID of the token._

### getCharacterTotalStatsModifiers

```solidity
function getCharacterTotalStatsModifiers(bytes id) public view returns (struct IStats.BasicStats, struct IStats.BasicStats)
```

_Returns the total modifiers from the equipment.
@param id Composed ID of the token._

### getCharacterTotalAttributes

```solidity
function getCharacterTotalAttributes(bytes id) public view returns (struct IItems.BaseAttributes, struct IItems.BaseAttributes)
```

_Returns the total attributes from the equipment.
@param id Composed ID of the token._
