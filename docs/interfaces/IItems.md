# Solidity API

## IItems

Interface for the [Items](/docs/items/Items.md) contract.

### Item

```solidity
struct Item {
  uint256 id;
  uint256 level_required;
  enum IItems.ItemType item_type;
  struct IItems.StatsModifiers stat_modifiers;
  struct IItems.Attributes attributes;
  bool available;
}
```

### ItemType

```solidity
enum ItemType {
  HELMET,
  SHOULDER_GUARDS,
  ARM_GUARDS,
  HANDS,
  RING,
  NECKLACE,
  CHEST,
  LEGS,
  BELT,
  FEET,
  CAPE,
  ONE_HANDED,
  TWO_HANDED
}

```

### StatsModifiers

```solidity
struct StatsModifiers {
  uint256 might;
  uint256 might_reducer;
  uint256 speed;
  uint256 speed_reducer;
  uint256 intellect;
  uint256 intellect_reducer;
}

```

### BaseAttributes

```solidity
struct BaseAttributes {
  uint256 atk;
  uint256 def;
  uint256 range;
  uint256 mag_atk;
  uint256 mag_def;
  uint256 rate;
}

```

### Attributes

```solidity
struct Attributes {
  uint256 atk;
  uint256 atk_reducer;
  uint256 def;
  uint256 def_reducer;
  uint256 range;
  uint256 range_reducer;
  uint256 mag_atk;
  uint256 mag_atk_reducer;
  uint256 mag_def;
  uint256 mag_def_reducer;
  uint256 rate;
  uint256 rate_reducer;
}

```

### mint

```solidity
function mint(address _to, uint256 _id) external
```

See [Items#mint](/docs/items/Items.md#mint)

### addItem

```solidity
function addItem(uint256 level_required, enum IItems.ItemType item_type, struct IItems.StatsModifiers stat_modifiers, struct IItems.Attributes attributes) external
```

See [Items#addItem](/docs/items/Items.md#addItem)

### disableItem

```solidity
function disableItem(uint256 id) external
```

See [Items#disableItem](/docs/items/Items.md#disableItem)

### enableItem

```solidity
function enableItem(uint256 id) external
```

See [Items#enableItem](/docs/items/Items.md#enableItem)

### getItem

```solidity
function getItem(uint256 id) external view returns (struct IItems.Item)
```

See [Items#getItem](/docs/items/Items.md#getItem)