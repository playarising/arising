const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Items", () => {
  before(async () => {
    const [owner, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const Items = await ethers.getContractFactory("Items");
    this.items = await Items.deploy();
    await this.items.deployed();
  });

  it("should fail to mint an invalid item", async () => {
    await expect(this.items.mint(this.owner.address, 1)).to.revertedWith(
      "Items: mint() invalid item id."
    );
  });

  it("should fail to mint from non owner or authority", async () => {
    await expect(
      this.items.connect(this.minter).mint(this.owner.address, 1)
    ).to.revertedWith("Items: onlyAuthorized() msg.sender not authorized.");
  });

  it("should fail to add and remove an authority from non owner", async () => {
    await expect(
      this.items.connect(this.minter).addAuthority(this.owner.address)
    ).to.revertedWith("Ownable: caller is not the owner");
    await expect(
      this.items.connect(this.minter).removeAuthority(this.owner.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail to remove a non authorized address", async () => {
    await expect(
      this.items.removeAuthority(this.minter.address)
    ).to.revertedWith("Items: removeAuthority() address is not authorized.");
  });

  it("should fail to disabling an invalid item", async () => {
    await expect(this.items.disableItem(1)).to.revertedWith(
      "Items: disableItem() invalid item it."
    );
  });

  it("should fail to disabling an item from non owner", async () => {
    await expect(
      this.items.connect(this.minter).disableItem(1)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail to enabling an invalid item", async () => {
    await expect(this.items.disableItem(1)).to.revertedWith(
      "Items: disableItem() invalid item it."
    );
  });

  it("should fail to enabling an item from non owner", async () => {
    await expect(
      this.items.connect(this.minter).disableItem(1)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail to getting the item information of an invalid item", async () => {
    await expect(this.items.getItem(1)).to.revertedWith(
      "Items: getItem() invalid item it."
    );
  });

  it("should fail to add an item from non owner", async () => {
    await expect(
      this.items.connect(this.minter).addItem(
        1,
        1,
        {
          might: 1,
          speed: 1,
          intellect: 1,
          might_reducer: 1,
          speed_reducer: 1,
          intellect_reducer: 1,
        },
        {
          atk: 0,
          atk_reducer: 0,
          def: 0,
          def_reducer: 0,
          range: 0,
          range_reducer: 0,
          mag_atk: 0,
          mag_atk_reducer: 0,
          mag_def: 0,
          mag_def_reducer: 0,
          rate: 0,
          rate_reducer: 0,
        }
      )
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should add an item correctly", async () => {
    await this.items.addItem(
      1,
      2,
      {
        might: 1,
        speed: 1,
        intellect: 1,
        might_reducer: 1,
        speed_reducer: 1,
        intellect_reducer: 1,
      },
      {
        atk: 0,
        atk_reducer: 0,
        def: 0,
        def_reducer: 0,
        range: 0,
        range_reducer: 0,
        mag_atk: 0,
        mag_atk_reducer: 0,
        mag_def: 0,
        mag_def_reducer: 0,
        rate: 0,
        rate_reducer: 0,
      }
    );
    const item = await this.items.getItem(1);
    expect(item.level_required).to.eq(1);
    expect(item.item_type).to.eq(2);
  });

  it("should add an authority, mint and remove it correctly", async () => {
    await this.items.addAuthority(this.minter.address);
    await this.items.connect(this.minter).mint(this.owner.address, 1);
    await this.items.removeAuthority(this.minter.address);
    await expect(
      this.items.connect(this.minter).mint(this.owner.address, 1)
    ).to.revertedWith("Items: onlyAuthorized() msg.sender not authorized.");
  });

  it("should mint an item correctly", async () => {
    await this.items.mint(this.owner.address, 1);
    expect(await this.items.balanceOf(this.owner.address, 1)).to.eq(2);
  });

  it("should return the item uri correctly", async () => {
    expect(await this.items.uri(1)).to.eq("https://items.playarising.com/{id}");
  });

  it("should disable an item correctly", async () => {
    await this.items.disableItem(1);
    const item = await this.items.getItem(1);
    expect(item.available).to.eq(false);
  });

  it("should enable an item correctly", async () => {
    await this.items.enableItem(1);
    const item = await this.items.getItem(1);
    expect(item.available).to.eq(true);
  });
});
