# Solidity API

## IItems

Interface for the [Items](/docs/items/Items.md) contract.

### Item

```solidity
struct Item {
  uint256 id;
  string name;
  string description;
  uint256 level_required;
  enum IItems.ItemType item_type;
  struct IItems.StatsModifiers stats_modifiers;
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
function mint(address _to, uint256 _item_id) external
```

See [Items#mint](/docs/items/Items.md#mint)

### burn

```solidity
function burn(address _from, uint256 _item_id) external
```

See [Items#burn](/docs/items/Items.md#burn)

### addAuthority

```solidity
function addAuthority(address _authority) external
```

See [Items#addAuthority](/docs/items/Items.md#addAuthority)

### removeAuthority

```solidity
function removeAuthority(address _authority) external
```

See [Items#removeAuthority](/docs/items/Items.md#removeAuthority)

### addItem

```solidity
function addItem(string _name, string _description, uint256 _level_required, enum IItems.ItemType _item_type, struct IItems.StatsModifiers _stats_modifiers, struct IItems.Attributes _attributes) external
```

See [Items#addItem](/docs/items/Items.md#addItem)

### updateItem

```solidity
function updateItem(struct IItems.Item _item) external
```

See [Items#updateItem](/docs/items/Items.md#updateItem)

### disableItem

```solidity
function disableItem(uint256 _item_id) external
```

See [Items#disableItem](/docs/items/Items.md#disableItem)

### enableItem

```solidity
function enableItem(uint256 _item_id) external
```

See [Items#enableItem](/docs/items/Items.md#enableItem)

### getItem

```solidity
function getItem(uint256 _item_id) external view returns (struct IItems.Item _item)
```

See [Items#getItem](/docs/items/Items.md#getItem)
