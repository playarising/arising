const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Forge", () => {
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
    this.wood = await BaseFungibleItem.deploy();
    await this.wood.deployed();
    await this.wood.initialize("Ard: Wood", "WOOD", this.civ.address);
    this.wood_plank = await BaseFungibleItem.deploy();
    await this.wood_plank.deployed();
    await this.wood_plank.initialize(
      "Ard: Wood Plank",
      "WOOD_PLANK",
      this.civ.address,
    );
    const Forge = await ethers.getContractFactory("Forge");
    this.forge = await Forge.deploy();
    await this.forge.deployed();
    await this.forge.initialize(
      this.civ.address,
      this.experience.address,
      this.stats.address,
      this.mock.address,
      ethers.utils.parseEther("49.99"),
    );
    await this.wood_plank.addAuthority(this.forge.address);

    await this.experience.addAuthority(this.forge.address);
  });

  it("should set the upgrade price correctly", async () => {
    await this.forge.setUpgradePrice(ethers.utils.parseEther("19.99"));
    expect(await this.forge.price()).to.eq(ethers.utils.parseEther("19.99"));
  });

  it("should fail to set the mint price from non owner", async () => {
    await expect(
      this.forge.connect(this.receiver).setUpgradePrice(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should not be able to forge when paused", async () => {
    const id = await this.civ.getTokenID(1, 2);
    expect(await this.forge.paused()).to.eq(false);
    await this.forge.pause();
    expect(await this.forge.paused()).to.eq(true);
    await expect(this.forge.forge(id, 1, 1)).to.revertedWith(
      "Pausable: paused",
    );
    await this.forge.unpause();
    expect(await this.forge.paused()).to.eq(false);
  });

  it("should not be able pause when not owner", async () => {
    await expect(this.forge.connect(this.receiver).pause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
    await expect(this.forge.connect(this.receiver).unpause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should fail adding a recipe when no owner", async () => {
    await expect(
      this.forge
        .connect(this.receiver)
        .addRecipe(
          "test",
          "test 2",
          [],
          [],
          { might: 1, speed: 1, intellect: 1 },
          1,
          1,
          this.owner.address,
          1,
        ),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should add recipes correctly", async () => {
    await expect(
      this.forge.addRecipe(
        "test",
        "test 2",
        [this.wood.address],
        [1],
        { might: 0, speed: 1, intellect: 0 },
        300,
        2,
        this.wood_plank.address,
        25,
      ),
    )
      .to.emit(this.forge, "AddRecipe")
      .withArgs(1, "test", "test 2");
  });

  it("should fail adding a recipe when material and amounts are not the same", async () => {
    await expect(
      this.forge.addRecipe(
        "test",
        "test 2",
        [this.wood.address],
        [1, 2],
        { might: 1, speed: 1, intellect: 1 },
        1,
        1,
        this.owner.address,
        1,
      ),
    ).to.revertedWith("Forge: addRecipe() materials and amounts not match.");
  });

  it("should fail disabling a recibe when no owner", async () => {
    await expect(
      this.forge.connect(this.receiver).disableRecipe(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail enabling a recibe when no owner", async () => {
    await expect(
      this.forge.connect(this.receiver).enableRecipe(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail disabling a recibe when it doesn't exists", async () => {
    await expect(this.forge.disableRecipe(0)).to.revertedWith(
      "Forge: disableRecipe() invalid recipe id.",
    );
    await expect(this.forge.disableRecipe(2)).to.revertedWith(
      "Forge: disableRecipe() invalid recipe id.",
    );
  });

  it("should disable and enable a recipe correctly", async () => {
    let r = await this.forge.getRecipe(1);
    expect(r.available).to.eq(true);
    await expect(this.forge.disableRecipe(1))
      .to.emit(this.forge, "DisableRecipe")
      .withArgs(1);
    r = await this.forge.getRecipe(1);
    expect(r.available).to.eq(false);
    await expect(this.forge.enableRecipe(1))
      .to.emit(this.forge, "EnableRecipe")
      .withArgs(1);
    r = await this.forge.getRecipe(1);
    expect(r.available).to.eq(true);
  });

  it("should fail when trying to purchase an upgrade from a non existing character", async () => {
    await expect(
      this.forge
        .connect(this.receiver)
        .buyUpgrade(
          "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000005",
        ),
    ).to.revertedWith("Forge: onlyAllowed() token not minted.");
  });

  it("should fail when trying to purchase an upgrade from a non owner character", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.forge.connect(this.receiver).buyUpgrade(id),
    ).to.revertedWith(
      "Forge: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail when trying to purchase an upgrade with no tokens", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.collection.setApprovalForAll(this.receiver.address, true);
    await expect(
      this.forge.connect(this.receiver).buyUpgrade(id),
    ).to.revertedWith("Forge: buyUpgrade() not enough balance to buy upgrade.");
  });

  it("should fail when trying to purchase an upgrade with no allowance", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.forge.buyUpgrade(id)).to.revertedWith(
      "Forge: buyUpgrade() not enough allowance to buy upgrade.",
    );
  });

  it("should return the character initial forges correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    const forge1 = await this.forge.getCharacterForge(id, 1);
    expect(forge1.available).to.eq(false);
    expect(forge1.cooldown).to.eq(0);
    expect(forge1.last_recipe).to.eq(0);
    expect(forge1.last_recipe_claimed).to.eq(false);
    const forge2 = await this.forge.getCharacterForge(id, 2);
    expect(forge2.available).to.eq(false);
    expect(forge2.cooldown).to.eq(0);
    expect(forge2.last_recipe).to.eq(0);
    expect(forge2.last_recipe_claimed).to.eq(false);
    const forge3 = await this.forge.getCharacterForge(id, 3);
    expect(forge3.available).to.eq(false);
    expect(forge3.cooldown).to.eq(0);
    expect(forge3.last_recipe).to.eq(0);
    expect(forge3.last_recipe_claimed).to.eq(false);
  });

  it("should return initial upgrades correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    const upgrades = await this.forge.getCharacterForgesUpgrades(id);
    expect(upgrades.length).to.eq(3);
    expect(upgrades[0]).to.eq(true);
    expect(upgrades[1]).to.eq(false);
    expect(upgrades[2]).to.eq(false);
  });

  it("should fail trying to return an invalid forge", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.forge.getCharacterForge(id, 0)).to.revertedWith(
      "Forge: getCharacterForge() invalid forge id.",
    );
    await expect(this.forge.getCharacterForge(id, 4)).to.revertedWith(
      "Forge: getCharacterForge() invalid forge id.",
    );
  });

  it("should return initial availability correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    const availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability.length).to.eq(3);
    expect(availability[0]).to.eq(true);
    expect(availability[1]).to.eq(false);
    expect(availability[2]).to.eq(false);
  });

  it("should be able to purchase the second upgrade", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.mock.approve(
      this.forge.address,
      ethers.utils.parseEther("1000"),
    );
    await this.forge.buyUpgrade(id);

    const forge2 = await this.forge.getCharacterForge(id, 2);
    expect(forge2.available).to.eq(true);
    expect(forge2.cooldown).to.eq(0);
    expect(forge2.last_recipe).to.eq(0);
    expect(forge2.last_recipe_claimed).to.eq(false);

    const upgrades = await this.forge.getCharacterForgesUpgrades(id);
    expect(upgrades.length).to.eq(3);
    expect(upgrades[0]).to.eq(true);
    expect(upgrades[1]).to.eq(true);
    expect(upgrades[2]).to.eq(false);

    const availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability.length).to.eq(3);
    expect(availability[0]).to.eq(true);
    expect(availability[1]).to.eq(true);
    expect(availability[2]).to.eq(false);
  });

  it("should be able to purchase the third upgrade", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.forge.buyUpgrade(id);

    const forge3 = await this.forge.getCharacterForge(id, 3);
    expect(forge3.available).to.eq(true);
    expect(forge3.cooldown).to.eq(0);
    expect(forge3.last_recipe).to.eq(0);
    expect(forge3.last_recipe_claimed).to.eq(false);

    const upgrades = await this.forge.getCharacterForgesUpgrades(id);
    expect(upgrades.length).to.eq(3);
    expect(upgrades[0]).to.eq(true);
    expect(upgrades[1]).to.eq(true);
    expect(upgrades[2]).to.eq(true);

    const availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability.length).to.eq(3);
    expect(availability[0]).to.eq(true);
    expect(availability[1]).to.eq(true);
    expect(availability[2]).to.eq(true);
  });

  it("should fail trying to purchase a fourth upgrade", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.forge.buyUpgrade(id)).to.revertedWith(
      "Forge: buyUpgrade() no spot available.",
    );
  });

  it("should fail trying to claim from non purchased forge 2", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await expect(
      this.forge.connect(this.receiver).claim(id, 2),
    ).to.revertedWith("Forge: claim() forge is not available.");
  });

  it("should fail trying to forge from non purchased forge 2", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 2),
    ).to.revertedWith("Forge: forge() forge is not available.");
  });

  it("should fail trying to claim from non purchased forge 3", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await expect(
      this.forge.connect(this.receiver).claim(id, 3),
    ).to.revertedWith("Forge: claim() forge is not available.");
  });

  it("should fail trying to forge from non purchased forge 3", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 3),
    ).to.revertedWith("Forge: forge() forge is not available.");
  });

  it("should fail trying to forge a recipe that doesnt exist", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 0, 1),
    ).to.revertedWith("Forge: forge() invalid recipe id.");
    await expect(
      this.forge.connect(this.receiver).forge(id, 15, 1),
    ).to.revertedWith("Forge: forge() invalid recipe id.");
  });

  it("should fail trying to forge a recipe that is not available", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await this.forge.disableRecipe(1);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1),
    ).to.revertedWith("Forge: forge() recipe not available.");
    await this.forge.enableRecipe(1);
  });

  it("should fail trying to forge a recipe with no enough level", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1),
    ).to.revertedWith("Forge: forge() not enough level.");
  });

  it("should fail trying to forge a recipe with no enough gold", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await this.experience.assignExperience(id, 50000);
    await this.collection
      .connect(this.receiver)
      .setApprovalForAll(this.forge.address, true);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1),
    ).to.revertedWith("BaseFungibleItem: consume() not enough balance.");
  });

  it("should fail trying to forge a recipe with no enough materials", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await this.gold.mintTo(id, 500);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1),
    ).to.revertedWith("BaseFungibleItem: consume() not enough balance.");
  });

  it("should fail trying to forge a recipe with no enough stats", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await this.wood.mintTo(id, 1);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1),
    ).to.revertedWith("Stats: consume() not enough speed.");
  });

  it("should forge an item correctly", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await this.stats
      .connect(this.receiver)
      .assignPoints(id, { might: 0, speed: 2, intellect: 0 });
    await this.forge.connect(this.receiver).forge(id, 1, 1);
    const forge1 = await this.forge.getCharacterForge(id, 1);
    expect(forge1.available).to.eq(true);
    expect(forge1.last_recipe).to.eq(1);
    expect(forge1.last_recipe_claimed).to.eq(false);

    const availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability.length).to.eq(3);
    expect(availability[0]).to.eq(false);
    expect(availability[1]).to.eq(false);
    expect(availability[2]).to.eq(false);
  });

  it("should forge try to use the same forge already being used", async () => {
    const id = await this.civ.getTokenID(1, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1),
    ).to.revertedWith("Forge: forge() forge is already being used.");
  });

  it("should forge and fill all 3 forges", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.experience.assignExperience(id, 50000);
    await this.collection.setApprovalForAll(this.forge.address, true);
    await this.stats.assignPoints(id, { might: 0, speed: 10, intellect: 0 });
    await this.gold.mintTo(id, 6);
    await this.wood.mintTo(id, 3);

    expect(await this.gold.balanceOf(id)).to.eq(6);
    expect(await this.wood.balanceOf(id)).to.eq(3);

    let availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability[0]).to.eq(true);
    expect(availability[1]).to.eq(true);
    expect(availability[2]).to.eq(true);

    let forge1 = await this.forge.getCharacterForge(id, 1);
    expect(forge1.available).to.eq(false);
    expect(forge1.last_recipe).to.eq(0);
    expect(forge1.last_recipe_claimed).to.eq(false);

    await this.forge.forge(id, 1, 1);
    availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability[0]).to.eq(false);
    expect(availability[1]).to.eq(true);
    expect(availability[2]).to.eq(true);

    forge1 = await this.forge.getCharacterForge(id, 1);
    expect(forge1.available).to.eq(true);
    expect(forge1.last_recipe).to.eq(1);
    expect(forge1.last_recipe_claimed).to.eq(false);

    let forge2 = await this.forge.getCharacterForge(id, 2);
    expect(forge2.available).to.eq(true);
    expect(forge2.last_recipe).to.eq(0);
    expect(forge2.last_recipe_claimed).to.eq(false);

    await this.forge.forge(id, 1, 2);
    availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability[0]).to.eq(false);
    expect(availability[1]).to.eq(false);
    expect(availability[2]).to.eq(true);

    forge2 = await this.forge.getCharacterForge(id, 2);
    expect(forge2.available).to.eq(true);
    expect(forge2.last_recipe).to.eq(1);
    expect(forge2.last_recipe_claimed).to.eq(false);

    let forge3 = await this.forge.getCharacterForge(id, 3);
    expect(forge3.available).to.eq(true);
    expect(forge3.last_recipe).to.eq(0);
    expect(forge3.last_recipe_claimed).to.eq(false);

    await this.forge.forge(id, 1, 3);
    availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability[0]).to.eq(false);
    expect(availability[1]).to.eq(false);
    expect(availability[2]).to.eq(false);

    forge3 = await this.forge.getCharacterForge(id, 3);
    expect(forge3.available).to.eq(true);
    expect(forge3.last_recipe).to.eq(1);
    expect(forge3.last_recipe_claimed).to.eq(false);

    expect(await this.wood.balanceOf(id)).to.eq(0);
  });

  it("should fail trying to claim forges before cooldown", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.forge.claim(id, 1)).to.revertedWith(
      "Forge: claim() forge not claimable.",
    );
    await expect(this.forge.claim(id, 2)).to.revertedWith(
      "Forge: claim() forge not claimable.",
    );
    await expect(this.forge.claim(id, 3)).to.revertedWith(
      "Forge: claim() forge not claimable.",
    );
  });

  it("should claim correctly all forges", async () => {
    const id = await this.civ.getTokenID(1, 1);
    const forge3 = await this.forge.getCharacterForge(id, 3);
    await ethers.provider.send("evm_mine", [forge3.cooldown.toNumber()]);
    let exp = await this.experience.getExperience(id);
    expect(exp).to.eq(50000);

    await this.forge.claim(id, 1);
    await this.forge.claim(id, 2);
    await this.forge.claim(id, 3);

    exp = await this.experience.getExperience(id);
    expect(exp).to.eq(50075);
    let availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability[0]).to.eq(true);
    expect(availability[1]).to.eq(true);
    expect(availability[2]).to.eq(true);

    expect(await this.wood_plank.balanceOf(id)).to.eq(3);
  });

  it("should fail when trying to claim after claimed", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.forge.claim(id, 1)).to.revertedWith(
      "Forge: claim() forge not claimable.",
    );
    await expect(this.forge.claim(id, 2)).to.revertedWith(
      "Forge: claim() forge not claimable.",
    );
    await expect(this.forge.claim(id, 3)).to.revertedWith(
      "Forge: claim() forge not claimable.",
    );
  });

  it("should be able to forge and claim again", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.gold.mintTo(id, 6);
    await this.wood.mintTo(id, 3);

    await this.forge.forge(id, 1, 1);
    await this.forge.forge(id, 1, 2);
    await this.forge.forge(id, 1, 3);

    const forge3 = await this.forge.getCharacterForge(id, 3);
    await ethers.provider.send("evm_mine", [forge3.cooldown.toNumber()]);

    await this.forge.claim(id, 1);
    await this.forge.claim(id, 2);
    await this.forge.claim(id, 3);

    expect(await this.experience.getExperience(id)).to.eq(50150);
    expect(await this.wood_plank.balanceOf(id)).to.eq(6);
  });

  it("should fail when trying to update recipe that doesn't exist", async () => {
    await expect(
      this.forge.updateRecipe({
        id: 4,
        name: "test",
        description: "test 2",
        materials: [this.wood.address],
        amounts: [1],
        stats_required: { might: 0, speed: 1, intellect: 0 },
        cooldown: 300,
        level_required: 5,
        gold_cost: 2,
        reward: this.wood_plank.address,
        experience_reward: 25,
        available: true,
      }),
    ).to.revertedWith("Forge: updateRecipe() invalid recipe id.");
  });

  it("should update a recipe correctly", async () => {
    let recipe = await this.forge.getRecipe(1);
    expect(recipe.stats_required.might).to.eq(0);
    this.forge.updateRecipe({
      id: 1,
      name: "test",
      description: "test 2",
      materials: [this.wood.address],
      amounts: [1],
      stats_required: { might: 0, speed: 1, intellect: 0 },
      cooldown: 300,
      level_required: 5,
      gold_cost: 2,
      reward: this.wood_plank.address,
      experience_reward: 25,
      available: true,
    });
    recipe = await this.forge.getRecipe(1);
    expect(recipe.stats_required.might).to.eq(0);
  });
});
