# Solidity API

## ICivilizations

Interface for the [Civilizations](/docs/core/Civilizations.md) contract.

### Upgrade

```solidity
struct Upgrade {
  uint256 price;
  bool available;
}

```

### UpgradedCharacters

```solidity
struct UpgradedCharacters {
  mapping(bytes => bool) upgrade_1;
  mapping(bytes => bool) upgrade_2;
  mapping(bytes => bool) upgrade_3;
}

```

### TokenUpgrades

```solidity
struct TokenUpgrades {
  bool upgrade_1;
  bool upgrade_2;
  bool upgrade_3;
}

```

### setInitializeUpgrade

```solidity
function setInitializeUpgrade(uint256 upgrade, bool available) external
```

### setUpgradePrice

```solidity
function setUpgradePrice(uint256 upgrade, uint256 price) external
```

### setToken

```solidity
function setToken(address _token) external
```

### addCivilization

```solidity
function addCivilization(address _instance) external
```

### mint

```solidity
function mint(address _instance) external
```

### buyUpgrade

```solidity
function buyUpgrade(bytes id, uint256 upgrade) external
```

### withdraw

```solidity
function withdraw() external
```

### getID

```solidity
function getID(address _instance) external view returns (uint256)
```

### getTokenUpgrades

```solidity
function getTokenUpgrades(bytes id) external view returns (struct ICivilizations.TokenUpgrades)
```

### getUpgradeInformation

```solidity
function getUpgradeInformation(uint256 upgrade) external view returns (struct ICivilizations.Upgrade)
```

### getCivilizations

```solidity
function getCivilizations() external view returns (address[])
```

### getTokenID

```solidity
function getTokenID(address _instance, uint256 _id) external view returns (bytes)
```

### isAllowed

```solidity
function isAllowed(address spender, bytes _id) external view returns (bool)
```

### exists

```solidity
function exists(bytes _id) external view returns (bool)
```

### ownerOf

```solidity
function ownerOf(bytes _id) external view returns (address)
```
