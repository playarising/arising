# Solidity API

## Craft

This contract is used to store and craft recipes through the ecosystem. This is the only contract able to mint
items through the [Items](/docs/items/Items.md) `ERC1155` implementation.

Implementation of the [ICraft](/docs/interfaces/ICraft.md) interface.

### recipes

```solidity
mapping(uint256 => struct ICraft.Recipe) recipes
```

Map to track available recipes for craft.

### \_recipes

```solidity
uint256[] _recipes
```

Array to track all the recipes IDs.

### upgrades

```solidity
mapping(uint256 => struct ICraft.Upgrade) upgrades
```

Map to track available upgrades to craft.

### \_upgrades

```solidity
uint256[] _upgrades
```

Array to track all the upgrades IDs.

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

### stats

```solidity
address stats
```

Address of the [Stats](/docs/core/Stats.md) instance.

### items

```solidity
address items
```

Address of the [Items](/docs/items/Items.md) instance.

### craft_slots

```solidity
mapping(bytes => struct ICraft.Slot) craft_slots
```

Map to track craft slots and cooldowns for each character.

### onlyAllowed

```solidity
modifier onlyAllowed(bytes _id)
```

Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
has allowance to access a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### AddRecipe

```solidity
event AddRecipe(uint256 _recipe_id, string _name, string _description)
```

