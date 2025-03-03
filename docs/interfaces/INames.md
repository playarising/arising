# Solidity API

## INames

Interface for the [Names](/docs/core/Names.md) contract.

### pause

```solidity
function pause() external
```

See [Names#pause](/docs/codex/Names.md#pause)

### unpause

```solidity
function unpause() external
```

See [Names#unpause](/docs/codex/Names.md#unpause)

### claimName

```solidity
function claimName(bytes _id, string _name) external
```

See [Names#claimName](/docs/codex/Names.md#claimName)

### replaceName

```solidity
function replaceName(bytes _id, string _new_name) external
```

See [Names#replaceName](/docs/codex/Names.md#replaceName)

### clearName

```solidity
function clearName(bytes _id) external
```

See [Names#clearName](/docs/codex/Names.md#clearName)

### getCharacterName

```solidity
function getCharacterName(bytes _id) external view returns (string _name)
```

See [Names#getCharacterName](/docs/codex/Names.md#getCharacterName)

### isNameAvailable

```solidity
function isNameAvailable(string _name) external view returns (bool _available)
```

See [Names#isNameAvailable](/docs/codex/Names.md#isNameAvailable)

### isNameValid

```solidity
function isNameValid(string _name) external pure returns (bool _available)
```

See [Names#isNameValid](/docs/codex/Names.md#isNameValid)

### toLowerCase

```solidity
function toLowerCase(string _name) external pure returns (string _lower_case)
```

See [Names#toLowerCase](/docs/codex/Names.md#toLowerCase)
