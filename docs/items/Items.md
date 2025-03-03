# Solidity API

## Items

This contract is an standard `ERC1155` implementation with internal mappings to store items additional
information for the characters usage.

Implementation of the [IItems](/docs/interfaces/IItems.md) interface.

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

### authorized

```solidity
mapping(address => bool) authorized
```

Map to store the list of authorized addresses to mint tokens.

### onlyAuthorized

```solidity
modifier onlyAuthorized()
```

Checks if the `msg.sender` is authorized to mint items.

### AddItem

```solidity
event AddItem(uint256 _item_id, string _name, string _description)
```

Event emmited when the [addItem](#addItem) function is called.

Requirements:

| Name          | Type    | Description           |
| ------------- | ------- | --------------------- |
| \_item_id     | uint256 | ID of the item added. |
| \_name        | string  | Name of the recipe.   |
| \_description | string  | Recipe description    |

### ItemUpdate

```solidity
event ItemUpdate(uint256 _item_id, string _name, string _description)
```

Event emmited when the [updateItem](#updateItem) function is called.

Requirements:

| Name          | Type    | Description           |
| ------------- | ------- | --------------------- |
| \_item_id     | uint256 | ID of the item added. |
| \_name        | string  | Name of the recipe.   |
| \_description | string  | Recipe description    |

### EnableItem

```solidity
event EnableItem(uint256 _item_id)
```

Event emmited when the [enableItem](#enableItem) function is called.

Requirements:

| Name      | Type    | Description             |
| --------- | ------- | ----------------------- |
| \_item_id | uint256 | ID of the item enabled. |

### DisableItem

```solidity
event DisableItem(uint256 _item_id)
```

Event emmited when the [disableItem](#disableItem) function is called.

Requirements:

| Name      | Type    | Description                  |
| --------- | ------- | ---------------------------- |
| \_item_id | uint256 | ID of the the item disabled. |

### initialize

```solidity
function initialize() public
```

Initializer.

### mint

```solidity
function mint(address _to, uint256 _item_id) public
```

Creates tokens to the address provided.

Requirements:

| Name      | Type    | Description                       |
| --------- | ------- | --------------------------------- |
| \_to      | address | Address that receives the tokens. |
| \_item_id | uint256 | ID of the item to be created.     |

### burn

```solidity
function burn(address _from, uint256 _item_id) public
```

Removes tokens from the address provided.

Requirements:

| Name      | Type    | Description                       |
| --------- | ------- | --------------------------------- |
| \_from    | address | Address that receives the tokens. |
| \_item_id | uint256 | ID of the item to be created.     |

### addAuthority

```solidity
function addAuthority(address _authority) public
```

Assigns a new address as an authority to mint items.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### removeAuthority

```solidity
function removeAuthority(address _authority) public
```

Removes an authority to mint items.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### addItem

```solidity
function addItem(string _name, string _description, uint256 _level_required, enum IItems.ItemType _item_type, struct IItems.StatsModifiers _stats_modifiers, struct IItems.Attributes _attributes) public
```

Adds the item data to relate with a specific token ID.

Requirements:

| Name              | Type                         | Description                                                                           |
| ----------------- | ---------------------------- | ------------------------------------------------------------------------------------- |
| \_name            | string                       | The item name.                                                                        |
| \_description     | string                       | The item description.                                                                 |
| \_level_required  | uint256                      | Minimum level for a character to use the item.                                        |
| \_item_type       | enum IItems.ItemType         | Type of the item defined by the enum [ItemType](/docs/interfaces/IItems.md#itemtype). |
| \_stats_modifiers | struct IItems.StatsModifiers | Item modifiers for the character stats.                                               |
| \_attributes      | struct IItems.Attributes     | Specific item attributes.                                                             |

### updateItem

```solidity
function updateItem(struct IItems.Item _item) public
```

Updates a previously added item.

Requirements:

| Name   | Type               | Description                   |
| ------ | ------------------ | ----------------------------- |
| \_item | struct IItems.Item | Full information of the item. |

### disableItem

```solidity
function disableItem(uint256 _item_id) public
```

Disables an item from beign equiped.

Requirements:

| Name      | Type    | Description     |
| --------- | ------- | --------------- |
| \_item_id | uint256 | ID of the item. |

### enableItem

```solidity
function enableItem(uint256 _item_id) public
```

Enables an item to be equiped.

Requirements:

| Name      | Type    | Description     |
| --------- | ------- | --------------- |
| \_item_id | uint256 | ID of the item. |

### getItem

```solidity
function getItem(uint256 _item_id) public view returns (struct IItems.Item _item)
```

Returns the full information of an item.

Requirements:

| Name      | Type    | Description     |
| --------- | ------- | --------------- |
| \_item_id | uint256 | ID of the item. |

| Name   | Type               | Description            |
| ------ | ------------------ | ---------------------- |
| \_item | struct IItems.Item | Full item information. |

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
