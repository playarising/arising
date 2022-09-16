# Solidity API

## Equipment

_`Equipment` is the contract to equip gear for Arising characters._

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` instance. \*_

### experience

```solidity
address experience
```

_Address of the `Experience` instance. \*_

### items

```solidity
address items
```

_Address of the `Items` instance. \*_

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

_Constructor._

| Name            | Type    | Description                                  |
| --------------- | ------- | -------------------------------------------- |
| \_civilizations | address | The address of the `Civilizations` instance. |
| \_experience    | address | The address of the `Experience` instance.    |
| \_items         | address | The address of the `Items` instance.         |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_

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
