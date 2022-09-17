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
