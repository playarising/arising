# Solidity API

## IForge

Interface for the [Forge](/docs/core/Forge.md) contract.

### Recipe

```solidity
struct Recipe {
  uint256 id;
  string name;
  string description;
  address[] materials;
  uint256[] amounts;
  struct IStats.BasicStats stats_required;
  uint256 cooldown;
  uint256 level_required;
  address reward;
  uint256 experience_reward;
  bool available;
}
```

### Forge

```solidity
struct Forge {
  bool available;
  uint256 cooldown;
  uint256 last_recipe;
  bool last_recipe_claimed;
}

```

### pause

```solidity
function pause() external
```

See [Forge#pause](/docs/core/Forge.md#pause)

### unpause

```solidity
function unpause() external
```

See [Forge#unpause](/docs/core/Forge.md#unpause)

### disableRecipe

```solidity
function disableRecipe(uint256 _recipe_id) external
```

See [Forge#disableRecipe](/docs/core/Forge.md#disableRecipe)

### enableRecipe

```solidity
function enableRecipe(uint256 _recipe_id) external
```

See [Forge#enableRecipe](/docs/core/Forge.md#enableRecipe)

### addRecipe

```solidity
function addRecipe(string _name, string _description, address[] _materials, uint256[] _amounts, struct IStats.BasicStats _stats, uint256 _cooldown, uint256 _level_required, address _reward, uint256 _experience_reward) external
```

See [Forge#addRecipe](/docs/core/Forge.md#addRecipe)

### updateRecipe

```solidity
function updateRecipe(struct IForge.Recipe _recipe) external
```

See [Forge#updateRecipe](/docs/core/Forge.md#updateRecipe)

### buyUpgrade

```solidity
function buyUpgrade(bytes _id) external
```

See [Forge#buyUpgrade](/docs/core/Forge.md#buyUpgrade)

### forge

```solidity
function forge(bytes _id, uint256 _recipe_id, uint256 _forge_id) external
```

See [Forge#forge](/docs/core/Forge.md#forge)

### claim

```solidity
function claim(bytes _id, uint256 _forge_id) external
```

See [Forge#claim](/docs/core/Forge.md#claim)

### getRecipe

```solidity
function getRecipe(uint256 _recipe_id) external view returns (struct IForge.Recipe _recipe)
```

See [Forge#getRecipe](/docs/core/Forge.md#getRecipe)

### getCharacterForge

```solidity
function getCharacterForge(bytes _id, uint256 _forge_id) external view returns (struct IForge.Forge _forge)
```

See [Forge#getCharacterForge](/docs/core/Forge.md#getCharacterForge)

### getCharacterForgesUpgrades

```solidity
function getCharacterForgesUpgrades(bytes _id) external view returns (bool[3] _upgrades)
```

See [Forge#getCharacterForgesUpgrades](/docs/core/Forge.md#getCharacterForgesUpgrades)

### getCharacterForgesAvailability

```solidity
function getCharacterForgesAvailability(bytes _id) external view returns (bool[3] _availability)
```

See [Forge#getCharacterForgesAvailability](/docs/core/Forge.md#getCharacterForgesAvailability)
