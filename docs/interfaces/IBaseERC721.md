# Solidity API

## IBaseERC721

Interface for the [BaseERC721](/docs/base/BaseERC721.md) contract.

### addAuthority

```solidity
function addAuthority(address _authority) external
```

See [BaseERC721#addAuthority](/docs/base/BaseERC721.md#addAuthority)

### removeAuthority

```solidity
function removeAuthority(address _authority) external
```

See [BaseERC721#removeAuthority](/docs/base/BaseERC721.md#removeAuthority)

### mint

```solidity
function mint(address _to) external returns (uint256 _token_id)
```

See [BaseERC721#mint](/docs/base/BaseERC721.md#mint)

### isApprovedOrOwner

```solidity
function isApprovedOrOwner(address _spender, uint256 _token_id) external view returns (bool _approved)
```

See [BaseERC721#isApprovedOrOwner](/docs/base/BaseERC721.md#isApprovedOrOwner)

### exists

```solidity
function exists(uint256 _token_id) external view returns (bool _exist)
```

See [BaseERC721#exists](/docs/base/BaseERC721.md#exists)
