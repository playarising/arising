# Solidity API

## ICraft

Interface for the [Craft](/docs/core/Craft.md) contract.

### Recipe

```solidity
struct Recipe {
  uint256 id;
  string name;
  string description;
  address[] materials;
  uint256[] material_amounts;
  struct IStats.BasicStats stats_required;
  uint256 cooldown;
  uint256 level_required;
  uint256 reward;
  uint256 experience_reward;
  bool available;
}
```

### Upgrade

```solidity
struct Upgrade {
  uint256 id;
  string name;
  string description;
  address[] materials;
  uint256[] material_amounts;
  struct IStats.BasicStats stats_required;
  struct IStats.BasicStats stats_sacrificed;
  uint256 level_required;
  uint256 upgraded_item;
  uint256 reward;
  bool available;
}
```

### Slot

```solidity
struct Slot {
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

### disableUpgrade

```solidity
function disableUpgrade(uint256 _upgrade_id) external
```

See [Craft#disableUpgrade](/docs/core/Craft.md#disableUpgrade)

### enableUpgrade

```solidity
function enableUpgrade(uint256 _upgrade_id) external
```

See [Craft#enableUpgrade](/docs/core/Craft.md#enableUpgrade)

### addRecipe

```solidity
function addRecipe(string _name, string _description, address[] _materials, uint256[] _amounts, struct IStats.BasicStats _stats, uint256 _cooldown, uint256 _level_required, uint256 _reward, uint256 _experience_reward) external
```

See [Craft#addRecipe](/docs/core/Craft.md#addRecipe)

### updateRecipe

```solidity
function updateRecipe(struct ICraft.Recipe _recipe) external
```

See [Craft#updateRecipe](/docs/core/Craft.md#updateRecipe)

### addUpgrade

```solidity
function addUpgrade(string _name, string _description, address[] _materials, uint256[] _amounts, struct IStats.BasicStats _stats, struct IStats.BasicStats _sacrifice, uint256 _level_required, uint256 _upgraded_item, uint256 _reward) external
```

See [Craft#addUpgrade](/docs/core/Craft.md#addUpgrade)

### updateUpgrade

```solidity
function updateUpgrade(struct ICraft.Upgrade _upgrade) external
```

See [Craft#updateUpgrade](/docs/core/Craft.md#updateUpgrade)

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

### upgrade

```solidity
function upgrade(bytes _id, uint256 _upgrade_id) external
```

See [Craft#upgrade](/docs/core/Craft.md#upgrade)

### getRecipe

```solidity
function getRecipe(uint256 _recipe_id) external view returns (struct ICraft.Recipe _recipe)
```

See [Craft#getRecipe](/docs/core/Craft.md#getRecipe)

### getUpgrade

```solidity
function getUpgrade(uint256 _upgrade_id) external view returns (struct ICraft.Upgrade _upgrade)
```

See [Craft#getUpgrade](/docs/core/Craft.md#getUpgrade)

### getCharacterCrafSlot

```solidity
function getCharacterCrafSlot(bytes _id) external view returns (struct ICraft.Slot _slot)
```

See [Craft#getCharacterCrafSlot](/docs/core/Craft.md#getCharacterCrafSlot)
