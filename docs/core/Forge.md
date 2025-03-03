# Solidity API

## Forge

This contract convets the raw resources into craftable material. It uses multiple instances of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) items.
Each character has access to a maximum of three usable forges to convert the resources.

Implementation of the [IForge](/docs/interfaces/IForge.md) interface.

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

### recipes

```solidity
mapping(uint256 => struct IForge.Recipe) recipes
```

Map to track available recipes on the forge.

### \_recipes

```solidity
uint256[] _recipes
```

Array to track all the forge recipes IDs.

### forges

```solidity
mapping(bytes => mapping(uint256 => struct IForge.Forge)) forges
```

Map to track forges and cooldowns for characters.

### token

```solidity
address token
```

Constant for address of the `ERC20` token used to purchase forge upgrades.

### price

```solidity
uint256 price
```

Constant for the price of each forge upgrade (in wei).

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

### initialize

```solidity
function initialize(address _civilizations, address _experience, address _stats, address _token, uint256 _price) public
```

Initialize.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |
| \_stats         | address | The address of the [Stats](/docs/core/Stats.md) instance.                 |
| \_token         | address | Address of the token used to purchase.                                    |
| \_price         | uint256 | Price for each upgrade.                                                   |

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

### setUpgradePrice

```solidity
function setUpgradePrice(uint256 _price) public
```

Sets the price to upgrade a character.

Requirements:

| Name    | Type    | Description                              |
| ------- | ------- | ---------------------------------------- |
| \_price | uint256 | Amount of tokens to pay for the upgrade. |

### disableRecipe

```solidity
function disableRecipe(uint256 _recipe_id) public
```

Disables a recipe from beign forged.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

### enableRecipe

```solidity
function enableRecipe(uint256 _recipe_id) public
```

Enables a recipe to be forged.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

### addRecipe

```solidity
function addRecipe(string _name, string _description, address[] _materials, uint256[] _amounts, struct IStats.BasicStats _stats, uint256 _cooldown, uint256 _level_required, address _reward, uint256 _experience_reward) public
```

Adds a new recipe to the forge.

Requirements:

| Name                | Type                     | Description                                                                                                |
| ------------------- | ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| \_name              | string                   | Name of the recipe.                                                                                        |
| \_description       | string                   | Description of the recipe.                                                                                 |
| \_materials         | address[]                | Array of material [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances address.                    |
| \_amounts           | uint256[]                | Array of amounts for each material.                                                                        |
| \_stats             | struct IStats.BasicStats | Stats to consume from the pool for recipe.                                                                 |
| \_cooldown          | uint256                  | Number of seconds for the recipe cooldown.                                                                 |
| \_level_required    | uint256                  | Minimum level required to forge the recipe.                                                                |
| \_reward            | address                  | Address of the [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances to be rewarded for the recipe. |
| \_experience_reward | uint256                  | Amount of experience rewarded for the recipe.                                                              |

### updateRecipe

```solidity
function updateRecipe(struct IForge.Recipe _recipe) public
```

Updates a previously added forge recipe.

Requirements:

| Name     | Type                 | Description                     |
| -------- | -------------------- | ------------------------------- |
| \_recipe | struct IForge.Recipe | Full information of the recipe. |

### buyUpgrade

```solidity
function buyUpgrade(bytes _id) public
```

Purchases a forge upgrade for the character provided.

Requirements:

| Name | Type  | Description                    |
| ---- | ----- | ------------------------------ |
| \_id | bytes | Composed ID of the characrter. |

### forge

```solidity
function forge(bytes _id, uint256 _recipe_id, uint256 _forge_id) public
```

Forges a recipe and assigns it to the forge provided.

Requirements:

| Name        | Type    | Description                           |
| ----------- | ------- | ------------------------------------- |
| \_id        | bytes   | Composed ID of the characrter.        |
| \_recipe_id | uint256 | ID of the recipe to forge.            |
| \_forge_id  | uint256 | ID of the forge to assign the recipe. |

### claim

```solidity
function claim(bytes _id, uint256 _forge_id) public
```

Claims a recipe already forged.

Requirements:

| Name       | Type    | Description                           |
| ---------- | ------- | ------------------------------------- |
| \_id       | bytes   | Composed ID of the character.         |
| \_forge_id | uint256 | ID of the forge to assign the recipe. |

### getRecipe

```solidity
function getRecipe(uint256 _recipe_id) public view returns (struct IForge.Recipe _recipe)
```

External function to return the recipe information.

Requirements:

| Name        | Type    | Description             |
| ----------- | ------- | ----------------------- |
| \_recipe_id | uint256 | ID of the forge recipe. |

| Name     | Type                 | Description                     |
| -------- | -------------------- | ------------------------------- |
| \_recipe | struct IForge.Recipe | Full information of the recipe. |

### getCharacterForge

```solidity
function getCharacterForge(bytes _id, uint256 _forge_id) public view returns (struct IForge.Forge _forge)
```

External function to return the information of a character forge.

Requirements:

| Name       | Type    | Description                   |
| ---------- | ------- | ----------------------------- |
| \_id       | bytes   | Composed ID of the character. |
| \_forge_id | uint256 | ID of the forge.              |

| Name    | Type                | Description                    |
| ------- | ------------------- | ------------------------------ |
| \_forge | struct IForge.Forge | Full information of the forge. |

### getCharacterForgesUpgrades

```solidity
function getCharacterForgesUpgrades(bytes _id) public view returns (bool[3] _upgrades)
```

External function to return an array of booleans with the purchased forge upgrades for a character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name       | Type    | Description                              |
| ---------- | ------- | ---------------------------------------- |
| \_upgrades | bool[3] | Array of booleans of upgrades purchases. |

### getCharacterForgesAvailability

```solidity
function getCharacterForgesAvailability(bytes _id) public view returns (bool[3] _availability)
```

External function to return an array of booleans with the availability of the character forges.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name           | Type    | Description                              |
| -------------- | ------- | ---------------------------------------- |
| \_availability | bool[3] | Array of booleans of forge availability. |

### \_isForgeAvailable

```solidity
function _isForgeAvailable(bytes _id, uint256 _forge_id) internal view returns (bool _available)
```

Internal function to check if a character forge is available.

Requirements:

| Name       | Type    | Description                   |
| ---------- | ------- | ----------------------------- |
| \_id       | bytes   | Composed ID of the character. |
| \_forge_id | uint256 | ID of the forge.              |

| Name        | Type | Description                                |
| ----------- | ---- | ------------------------------------------ |
| \_available | bool | Boolean to know if the forge is available. |

### \_isForgeClaimable

```solidity
function _isForgeClaimable(bytes _id, uint256 _forge_id) internal view returns (bool _claimable)
```

Internal function to check if a character forge is claimable.

Requirements:

| Name       | Type    | Description                   |
| ---------- | ------- | ----------------------------- |
| \_id       | bytes   | Composed ID of the character. |
| \_forge_id | uint256 | ID of the forge.              |

| Name        | Type | Description                                |
| ----------- | ---- | ------------------------------------------ |
| \_claimable | bool | Boolean to know if the forge is claimable. |

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