Event emmited when the [addRecipe](#addRecipe) function is called.

Requirements:

| Name          | Type    | Description             |
| ------------- | ------- | ----------------------- |
| \_recipe_id   | uint256 | ID of the recipe added. |
| \_name        | string  | Name of the recipe.     |
| \_description | string  | Recipe description      |

### RecipeUpdate

```solidity
event RecipeUpdate(uint256 _recipe_id, string _name, string _description)
```

Event emmited when the [updateRecipe](#updateRecipe) function is called.

Requirements:

| Name          | Type    | Description             |
| ------------- | ------- | ----------------------- |
| \_recipe_id   | uint256 | ID of the recipe added. |
| \_name        | string  | Name of the recipe.     |
| \_description | string  | Recipe description      |

### EnableRecipe

```solidity
event EnableRecipe(uint256 _recipe_id)
```

Event emmited when the [enableRecipe](#enableRecipe) function is called.

Requirements:

| Name        | Type    | Description               |
| ----------- | ------- | ------------------------- |
| \_recipe_id | uint256 | ID of the recipe enabled. |

### DisableRecipe

```solidity
event DisableRecipe(uint256 _recipe_id)
```

Event emmited when the [disableRecipe](#disableRecipe) function is called.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_recipe_id | uint256 | ID of the recipe disabled. |

### AddUpgrade

```solidity
event AddUpgrade(uint256 _upgrade_id, string _name, string _description)
```

Event emmited when the [addUpgrade](#addUpgrade) function is called.

Requirements:

| Name          | Type    | Description                  |
| ------------- | ------- | ---------------------------- |
| \_upgrade_id  | uint256 | ID of the the upgrade added. |
| \_name        | string  | Name of the recipe.          |
| \_description | string  | Recipe description           |

### UpgradeUpdate

```solidity
event UpgradeUpdate(uint256 _upgrade_id, string _name, string _description)
```

Event emmited when the [updateUpgrade](#updateUpgrade) function is called.

Requirements:

| Name          | Type    | Description                  |
| ------------- | ------- | ---------------------------- |
| \_upgrade_id  | uint256 | ID of the the upgrade added. |
| \_name        | string  | Name of the recipe.          |
| \_description | string  | Recipe description           |

### EnableUpgrade

```solidity
event EnableUpgrade(uint256 _upgrade_id)
```

Event emmited when the [enableUpgrade](#enableUpgrade) function is called.

Requirements:

| Name         | Type    | Description                 |
| ------------ | ------- | --------------------------- |
| \_upgrade_id | uint256 | ID of the the recipe added. |

### DisableUpgrade

```solidity
event DisableUpgrade(uint256 _upgrade_id)
```

Event emmited when the [disableUpgrade](#disableUpgrade) function is called.

Requirements:

| Name         | Type    | Description                 |
| ------------ | ------- | --------------------------- |
| \_upgrade_id | uint256 | ID of the the recipe added. |

### initialize

```solidity
function initialize(address _civilizations, address _experience, address _stats, address _items) public
```

Initialize.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |
| \_stats         | address | The address of the [Stats](/docs/core/Stats.md) instance.                 |
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

### disableRecipe

```solidity
function disableRecipe(uint256 _recipe_id) public
```

Disables a recipe from beign crafted.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

### enableRecipe

```solidity
function enableRecipe(uint256 _recipe_id) public
```

Enables a recipe to be crafted.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

### disableUpgrade

```solidity
function disableUpgrade(uint256 _upgrade_id) public
```

Disables an upgrade from beign crafted.

Requirements:

| Name         | Type    | Description        |
| ------------ | ------- | ------------------ |
| \_upgrade_id | uint256 | ID of the upgrade. |

### enableUpgrade

```solidity
function enableUpgrade(uint256 _upgrade_id) public
```

Enables an upgrade to be crafted.

Requirements:

| Name         | Type    | Description        |
| ------------ | ------- | ------------------ |
| \_upgrade_id | uint256 | ID of the upgrade. |

### addRecipe

```solidity
function addRecipe(string _name, string _description, address[] _materials, uint256[] _amounts, struct IStats.BasicStats _stats, uint256 _cooldown, uint256 _level_required, uint256 _reward, uint256 _experience_reward) public
```

Adds a new recipe to craft.

Requirements:

| Name                | Type                     | Description                                                                             |
| ------------------- | ------------------------ | --------------------------------------------------------------------------------------- |
| \_name              | string                   | Name of the recipe.                                                                     |
| \_description       | string                   | Description of the recipe.                                                              |
| \_materials         | address[]                | Array of material [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances address. |
| \_amounts           | uint256[]                | Array of amounts for each material.                                                     |
| \_stats             | struct IStats.BasicStats | Stats to consume from the pool for craft.                                               |
| \_cooldown          | uint256                  | Number of seconds for the recipe cooldown.                                              |
| \_level_required    | uint256                  | Minimum level required to craft the recipe.                                             |
| \_reward            | uint256                  | ID of the token to reward for the [Items](/docs/items/Items.md) instance.               |
| \_experience_reward | uint256                  | Amount of experience rewarded for the recipe.                                           |

### updateRecipe

```solidity
function updateRecipe(struct ICraft.Recipe _recipe) public
```

Updates a previously added craft recipe.

Requirements:

| Name     | Type                 | Description                     |
| -------- | -------------------- | ------------------------------- |
| \_recipe | struct ICraft.Recipe | Full information of the recipe. |

### addUpgrade

```solidity
function addUpgrade(string _name, string _description, address[] _materials, uint256[] _amounts, struct IStats.BasicStats _stats, struct IStats.BasicStats _sacrifice, uint256 _level_required, uint256 _upgraded_item, uint256 _reward) public
```

Adds a new recipe to craft.

Requirements:

| Name             | Type                     | Description                                                                                  |
| ---------------- | ------------------------ | -------------------------------------------------------------------------------------------- |
| \_name           | string                   | Name of the upgrade.                                                                         |
| \_description    | string                   | Description of the upgrade.                                                                  |
| \_materials      | address[]                | Array of material [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances address.      |
| \_amounts        | uint256[]                | Array of amounts for each material.                                                          |
| \_stats          | struct IStats.BasicStats | Stats to consume from the pool for upgrade.                                                  |
| \_sacrifice      | struct IStats.BasicStats | Stats to sacrficed from the base stats for upgrade.                                          |
| \_level_required | uint256                  | Minimum level required to craft the recipe.                                                  |
| \_upgraded_item  | uint256                  | ID of the token item that is being upgraded from the [Items](/docs/items/Items.md) instance. |
| \_reward         | uint256                  | ID of the token to reward for the [Items](/docs/items/Items.md) instance.                    |

### updateUpgrade

```solidity
function updateUpgrade(struct ICraft.Upgrade _upgrade) public
```

Updates a previously added upgrade recipe.

Requirements:

| Name      | Type                  | Description                     |
| --------- | --------------------- | ------------------------------- |
| \_upgrade | struct ICraft.Upgrade | Full information of the recipe. |

### craft

```solidity
function craft(bytes _id, uint256 _recipe_id) public
```

Initializes a recipe to be crafted.

Requirements:

| Name        | Type    | Description                   |
| ----------- | ------- | ----------------------------- |
| \_id        | bytes   | Composed ID of the character. |
| \_recipe_id | uint256 | ID of the recipe.             |

### claim

```solidity
function claim(bytes _id) public
```

Claims a recipe already crafted.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### upgrade

```solidity
function upgrade(bytes _id, uint256 _upgrade_id) public
```

Upgrades an item.

Requirements:

| Name         | Type    | Description                   |
| ------------ | ------- | ----------------------------- |
| \_id         | bytes   | Composed ID of the character. |
| \_upgrade_id | uint256 | ID of the upgrade to perform. |

### getRecipe

```solidity
function getRecipe(uint256 _recipe_id) public view returns (struct ICraft.Recipe _recipe)
```

Returns the full information of a recipe.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

| Name     | Type                 | Description                    |
| -------- | -------------------- | ------------------------------ |
| \_recipe | struct ICraft.Recipe | Full information of the recipe |

### getUpgrade

```solidity
function getUpgrade(uint256 _upgrade_id) public view returns (struct ICraft.Upgrade _upgrade)
```

Returns the full information of an upgrade.

Requirements:

| Name         | Type    | Description        |
| ------------ | ------- | ------------------ |
| \_upgrade_id | uint256 | ID of the upgrade. |

| Name      | Type                  | Description                     |
| --------- | --------------------- | ------------------------------- |
| \_upgrade | struct ICraft.Upgrade | Full information of the upgrade |

### getCharacterCrafSlot

```solidity
function getCharacterCrafSlot(bytes _id) public view returns (struct ICraft.Slot _slot)
```

Returns character craft slot information.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name   | Type               | Description                                  |
| ------ | ------------------ | -------------------------------------------- |
| \_slot | struct ICraft.Slot | Full information of character crafting slot. |

### \_isSlotAvailable

```solidity
function _isSlotAvailable(bytes _id) internal view returns (bool _available)
```

Internal check if the crafting slot is available to be used.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name        | Type | Description                               |
| ----------- | ---- | ----------------------------------------- |
| \_available | bool | Boolean to know if the slot is available. |

### \_isSlotClaimable

```solidity
function _isSlotClaimable(bytes _id) internal view returns (bool)
```

Internal check if the crafting slot is claimable.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name | Type | Description                                           |
| ---- | ---- | ----------------------------------------------------- |
| [0]  | bool | \_available Boolean to know if the slot is claimable. |

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
