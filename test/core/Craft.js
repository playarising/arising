const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Craft", () => {
  before(async () => {
    const [owner, receiver] = await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;

    const Levels = await ethers.getContractFactory("Levels");
    const levels = await Levels.deploy();
    await levels.deployed();

    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.collection = await BaseERC721.deploy(
      "Test Collection",
      "TEST",
      "https://test.uri/"
    );
    await this.collection.deployed();

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("1000"));
    await this.mock.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy(this.mock.address);
    await this.civ.deployed();

    await this.civ.addCivilization(this.collection.address);
    await this.collection.transferOwnership(this.civ.address);

    await this.civ.mint(1);
    await this.civ.connect(this.receiver).mint(1);

    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();

    const Stats = await ethers.getContractFactory("Stats");
    this.stats = await Stats.deploy(this.civ.address, this.experience.address);
    await this.stats.deployed();

    const BaseFungibleItem = await ethers.getContractFactory(
      "BaseFungibleItem"
    );

    this.gold = await BaseFungibleItem.deploy(
      "Ard: Gold",
      "GOLD",
      this.civ.address
    );
    await this.gold.deployed();

    this.resource = await BaseFungibleItem.deploy(
      "Ard: Resource",
      "WOOD",
      this.civ.address
    );
    await this.resource.deployed();

    const Items = await ethers.getContractFactory("Items");
    this.items = await Items.deploy();
    await this.items.deployed();

    const Craft = await ethers.getContractFactory("Craft");
    this.craft = await Craft.deploy(
      this.civ.address,
      this.experience.address,
      this.stats.address,
      this.gold.address,
      this.items.address
    );
    await this.craft.deployed();

    await this.experience.addAuthority(this.craft.address);

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

    await this.items.addAuthority(this.craft.address);
  });

  it("should not be able to craft when paused", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.craft.paused()).to.eq(false);
    await this.craft.pause();
    expect(await this.craft.paused()).to.eq(true);
    await expect(this.craft.craft(id, 1)).to.revertedWith("Pausable: paused");
    await this.craft.unpause();
    expect(await this.craft.paused()).to.eq(false);
  });

  it("should not be able pause when not owner", async () => {
    await expect(this.craft.connect(this.receiver).pause()).to.revertedWith(
      "Ownable: caller is not the owner"
    );
    await expect(this.craft.connect(this.receiver).unpause()).to.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should fail disabling a recibe when no owner", async () => {
    await expect(
      this.craft.connect(this.receiver).disableRecipe(1)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail enabling a recibe when no owner", async () => {
    await expect(
      this.craft.connect(this.receiver).enableRecipe(1)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail disabling a recibe when it doesn't exists", async () => {
    await expect(this.craft.disableRecipe(1)).to.revertedWith(
      "Craft: disableRecipe() invalid recipe id."
    );
  });

  it("should fail enabling a recibe when it doesn't exists", async () => {
    await expect(this.craft.enableRecipe(1)).to.revertedWith(
      "Craft: craft() invalid recipe id."
    );
  });

  it("should fail adding a recipe when no owner", async () => {
    await expect(
      this.craft
        .connect(this.receiver)
        .addRecipe([], [], { might: 1, speed: 1, intellect: 1 }, 1, 1, 1, 1, 1)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail adding a recipe when materials and amount doesn't match", async () => {
    await expect(
      this.craft.addRecipe(
        [this.resource.address],
        [],
        { might: 1, speed: 1, intellect: 1 },
        1,
        1,
        1,
        1,
        1
      )
    ).to.revertedWith("Craft: addRecipe() materials and amounts not match.");
  });

  it("should enable a recipe correctly", async () => {
    await this.craft.addRecipe(
      [this.resource.address],
      [10],
      { might: 1, speed: 1, intellect: 1 },
      3600,
      3,
      100,
      1,
      1
    );

    const recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(true);
  });

  it("should enable and disable the recupe correctly", async () => {
    let recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(true);
    await this.craft.disableRecipe(1);
    recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(false);
    await this.craft.enableRecipe(1);
    recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(true);
  });

  it("should fail when trying to craft for a non owned token", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.craft.connect(this.receiver).craft(id, 1)
    ).to.revertedWith(
      "Craft: onlyAllowed() msg.sender is not allowed to access this token."
    );
  });

  it("should fail when trying to craft for a non minted token", async () => {
    await expect(
      this.craft.craft(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000005",
        1
      )
    ).to.revertedWith("Craft: onlyAllowed() token not minted.");
  });

  it("should fail crafting an invalid recipe", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.craft(id, 2)).to.revertedWith(
      "Craft: craft() invalid recipe id."
    );
  });

  it("should fail crafting a not available recipe", async () => {
    await this.craft.disableRecipe(1);
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.craft(id, 1)).to.revertedWith(
      "Craft: craft() recipe is not available."
    );
    await this.craft.enableRecipe(1);
  });

  it("should fail crafting a recipe with not enough level", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.craft(id, 1)).to.revertedWith(
      "Craft: craft() not enough level."
    );
  });

  it("should craft a recipe correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.experience.assignExperience(id, 5000);
    await this.stats.assignPoints(id, { might: 2, speed: 2, intellect: 2 });
    await this.collection.setApprovalForAll(this.craft.address, true);
    await this.resource.mintTo(id, 10000);
    await this.gold.mintTo(id, 10000);
    await this.craft.craft(id, 1);
    const slot = await this.craft.getCharacterCrafSlot(id);
    expect(slot.last_recipe).to.eq(1);
    expect(slot.claimed).to.eq(false);
  });

  it("should fail when trying to craft a recipe with the slot not claimed", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.craft(id, 1)).to.revertedWith(
      "Craft: craft() slot not available to craft."
    );
  });

  it("should fail when trying to claim that is not ready", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.claim(id)).to.revertedWith(
      "Craft: claim() slot is not claimable."
    );
  });

  it("should claim the recipe successfully", async () => {
    const id = await this.civ.getTokenID(1, 1);
    let slot = await this.craft.getCharacterCrafSlot(id);
    await ethers.provider.send("evm_mine", [slot.cooldown.toNumber()]);
    expect(await this.items.balanceOf(this.owner.address, 1)).to.eq(0);
    await this.craft.claim(id);
    expect(await this.items.balanceOf(this.owner.address, 1)).to.eq(1);
    slot = await this.craft.getCharacterCrafSlot(id);
    expect(slot.claimed).to.eq(true);
  });
});
