const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Civilizations", () => {
  before(async () => {
    const [owner, minter, minter2, minter3] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;
    this.minter2 = minter2;
    this.minter3 = minter3;

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("5000"));
    await this.mock.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy();
    await this.civ.deployed();

    await this.civ.initialize(this.mock.address);
    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.collection = await BaseERC721.deploy();
    await this.collection.deployed();
    await this.collection.initialize(
      "Test Collection",
      "TEST",
      "https://test.uri/",
      this.civ.address,
    );
    await this.collection.addAuthority(this.civ.address);

    await this.civ.addCivilization(this.collection.address);
  });

  it("should not be able to mint when paused", async () => {
    expect(await this.civ.paused()).to.eq(false);
    await this.civ.pause();
    expect(await this.civ.paused()).to.eq(true);
    await expect(this.civ.mint(1)).to.revertedWith("Pausable: paused");
    await this.civ.unpause();
    expect(await this.civ.paused()).to.eq(false);
  });

  it("should not be able pause when not owner", async () => {
    await expect(this.civ.connect(this.minter).pause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
    await expect(this.civ.connect(this.minter).unpause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should try to mint to an empty instance", async () => {
    await expect(
      this.civ.mint("0x0000000000000000000000000000000000000000"),
    ).to.revertedWith("Civilizations: mint() invalid civilization id.");
  });

  it("should try to mint to a non arising civilization", async () => {
    await expect(this.civ.mint(5)).to.revertedWith(
      "Civilizations: mint() invalid civilization id.",
    );
  });

  it("should mint a token correctly", async () => {
    await expect(this.civ.mint(1))
      .to.emit(this.civ, "Summoned")
      .withArgs(
        this.owner.address,
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001",
      );
    expect(await this.collection.balanceOf(this.owner.address)).to.eq(1);
  });

  it("should attempt to add a civilization contract from a non owner", async () => {
    await expect(
      this.civ.connect(this.minter).addCivilization(this.collection.address),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail when trying to mint through a contract", async () => {
    const MockMinter = await ethers.getContractFactory("MockMinter");
    this.mockMinter = await MockMinter.deploy(this.civ.address);
    await this.mockMinter.deployed();
    await expect(this.mockMinter.mintMock(1)).to.revertedWith(
      "Civilizations: _canMint() contract cannot mint.",
    );
  });

  it("should return civilization token ids correctly", async () => {
    expect(await this.civ.getTokenID(1, 1)).to.eq(
      "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001",
    );
  });

  it("should fail when try to get the civilization token ids from an invalid instance", async () => {
    await expect(this.civ.getTokenID(5, 1)).to.revertedWith(
      "Civilizations: getTokenID() invalid civilization id.",
    );
  });

  it("should fail when try to get the civilization token ids from a non minted token", async () => {
    await expect(this.civ.getTokenID(1, 3)).to.revertedWith(
      "Civilizations: getTokenID() token not minted.",
    );
  });

  it("should return false for not allowance token", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.civ.isAllowed(this.minter.address, id)).to.eq(false);
  });

  it("should return true when allowed and approved for all", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.collection.approve(this.minter.address, 1);
    await this.collection.setApprovalForAll(this.minter2.address, true);
    expect(await this.civ.isAllowed(this.minter.address, id)).to.eq(true);
    expect(await this.civ.isAllowed(this.minter2.address, id)).to.eq(true);
  });

  it("should revert when check allowance from a wrong civilization composed ID", async () => {
    await expect(
      this.civ.isAllowed(
        this.minter.address,
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001",
      ),
    ).to.revertedWith("Civilizations: isAllowed() invalid civilization id.");
  });

  it("should check if a token is already minted", async () => {
    expect(
      await this.civ.exists(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001",
      ),
    ).to.eq(true);
  });

  it("should fail when trying to check if a token exists from an invalid civilization", async () => {
    await expect(
      this.civ.exists(
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001",
      ),
    ).to.revertedWith("Civilizations: exists() invalid civilization id.");
  });

  it("should fail when trying to check if a token exists from a non minted id", async () => {
    expect(
      await this.civ.exists(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000004",
      ),
    ).to.eq(false);
  });

  it("should check the owner of a minted token", async () => {
    expect(
      await this.civ.ownerOf(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001",
      ),
    ).to.eq(this.owner.address);
  });

  it("should fail when trying to check if the owner of a token from an invalid civilization", async () => {
    await expect(
      this.civ.ownerOf(
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001",
      ),
    ).to.revertedWith("Civilizations: ownerOf() invalid civilization id.");
  });

  it("should fail when trying to check the token owner from a non minted id", async () => {
    await expect(
      this.civ.ownerOf(
        "0x00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000004",
      ),
    ).to.revertedWith("Civilizations: ownerOf() invalid civilization id.");
  });

  it("should fail to set the mint price from non owner", async () => {
    await expect(this.civ.connect(this.minter).setMintPrice(1)).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should fail to set upgrade price from non owner", async () => {
    await expect(
      this.civ.connect(this.minter).setUpgradePrice(1, 1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail to set intialize upgrade from non owner", async () => {
    await expect(
      this.civ.connect(this.minter).setInitializeUpgrade(1, 1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail to set intialize charge token from non owner", async () => {
    await expect(
      this.civ.connect(this.minter).setToken(this.owner.address),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail when trying to set price and initialize invalid upgrades", async () => {
    await expect(this.civ.setUpgradePrice(0, 1)).to.revertedWith(
      "Civilizations: setUpgradePrice() invalid upgrade id.",
    );
    await expect(this.civ.setUpgradePrice(4, 1)).to.revertedWith(
      "Civilizations: setUpgradePrice() invalid upgrade id.",
    );
    await expect(this.civ.setInitializeUpgrade(0, 1)).to.revertedWith(
      "Civilizations: setInitializeUpgrade() invalid upgrade id.",
    );
    await expect(this.civ.setInitializeUpgrade(4, 1)).to.revertedWith(
      "Civilizations: setInitializeUpgrade() invalid upgrade id.",
    );
  });

  it("should fail trying to initialize an upgrade when no price is set", async () => {
    await expect(this.civ.setInitializeUpgrade(1, 1)).to.revertedWith(
      "Civilizations: setInitializeUpgrade() no price set.",
    );
    await expect(this.civ.setInitializeUpgrade(2, 1)).to.revertedWith(
      "Civilizations: setInitializeUpgrade() no price set.",
    );
    await expect(this.civ.setInitializeUpgrade(3, 1)).to.revertedWith(
      "Civilizations: setInitializeUpgrade() no price set.",
    );
  });

  it("should fail to get information of an invalid upgrade", async () => {
    await expect(this.civ.getUpgradeInformation(0)).to.revertedWith(
      "Civilizations: getUpgradeInformation() invalid upgrade id.",
    );
    await expect(this.civ.getUpgradeInformation(4)).to.revertedWith(
      "Civilizations: getUpgradeInformation() invalid upgrade id.",
    );
  });

  it("should set the price of the upgrades correctly", async () => {
    await this.civ.setUpgradePrice(1, ethers.utils.parseEther("39.99"));
    await this.civ.setUpgradePrice(2, ethers.utils.parseEther("49.99"));
    await this.civ.setUpgradePrice(3, ethers.utils.parseEther("89.99"));
    const upgrade1 = await this.civ.getUpgradeInformation(1);
    expect(upgrade1.available).to.eq(false);
    expect(upgrade1.price).to.eq(ethers.utils.parseEther("39.99"));
    const upgrade2 = await this.civ.getUpgradeInformation(2);
    expect(upgrade2.available).to.eq(false);
    expect(upgrade2.price).to.eq(ethers.utils.parseEther("49.99"));
    const upgrade3 = await this.civ.getUpgradeInformation(3);
    expect(upgrade3.available).to.eq(false);
    expect(upgrade3.price).to.eq(ethers.utils.parseEther("89.99"));
  });

  it("should set the mint price correctly", async () => {
    await this.civ.setMintPrice(ethers.utils.parseEther("19.99"));
    expect(await this.civ.price()).to.eq(ethers.utils.parseEther("19.99"));
  });

  it("should change the charge token correctly", async () => {
    await this.civ.setToken(this.owner.address);
    expect(await this.civ.token()).to.eq(this.owner.address);
    await this.civ.setToken(this.mock.address);
    expect(await this.civ.token()).to.eq(this.mock.address);
  });

  it("should set initialize upgrades correctly", async () => {
    await this.civ.setInitializeUpgrade(1, true);
    await this.civ.setInitializeUpgrade(2, true);
    await this.civ.setInitializeUpgrade(3, true);
    let upgrade1 = await this.civ.getUpgradeInformation(1);
    expect(upgrade1.available).to.eq(true);
    let upgrade2 = await this.civ.getUpgradeInformation(2);
    expect(upgrade2.available).to.eq(true);
    let upgrade3 = await this.civ.getUpgradeInformation(3);
    expect(upgrade3.available).to.eq(true);

    await this.civ.setInitializeUpgrade(1, false);
    await this.civ.setInitializeUpgrade(2, false);
    await this.civ.setInitializeUpgrade(3, false);
    upgrade1 = await this.civ.getUpgradeInformation(1);
    expect(upgrade1.available).to.eq(false);
    upgrade2 = await this.civ.getUpgradeInformation(2);
    expect(upgrade2.available).to.eq(false);
    upgrade3 = await this.civ.getUpgradeInformation(3);
    expect(upgrade3.available).to.eq(false);

    await this.civ.setInitializeUpgrade(1, true);
    await this.civ.setInitializeUpgrade(2, true);
    await this.civ.setInitializeUpgrade(3, true);
    upgrade1 = await this.civ.getUpgradeInformation(1);
    expect(upgrade1.available).to.eq(true);
    upgrade2 = await this.civ.getUpgradeInformation(2);
    expect(upgrade2.available).to.eq(true);
    upgrade3 = await this.civ.getUpgradeInformation(3);
    expect(upgrade3.available).to.eq(true);
  });

  it("should set initialize upgrades correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    const upgrades = await this.civ.getCharacterUpgrades(id);
    expect(upgrades[0]).to.eq(false);
    expect(upgrades[1]).to.eq(false);
    expect(upgrades[2]).to.eq(false);
  });

  it("should fail buying an upgrade from for a non available upgrade", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.civ.setInitializeUpgrade(1, false);
    await expect(this.civ.buyUpgrade(id, 1)).to.revertedWith(
      "Civilizations: buyUpgrade() upgrade is not initialized.",
    );
    await this.civ.setInitializeUpgrade(1, true);
  });

  it("should fail buying an upgrade for an invalid upgrade", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.civ.buyUpgrade(id, 0)).to.revertedWith(
      "Civilizations: buyUpgrade() invalid upgrade id.",
    );
  });

  it("should fail buying an upgrade from a non allowed address", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.civ.connect(this.minter3).buyUpgrade(id, 1),
    ).to.revertedWith(
      "Civilizations: buyUpgrade() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail buying an upgrade when no enough balance.", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.collection.setApprovalForAll(this.minter2.address, true);
    await expect(
      this.civ.connect(this.minter2).buyUpgrade(id, 1),
    ).to.revertedWith(
      "Civilizations: buyUpgrade() not enough balance to mint tokens.",
    );
  });

  it("should fail buying an upgrade when no allowance.", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.collection.setApprovalForAll(this.minter3.address, true);
    await this.mock.transfer(
      this.minter3.address,
      ethers.utils.parseEther("1000"),
    );
    await expect(
      this.civ.connect(this.minter3).buyUpgrade(id, 1),
    ).to.revertedWith(
      "Civilizations: buyUpgrade() not enough allowance to mint tokens.",
    );
  });

  it("should buy upgrades correctly.", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.collection.setApprovalForAll(this.minter3.address, true);
    await this.mock.transfer(
      this.minter3.address,
      ethers.utils.parseEther("500"),
    );
    await this.mock
      .connect(this.minter3)
      .approve(this.civ.address, ethers.utils.parseEther("1000"));
    await expect(this.civ.connect(this.minter3).buyUpgrade(id, 1))
      .to.emit(this.civ, "UpgradePurchased")
      .withArgs(id, 1);
    await expect(this.civ.connect(this.minter3).buyUpgrade(id, 2))
      .to.emit(this.civ, "UpgradePurchased")
      .withArgs(id, 2);
    await expect(this.civ.connect(this.minter3).buyUpgrade(id, 3))
      .to.emit(this.civ, "UpgradePurchased")
      .withArgs(id, 3);
    const upgrades = await this.civ.getCharacterUpgrades(id);
    expect(upgrades[0]).to.eq(true);
    expect(upgrades[1]).to.eq(true);
    expect(upgrades[2]).to.eq(true);
  });

  it("should be able to mint with price", async () => {
    expect(await this.mock.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("3679.97"),
    );
    await this.mock.approve(this.civ.address, ethers.utils.parseEther("1000"));
    await this.civ.mint(1);
    expect(await this.mock.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("3679.97"),
    );
  });
});
