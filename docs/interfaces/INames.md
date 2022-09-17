# Solidity API

## INames

Interface for the [Names](/docs/core/Names.md) contract.

### claimName

```solidity
function claimName(bytes id, string name) external
```

### replaceName

```solidity
function replaceName(bytes id, string newName) external
```

### clearName

```solidity
function clearName(bytes id) external
```

### getTokenName

```solidity
function getTokenName(bytes id) external view returns (string)
```

### isNameAvailable

```solidity
function isNameAvailable(string str) external view returns (bool)
```

### isNameValid

```solidity
function isNameValid(string str) external pure returns (bool)
```

### toLowerCase

```solidity
function toLowerCase(string str) external pure returns (string)
```
