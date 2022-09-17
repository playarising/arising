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

### pause

```solidity
function pause() external
```

See [Civilizations#pause](/docs/core/Civilizations.md#pause)

### unpause

```solidity
function unpause() external
```

See [Civilizations#unpause](/docs/core/Civilizations.md#unpause)

### setInitializeUpgrade

```solidity
function setInitializeUpgrade(uint256 _upgrade_id, bool _available) external
```

See [Civilizations#setInitializeUpgrade](/docs/core/Civilizations.md#setInitializeUpgrade)

### setUpgradePrice

```solidity
function setUpgradePrice(uint256 _upgrade_id, uint256 _price) external
```

See [Civilizations#setUpgradePrice](/docs/core/Civilizations.md#setUpgradePrice)

### setToken

```solidity
function setToken(address _token) external
```

See [Civilizations#setToken](/docs/core/Civilizations.md#setToken)

### addCivilization

```solidity
function addCivilization(address _civilization) external
```

See [Civilizations#addCivilization](/docs/core/Civilizations.md#addCivilization)

### mint

```solidity
function mint(address _civilization) external
```

See [Civilizations#mint](/docs/core/Civilizations.md#mint)

### buyUpgrade

```solidity
function buyUpgrade(bytes _id, uint256 _upgrade_id) external
```

See [Civilizations#buyUpgrade](/docs/core/Civilizations.md#buyUpgrade)

### withdraw

```solidity
function withdraw() external
```

See [Civilizations#withdraw](/docs/core/Civilizations.md#withdraw)

### getID

```solidity
function getID(address _civilization) external view returns (uint256)
```

See [Civilizations#getID](/docs/core/Civilizations.md#getID)

### getCharacterUpgrades

```solidity
function getCharacterUpgrades(bytes _id) external view returns (bool[3])
```

See [Civilizations#getCharacterUpgrades](/docs/core/Civilizations.md#getCharacterUpgrades)

### getUpgradeInformation

```solidity
function getUpgradeInformation(uint256 _upgrade_id) external view returns (struct ICivilizations.Upgrade)
```

See [Civilizations#getUpgradeInformation](/docs/core/Civilizations.md#getUpgradeInformation)

### getCivilizations

```solidity
function getCivilizations() external view returns (address[])
```

See [Civilizations#getCivilizations](/docs/core/Civilizations.md#getCivilizations)

### getTokenID

```solidity
function getTokenID(address _civilization, uint256 _token_id) external view returns (bytes)
```

See [Civilizations#getTokenID](/docs/core/Civilizations.md#getTokenID)

### isAllowed

```solidity
function isAllowed(address _spender, bytes _id) external view returns (bool)
```

See [Civilizations#isAllowed](/docs/core/Civilizations.md#isAllowed)

### exists

```solidity
function exists(bytes _id) external view returns (bool)
```

See [Civilizations#exists](/docs/core/Civilizations.md#exists)

### ownerOf

```solidity
function ownerOf(bytes _id) external view returns (address)
```

See [Civilizations#ownerOf](/docs/core/Civilizations.md#ownerOf)
