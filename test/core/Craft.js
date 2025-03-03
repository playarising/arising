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
    await levels.initialize();

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("1000"));
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

    await this.civ.mint(1);
    await this.civ.connect(this.receiver).mint(1);

    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy();
    await this.experience.deployed();
    await this.experience.initialize(this.civ.address, levels.address);

    const Items = await ethers.getContractFactory("Items");
    this.items = await Items.deploy();
    await this.items.deployed();
    await this.items.initialize();

    const Equipment = await ethers.getContractFactory("Equipment");
    this.equipment = await Equipment.deploy();
    await this.equipment.deployed();
    await this.equipment.initialize(
      this.civ.address,
      this.experience.address,
      this.items.address,
    );

    const BaseGadgetToken = await ethers.getContractFactory("BaseGadgetToken");
    this.refresher = await BaseGadgetToken.deploy();
    await this.refresher.deployed();
    await this.refresher.initialize(
      "Arising: Refresher",
      "REFRESER",
      this.mock.address,
      ethers.utils.parseEther("1"),
    );
    this.vitalizer = await BaseGadgetToken.deploy();
    await this.vitalizer.deployed();
    await this.vitalizer.initialize(
      "Arising: Vitalizer",
      "VITALIZER",
      this.mock.address,
      ethers.utils.parseEther("1"),
    );

    const Stats = await ethers.getContractFactory("Stats");
    this.stats = await Stats.deploy();
    await this.stats.deployed();
    await this.stats.initialize(
      this.civ.address,
      this.experience.address,
      this.equipment.address,
      this.refresher.address,
      this.vitalizer.address,
    );
    const BaseFungibleItem =
      await ethers.getContractFactory("BaseFungibleItem");

    this.gold = await BaseFungibleItem.deploy();
    await this.gold.deployed();
    await this.gold.initialize("Ard: Gold", "GOLD", this.civ.address);

    this.resource = await BaseFungibleItem.deploy();
    await this.resource.deployed();
    await this.resource.initialize("Ard: Resource", "WOOD", this.civ.address);
    const Craft = await ethers.getContractFactory("Craft");
    this.craft = await Craft.deploy();
    await this.craft.deployed();
    await this.craft.initialize(
      this.civ.address,
      this.experience.address,
      this.stats.address,
      this.items.address,
    );
    await this.experience.addAuthority(this.craft.address);

    await this.items.addItem(
      "test",
      "test 2",
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
      },
    );

    await this.items.addItem(
      "test",
      "test 2",
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
      },
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
      "Ownable: caller is not the owner",
    );
    await expect(this.craft.connect(this.receiver).unpause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should fail disabling a recibe when no owner", async () => {
    await expect(
      this.craft.connect(this.receiver).disableRecipe(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail enabling a recibe when no owner", async () => {
    await expect(
      this.craft.connect(this.receiver).enableRecipe(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail disabling an upgrade when no owner", async () => {
    await expect(
      this.craft.connect(this.receiver).disableUpgrade(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail enabling an upgrade when no owner", async () => {
    await expect(
      this.craft.connect(this.receiver).enableUpgrade(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail disabling a recibe when it doesn't exists", async () => {
    await expect(this.craft.disableRecipe(1)).to.revertedWith(
      "Craft: disableRecipe() invalid recipe id.",
    );
  });

  it("should fail enabling a recibe when it doesn't exists", async () => {
    await expect(this.craft.enableRecipe(1)).to.revertedWith(
      "Craft: enableRecipe() invalid recipe id.",
    );
  });

  it("should fail disabling an upgrade when it doesn't exists", async () => {
    await expect(this.craft.disableUpgrade(1)).to.revertedWith(
      "Craft: disableUpgrade() invalid upgrade id.",
    );
  });

  it("should fail enabling an upgrade when it doesn't exists", async () => {
    await expect(this.craft.enableUpgrade(1)).to.revertedWith(
      "Craft: enableUpgrade() invalid upgrade id.",
    );
  });

  it("should fail adding a recipe when no owner", async () => {
    await expect(
      this.craft
        .connect(this.receiver)
        .addRecipe(
          "Test",
          "test 2",
          [],
          [],
          { might: 1, speed: 1, intellect: 1 },
          1,
          1,
          1,
          1,
        ),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail adding an upgrade when no owner", async () => {
    await expect(
      this.craft
        .connect(this.receiver)
        .addUpgrade(
          "Test",
          "test 2",
          [],
          [],
          { might: 1, speed: 1, intellect: 1 },
          { might: 1, speed: 1, intellect: 1 },
          1,
          1,
          1,
        ),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail adding a recipe when materials and amount doesn't match", async () => {
    await expect(
      this.craft.addRecipe(
        "Test",
        "test 2",
        [this.resource.address],
        [],
        { might: 1, speed: 1, intellect: 1 },
        1,
        1,
        1,
        1,
      ),
    ).to.revertedWith("Craft: addRecipe() materials and amounts not match.");
  });

  it("should fail adding an upgrade when no owner", async () => {
    await expect(
      this.craft.addUpgrade(
        "Test",
        "test 2",
        [this.resource.address],
        [],
        { might: 1, speed: 1, intellect: 1 },
        { might: 1, speed: 1, intellect: 1 },
        1,
        1,
        1,
      ),
    ).to.revertedWith("Craft: addUpgrade() materials and amounts not match.");
  });

  it("should add a recipe correctly", async () => {
    await expect(
      this.craft.addRecipe(
        "Test",
        "test 2",
        [this.resource.address],
        [10],
        { might: 1, speed: 1, intellect: 1 },
        3600,
        3,
        1,
        1,
      ),
    )
      .to.emit(this.craft, "AddRecipe")
      .withArgs(1, "Test", "test 2");

    const recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(true);
  });

  it("should add an upgrade correctly", async () => {
    await expect(
      this.craft.addUpgrade(
        "Test",
        "test 2",
        [this.resource.address],
        [10],
        { might: 1, speed: 1, intellect: 1 },
        { might: 1, speed: 1, intellect: 1 },
        5,
        1,
        2,
      ),
    )
      .to.emit(this.craft, "AddUpgrade")
      .withArgs(1, "Test", "test 2");

    const upgrade = await this.craft.getUpgrade(1);
    expect(upgrade.available).to.eq(true);
  });

  it("should enable and disable the recipe correctly", async () => {
    let recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(true);
    await expect(this.craft.disableRecipe(1))
      .to.emit(this.craft, "DisableRecipe")
      .withArgs(1);
    recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(false);
    await expect(this.craft.enableRecipe(1))
      .to.emit(this.craft, "EnableRecipe")
      .withArgs(1);
    recipe = await this.craft.getRecipe(1);
    expect(recipe.available).to.eq(true);
  });

  it("should enable and disable the upgrade correctly", async () => {
    let upgrade = await this.craft.getUpgrade(1);
    expect(upgrade.available).to.eq(true);
    await expect(this.craft.disableUpgrade(1))
      .to.emit(this.craft, "DisableUpgrade")
      .withArgs(1);
    upgrade = await this.craft.getUpgrade(1);
    expect(upgrade.available).to.eq(false);
    await expect(this.craft.enableUpgrade(1))
      .to.emit(this.craft, "EnableUpgrade")
      .withArgs(1);
    upgrade = await this.craft.getUpgrade(1);
    expect(upgrade.available).to.eq(true);
  });

  it("should fail when trying to craft for a non owned token", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.craft.connect(this.receiver).craft(id, 1),
    ).to.revertedWith(
      "Craft: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail when trying to craft for a non minted token", async () => {
    await expect(
      this.craft.craft(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000005",
        1,
      ),
    ).to.revertedWith("Craft: onlyAllowed() token not minted.");
  });

  it("should fail crafting an invalid recipe", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.craft(id, 2)).to.revertedWith(
      "Craft: craft() invalid recipe id.",
    );
  });

  it("should fail crafting a not available recipe", async () => {
    await this.craft.disableRecipe(1);
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.craft(id, 1)).to.revertedWith(
      "Craft: craft() recipe is not available.",
    );
    await this.craft.enableRecipe(1);
  });

  it("should fail crafting a recipe with not enough level", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.craft(id, 1)).to.revertedWith(
      "Craft: craft() not enough level.",
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
      "Craft: craft() slot not available to craft.",
    );
  });

  it("should fail when trying to claim that is not ready", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.claim(id)).to.revertedWith(
      "Craft: claim() slot is not claimable.",
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

  it("should fail when trying to upgrade an invalid upgrade", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.upgrade(id, 2)).to.revertedWith(
      "Craft: upgrade() invalid recipe id.",
    );
  });

  it("should fail when trying to upgrade an upgrade not available", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.craft.disableUpgrade(1);
    await expect(this.craft.upgrade(id, 1)).to.revertedWith(
      "Craft: upgrade() upgrade is not available.",
    );
    await this.craft.enableUpgrade(1);
  });

  it("should fail when trying to upgrade an upgrade with not enough level", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.craft.upgrade(id, 1)).to.revertedWith(
      "Craft: upgrade() not enough level.",
    );
  });

  it("should upgrade the recipe successfully", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.experience.assignExperience(id, 50000);
    expect(await this.items.balanceOf(this.owner.address, 2)).to.eq(0);
    expect(await this.items.balanceOf(this.owner.address, 1)).to.eq(1);
    await this.craft.upgrade(id, 1);
    expect(await this.items.balanceOf(this.owner.address, 2)).to.eq(1);
    expect(await this.items.balanceOf(this.owner.address, 1)).to.eq(0);
  });

  it("should fail when trying to upgrade an invalid recipe ", async () => {
    await expect(
      this.craft.updateRecipe({
        id: 5,
        name: "Test",
        description: "test 2",
        materials: [this.resource.address],
        material_amounts: [10],
        stats_required: { might: 1, speed: 1, intellect: 1 },
        cooldown: 3600,
        level_required: 3,
        gold_cost: 100,
        reward: 1,
        experience_reward: 1,
        available: true,
      }),
    ).to.revertedWith("Craft: updateRecipe() invalid recipe id.");
  });

  it("should fail when trying to upgrade an invalid upgrade ", async () => {
    await expect(
      this.craft.updateUpgrade({
        id: 5,
        name: "Test",
        description: "test 2",
        materials: [this.resource.address],
        material_amounts: [10],
        stats_required: { might: 1, speed: 1, intellect: 1 },
        stats_sacrificed: { might: 1, speed: 1, intellect: 1 },
        level_required: 3,
        gold_cost: 100,
        upgraded_item: 1,
        reward: 1,
        available: true,
      }),
    ).to.revertedWith("Craft: updateUpgrade() invalid upgrade id.");
  });

  it("should update a recipe correctly", async () => {
    let recipe = await this.craft.getRecipe(1);
    expect(recipe.stats_required.might).to.eq(1);
    await this.craft.updateRecipe({
      id: 1,
      name: "Test",
      description: "test 2",
      materials: [this.resource.address],
      material_amounts: [10],
      stats_required: { might: 5, speed: 1, intellect: 1 },
      cooldown: 3600,
      level_required: 3,
      gold_cost: 100,
      reward: 1,
      experience_reward: 1,
      available: true,
    });
    recipe = await this.craft.getRecipe(1);
    expect(recipe.stats_required.might).to.eq(5);
  });

  it("should update an upgrade correctly", async () => {
    let upgrade = await this.craft.getUpgrade(1);
    expect(upgrade.stats_required.might).to.eq(1);
    await this.craft.updateUpgrade({
      id: 1,
      name: "Test",
      description: "test 2",
      materials: [this.resource.address],
      material_amounts: [10],
      stats_required: { might: 5, speed: 1, intellect: 1 },
      stats_sacrificed: { might: 1, speed: 1, intellect: 1 },
      level_required: 3,
      gold_cost: 100,
      upgraded_item: 1,
      reward: 1,
      available: true,
    });
    upgrade = await this.craft.getUpgrade(1);
    expect(upgrade.stats_required.might).to.eq(5);
  });
});
