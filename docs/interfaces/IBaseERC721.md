# Solidity API

## IBaseERC721

Interface for the [BaseERC721](/docs/base/BaseERC721.md) contract.

### mint

```solidity
function mint(address _to) external
```

See [BaseERC721#mint](/docs/base/BaseERC721.md#mint)

### isApprovedOrOwner

```solidity
function isApprovedOrOwner(address _spender, uint256 _id) external view returns (bool _approved)
```

See [BaseERC721#isApprovedOrOwner](/docs/base/BaseERC721.md#isApprovedOrOwner)

### exists

```solidity
function exists(uint256 _id) external view returns (bool _exist)
```

See [BaseERC721#exists](/docs/base/BaseERC721.md#exists)