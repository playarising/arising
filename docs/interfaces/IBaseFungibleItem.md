# Solidity API

## IBaseFungibleItem

Interface for the {BaseFungibleItem} contract.

### mintTo

```solidity
function mintTo(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.mintTo}

### consume

```solidity
function consume(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.consume}

### wrap

```solidity
function wrap(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.wrap}

### unwrap

```solidity
function unwrap(bytes _id, uint256 _amount) external
```

See {BaseFungibleItem.unwrap}

### balanceOf

```solidity
function balanceOf(bytes _id) external view returns (uint256 _balance)
```

See {BaseFungibleItem.balanceOf}
