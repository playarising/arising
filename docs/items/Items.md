# Solidity API

## Items

This contract is an standard {ERC1155} implementation with internal mappings to store items additional
information for the characters usage.

_Implementation of the {IItems} interface._

### items

```solidity
mapping(uint256 => struct IItems.Item) items
```

Map to track the extra items data.

### \_items

```solidity
uint256[] _items
```

Array to track a full list of item IDs.

### constructor

```solidity
constructor() public
```

Constructor.

### mint

```solidity
function mint(address _to, uint256 _id) public
```

Creates tokens to the address provided.

Requirements:

| Name | Type    | Description                       |
| ---- | ------- | --------------------------------- |
| \_to | address | Address that receives the tokens. |
| \_id | uint256 | ID of the item to be created.     |

### addItem

```solidity
function addItem(uint256 _level_required, enum IItems.ItemType _item_type, struct IItems.StatsModifiers _stats_modifiers, struct IItems.Attributes _attributes) public
```

Adds the item data to relate with a specific token ID.

Requirements:

| Name              | Type                         | Description                                      |
| ----------------- | ---------------------------- | ------------------------------------------------ |
| \_level_required  | uint256                      | Minimum level for a character to use the item.   |
| \_item_type       | enum IItems.ItemType         | Type of the item defined by the enum {ItemType}. |
| \_stats_modifiers | struct IItems.StatsModifiers | Item modifiers for the character stats.          |
| \_attributes      | struct IItems.Attributes     | Specific item attributes.                        |

### disableItem

```solidity
function disableItem(uint256 _id) public
```

Disables an item from beign equiped.

Requirements:

| Name | Type    | Description     |
| ---- | ------- | --------------- |
| \_id | uint256 | ID of the item. |

### enableItem

```solidity
function enableItem(uint256 _id) public
```

Enables an item to be equiped.

Requirements:

| Name | Type    | Description     |
| ---- | ------- | --------------- |
| \_id | uint256 | ID of the item. |

### getItem

```solidity
function getItem(uint256 _id) public view returns (struct IItems.Item _item)
```

Returns the full information of an item.

Requirements:

| Name | Type    | Description     |
| ---- | ------- | --------------- |
| \_id | uint256 | ID of the item. |

| Name   | Type               | Description            |
| ------ | ------------------ | ---------------------- |
| \_item | struct IItems.Item | Full item information. |
