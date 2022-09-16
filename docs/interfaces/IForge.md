# Solidity API

## IForge

Interface for the {Forge} contract.

### Recipe

```solidity
struct Recipe {
  uint256 id;
  address[] materials;
  uint256[] material_amounts;
  struct IStats.BasicStats stats_required;
  uint256 cooldown;
  uint256 level_required;
  address reward;
  uint256 experience_reward;
  uint256 cost;
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

### Forges

```solidity
struct Forges {
  struct IForge.Forge forge_1;
  struct IForge.Forge forge_2;
  struct IForge.Forge forge_3;
}
```

### disableRecipe

```solidity
function disableRecipe(uint256 id) external
```

### enableRecipe

```solidity
function enableRecipe(uint256 id) external
```

### addRecipe

```solidity
function addRecipe(address[] _materials, uint256[] _amounts, struct IStats.BasicStats stats, uint256 cooldown, uint256 level_required, uint256 cost, uint256 exp_reward, address reward) external
```

### buyUpgrade

```solidity
function buyUpgrade(bytes id) external
```

### forge

```solidity
function forge(bytes id, uint256 recipe, uint256 _forge) external
```

### claim

```solidity
function claim(bytes id, uint256 _forge) external
```

### withdraw

```solidity
function withdraw() external
```

### getRecipe

```solidity
function getRecipe(uint256 id) external view returns (struct IForge.Recipe)
```

### getCharacterForge

```solidity
function getCharacterForge(bytes id, uint256 _forge) external view returns (struct IForge.Forge)
```

### getCharacterForgesUpgrades

```solidity
function getCharacterForgesUpgrades(bytes id) external view returns (bool[3])
```

### getCharacterForgesAvailability

```solidity
function getCharacterForgesAvailability(bytes id) external view returns (bool[3])
```
