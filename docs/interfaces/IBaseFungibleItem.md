# Solidity API

## IBaseFungibleItem

Interface for the [BaseFungibleItem](/docs/base/BaseFungibleItem.md) contract.

### mintTo

```solidity
function mintTo(bytes _id, uint256 _amount) external
```

See [BaseFungibleItem#mintTo](/docs/base/BaseFungibleItem.md#mintTo)

### consume

```solidity
function consume(bytes _id, uint256 _amount) external
```

See [BaseFungibleItem#consume](/docs/base/BaseFungibleItem.md#consume)

### wrap

```solidity
function wrap(bytes _id, uint256 _amount) external
```

See [BaseFungibleItem#wrap](/docs/base/BaseFungibleItem.md#wrap)

### unwrap

```solidity
function unwrap(bytes _id, uint256 _amount) external
```

See [BaseFungibleItem#unwrap](/docs/base/BaseFungibleItem.md#unwrap)

### balanceOf

```solidity
function balanceOf(bytes _id) external view returns (uint256 _balance)
```

See [BaseFungibleItem#balanceOf](/docs/base/BaseFungibleItem.md#balanceOf)
