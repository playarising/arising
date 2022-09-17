# Solidity API

## ICraft

Interface for the [Craft](/docs/core/Craft.md) contract.

### Recipe

```solidity
struct Recipe {
  uint256 id;
  address[] materials;
  uint256[] material_amounts;
  struct IStats.BasicStats stats_required;
  uint256 cooldown;
  uint256 level_required;
  uint256 experience_reward;
  uint256 cost;
  bool available;
  uint256 item_reward;
}
```

### CraftSlot

```solidity
struct CraftSlot {
  uint256 cooldown;
  uint256 last_recipe;
  bool claimed;
}

```

### pause

```solidity
function pause() external
```

See [Craft#pause](/docs/core/Craft.md#pause)

### unpause

```solidity
function unpause() external
```

See [Craft#unpause](/docs/core/Craft.md#unpause)

### disableRecipe

```solidity
function disableRecipe(uint256 _recipe_id) external
```

See [Craft#disableRecipe](/docs/core/Craft.md#disableRecipe)

### enableRecipe

```solidity
function enableRecipe(uint256 _recipe_id) external
```

See [Craft#enableRecipe](/docs/core/Craft.md#enableRecipe)

### craft

```solidity
function craft(bytes _id, uint256 _recipe_id) external
```

See [Craft#craft](/docs/core/Craft.md#craft)

### claim

```solidity
function claim(bytes _id) external
```

See [Craft#claim](/docs/core/Craft.md#claim)

### getRecipe

```solidity
function getRecipe(uint256 _recipe_id) external view returns (struct ICraft.Recipe _recipe)
```

See [Craft#getRecipe](/docs/core/Craft.md#getRecipe)

### getCharacterSlot

```solidity
function getCharacterSlot(bytes _id) external view returns (struct ICraft.CraftSlot _slot)
```

See [Craft#getCharacterSlot](/docs/core/Craft.md#getCharacterSlot)
