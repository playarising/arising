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

    const Ard = await ethers.getContractFactory("Ard");
    this.ard = await Ard.deploy();
    await this.ard.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy(receiver.address);
    await this.civ.deployed();
    await this.civ.addCivilization(this.ard.address);
    await this.civ.setInitialized();

    await this.ard.transferOwnership(this.civ.address);
    await this.civ.mint(this.ard.address);
    await this.civ.connect(this.receiver).mint(this.ard.address);

    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();

    const Stats = await ethers.getContractFactory("Stats");
    this.stats = await Stats.deploy(this.civ.address, this.experience.address);
    await this.stats.deployed();

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("1000"));
    await this.mock.deployed();

    const Gold = await ethers.getContractFactory("Gold");
    this.gold = await Gold.deploy(this.civ.address);
    await this.gold.deployed();

    const Forge = await ethers.getContractFactory("Forge");
    this.forge = await Forge.deploy(
      this.civ.address,
      this.experience.address,
      this.stats.address,
      this.gold.address,
      this.mock.address,
      ethers.utils.parseEther("49.99")
    );
    await this.forge.deployed();

    await this.experience.addAuthority(this.forge.address);

    const Adamantine = await ethers.getContractFactory("Adamantine");
    this.adamantine = await Adamantine.deploy(this.civ.address);
    await this.adamantine.deployed();

    const Bronze = await ethers.getContractFactory("Bronze");
    this.bronze = await Bronze.deploy(this.civ.address);
    await this.bronze.deployed();

    const Coal = await ethers.getContractFactory("Coal");
    this.coal = await Coal.deploy(this.civ.address);
    await this.coal.deployed();

    const Cobalt = await ethers.getContractFactory("Cobalt");
    this.cobalt = await Cobalt.deploy(this.civ.address);
    await this.cobalt.deployed();

    const Cotton = await ethers.getContractFactory("Cotton");
    this.cotton = await Cotton.deploy(this.civ.address);
    await this.cotton.deployed();

    const Iron = await ethers.getContractFactory("Iron");
    this.iron = await Iron.deploy(this.civ.address);
    await this.iron.deployed();

    const Leather = await ethers.getContractFactory("Leather");
    this.leather = await Leather.deploy(this.civ.address);
    await this.leather.deployed();

    const Platinum = await ethers.getContractFactory("Platinum");
    this.platinum = await Platinum.deploy(this.civ.address);
    await this.platinum.deployed();

    const Silk = await ethers.getContractFactory("Silk");
    this.silk = await Silk.deploy(this.civ.address);
    await this.silk.deployed();

    const Silver = await ethers.getContractFactory("Silver");
    this.silver = await Silver.deploy(this.civ.address);
    await this.silver.deployed();

    const Stone = await ethers.getContractFactory("Stone");
    this.stone = await Stone.deploy(this.civ.address);
    await this.stone.deployed();

    const Wood = await ethers.getContractFactory("Wood");
    this.wood = await Wood.deploy(this.civ.address);
    await this.wood.deployed();

    const Wool = await ethers.getContractFactory("Wool");
    this.wool = await Wool.deploy(this.civ.address);
    await this.wool.deployed();

    const AdamantineBar = await ethers.getContractFactory("AdamantineBar");
    this.adamantine_bar = await AdamantineBar.deploy(this.civ.address);
    await this.adamantine_bar.deployed();

    const BronzeBar = await ethers.getContractFactory("BronzeBar");
    this.bronze_bar = await BronzeBar.deploy(this.civ.address);
    await this.bronze_bar.deployed();

    const CobaltBar = await ethers.getContractFactory("CobaltBar");
    this.cobalt_bar = await CobaltBar.deploy(this.civ.address);
    await this.cobalt_bar.deployed();

    const CottonFabric = await ethers.getContractFactory("CottonFabric");
    this.cotton_fabric = await CottonFabric.deploy(this.civ.address);
    await this.cotton_fabric.deployed();

    const GoldBar = await ethers.getContractFactory("GoldBar");
    this.gold_bar = await GoldBar.deploy(this.civ.address);
    await this.gold_bar.deployed();

    const HardenedLeather = await ethers.getContractFactory("HardenedLeather");
    this.hardened_leather = await HardenedLeather.deploy(this.civ.address);
    await this.hardened_leather.deployed();

    const IronBar = await ethers.getContractFactory("IronBar");
    this.iron_bar = await IronBar.deploy(this.civ.address);
    await this.iron_bar.deployed();

    const Ironstone = await ethers.getContractFactory("Ironstone");
    this.ironstone = await Ironstone.deploy(this.civ.address);
    await this.ironstone.deployed();

    const PlatinumBar = await ethers.getContractFactory("PlatinumBar");
    this.platinum_bar = await PlatinumBar.deploy(this.civ.address);
    await this.platinum_bar.deployed();

    const SilkFabric = await ethers.getContractFactory("SilkFabric");
    this.silk_fabric = await SilkFabric.deploy(this.civ.address);
    await this.silk_fabric.deployed();

    const SilverBar = await ethers.getContractFactory("SilverBar");
    this.silver_bar = await SilverBar.deploy(this.civ.address);
    await this.silver_bar.deployed();

    const SteelBar = await ethers.getContractFactory("SteelBar");
    this.steel_bar = await SteelBar.deploy(this.civ.address);
    await this.steel_bar.deployed();

    const WoodPlank = await ethers.getContractFactory("WoodPlank");
    this.wood_plank = await WoodPlank.deploy(this.civ.address);
    await this.wood_plank.deployed();
    await this.wood_plank.transferOwnership(this.forge.address);

    const WoolFabric = await ethers.getContractFactory("WoolFabric");
    this.wool_fabric = await WoolFabric.deploy(this.civ.address);
    await this.wool_fabric.deployed();
  });

  it("should fail adding a recipe when no owner", async () => {
    await expect(
      this.forge
        .connect(this.receiver)
        .addRecipe([], [], 1, 1, 1, 1, 1, 1, 1, this.owner.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should add recipes correctly", async () => {
    await this.forge.addRecipe(
      [this.wood.address],
      [1],
      0,
      1,
      0,
      300,
      5,
      2,
      25,
      this.wood_plank.address
    );
    await this.forge.addRecipe(
      [this.bronze.address],
      [10],
      5,
      3,
      0,
      10800,
      10,
      5,
      100,
      this.bronze_bar.address
    );
    await this.forge.addRecipe(
      [this.stone.address, this.iron.address],
      [10, 1],
      8,
      2,
      3,
      12600,
      15,
      25,
      150,
      this.ironstone.address
    );
    await this.forge.addRecipe(
      [this.leather.address],
      [1],
      2,
      1,
      2,
      1800,
      15,
      3,
      50,
      this.hardened_leather.address
    );
    await this.forge.addRecipe(
      [this.cotton.address],
      [20],
      3,
      5,
      7,
      6000,
      15,
      10,
      75,
      this.cotton_fabric.address
    );
    await this.forge.addRecipe(
      [this.silk.address],
      [20],
      3,
      5,
      7,
      6000,
      15,
      10,
      75,
      this.silk_fabric.address
    );
    await this.forge.addRecipe(
      [this.wool.address],
      [20],
      3,
      5,
      7,
      6000,
      15,
      10,
      75,
      this.wool_fabric.address
    );
    await this.forge.addRecipe(
      [this.iron.address],
      [10],
      10,
      6,
      3,
      21600,
      20,
      50,
      150,
      this.iron_bar.address
    );
    await this.forge.addRecipe(
      [this.silver.address],
      [10],
      12,
      8,
      3,
      28800,
      25,
      0,
      200,
      this.silver_bar.address
    );
    await this.forge.addRecipe(
      [this.gold.address],
      [10],
      12,
      8,
      3,
      28800,
      25,
      0,
      200,
      this.gold_bar.address
    );
    await this.forge.addRecipe(
      [this.iron_bar.address, this.coal.address],
      [1, 10],
      15,
      10,
      3,
      36000,
      30,
      100,
      250,
      this.steel_bar.address
    );
    await this.forge.addRecipe(
      [this.cobalt.address, this.coal.address],
      [10, 20],
      17,
      12,
      5,
      43200,
      40,
      150,
      300,
      this.cobalt_bar.address
    );
    await this.forge.addRecipe(
      [this.platinum.address, this.coal.address],
      [10, 20],
      23,
      15,
      8,
      54000,
      50,
      175,
      350,
      this.platinum_bar.address
    );
    await this.forge.addRecipe(
      [this.adamantine.address, this.coal.address],
      [10, 20],
      28,
      17,
      11,
      61200,
      60,
      200,
      400,
      this.adamantine_bar.address
    );
  });

  it("should fail adding a recipe when material and amounts are not the same", async () => {
    await expect(
      this.forge.addRecipe(
        [this.wood.address],
        [1, 2],
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        this.owner.address
      )
    ).to.revertedWith(
      "Forge: materials and amounts arrays should be the same length"
    );
  });

  it("should fail disabling a recibe when no owner", async () => {
    await expect(
      this.forge.connect(this.receiver).disableRecipe(1)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail enabling a recibe when no owner", async () => {
    await expect(
      this.forge.connect(this.receiver).enableRecipe(1)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail disabling a recibe when it doesn't exists", async () => {
    await expect(this.forge.disableRecipe(0)).to.revertedWith(
      "Forge: recipe id doesn't exist."
    );
    await expect(this.forge.disableRecipe(15)).to.revertedWith(
      "Forge: recipe id doesn't exist."
    );
  });

  it("should disable and enable a recipe correctly", async () => {
    let r = await this.forge.getRecipe(1);
    expect(r.available).to.eq(true);
    await this.forge.disableRecipe(1);
    r = await this.forge.getRecipe(1);
    expect(r.available).to.eq(false);
    await this.forge.enableRecipe(1);
    r = await this.forge.getRecipe(1);
    expect(r.available).to.eq(true);
  });

  it("should fail when trying to purchase an upgrade from a non existing character", async () => {
    await expect(
      this.forge
        .connect(this.receiver)
        .buyUpgrade(
          "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000005"
        )
    ).to.revertedWith("Forge: can't get access to a non minted token.");
  });

  it("should fail when trying to purchase an upgrade from a non owner character", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(
      this.forge.connect(this.receiver).buyUpgrade(id)
    ).to.revertedWith("Forge: msg.sender is not allowed to access this token.");
  });

  it("should fail when trying to purchase an upgrade with no tokens", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.ard.setApprovalForAll(this.receiver.address, true);
    await expect(
      this.forge.connect(this.receiver).buyUpgrade(id)
    ).to.revertedWith(
      "Forge: not enough balance of payment tokens to mint tokens."
    );
  });

  it("should fail when trying to purchase an upgrade with no allowance", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.forge.buyUpgrade(id)).to.revertedWith(
      "Forge: not enough allowance to mint tokens."
    );
  });

  it("should return the character initial forges correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
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
    const id = await this.civ.getTokenID(this.ard.address, 1);
    const upgrades = await this.forge.getCharacterForgesUpgrades(id);
    expect(upgrades.length).to.eq(3);
    expect(upgrades[0]).to.eq(true);
    expect(upgrades[1]).to.eq(false);
    expect(upgrades[2]).to.eq(false);
  });

  it("should fail trying to return an invalid forge", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.forge.getCharacterForge(id, 0)).to.revertedWith(
      "Forge: selected forge is invalid"
    );
    await expect(this.forge.getCharacterForge(id, 4)).to.revertedWith(
      "Forge: selected forge is invalid"
    );
  });

  it("should return initial availability correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    const availability = await this.forge.getCharacterForgesAvailability(id);
    expect(availability.length).to.eq(3);
    expect(availability[0]).to.eq(true);
    expect(availability[1]).to.eq(false);
    expect(availability[2]).to.eq(false);
  });

  it("should be able to purchase the second upgrade", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.mock.approve(
      this.forge.address,
      ethers.utils.parseEther("1000")
    );
    await this.forge.buyUpgrade(id);
    expect(await this.mock.balanceOf(this.forge.address)).to.eq(
      ethers.utils.parseEther("49.99")
    );

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
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.forge.buyUpgrade(id);
    expect(await this.mock.balanceOf(this.forge.address)).to.eq(
      ethers.utils.parseEther("99.98")
    );

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
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.forge.buyUpgrade(id)).to.revertedWith(
      "Forge: user doesn't have buyable spots"
    );
  });

  it("should fail when trying to withdraw tokens from non owner", async () => {
    await expect(this.forge.connect(this.receiver).withdraw()).to.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should withdraw tokens correctly", async () => {
    let balance = await this.mock.balanceOf(this.owner.address);
    await this.forge.withdraw();
    balance = await this.mock.balanceOf(this.owner.address);
    expect(balance).to.eq(ethers.utils.parseEther("1000"));
  });

  it("should fail trying to claim from non purchased forge 2", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.forge.connect(this.receiver).claim(id, 2)
    ).to.revertedWith("Forge: forge 2 is not upgraded");
  });

  it("should fail trying to forge from non purchased forge 2", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 2)
    ).to.revertedWith("Forge: forge 2 is not upgraded");
  });

  it("should fail trying to claim from non purchased forge 3", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.forge.connect(this.receiver).claim(id, 3)
    ).to.revertedWith("Forge: forge 3 is not upgraded");
  });

  it("should fail trying to forge from non purchased forge 3", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 3)
    ).to.revertedWith("Forge: forge 3 is not upgraded");
  });

  it("should fail trying to forge a recipe that doesnt exist", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 0, 1)
    ).to.revertedWith("Forge: recipe id doesn't exist.");
    await expect(
      this.forge.connect(this.receiver).forge(id, 15, 1)
    ).to.revertedWith("Forge: recipe id doesn't exist.");
  });

  it("should fail trying to forge a recipe that is not available", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await this.forge.disableRecipe(1);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1)
    ).to.revertedWith(
      "Forge: the recipe trying to forge is not available at the moment."
    );
    await this.forge.enableRecipe(1);
  });

  it("should fail trying to forge a recipe with no enough level", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1)
    ).to.revertedWith(
      "Forge: the character doesn't have the level required to forge the material."
    );
  });

  it("should fail trying to forge a recipe with no enough gold", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await this.experience.assignExperience(id, 50000);
    await this.ard
      .connect(this.receiver)
      .setApprovalForAll(this.forge.address, true);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1)
    ).to.revertedWith("BaseFungibleItem: not enough balance to consume");
  });

  it("should fail trying to forge a recipe with no enough materials", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await this.gold.mintTo(id, 500);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1)
    ).to.revertedWith("BaseFungibleItem: not enough balance to consume");
  });

  it("should fail trying to forge a recipe with no enough stats", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await this.wood.mintTo(id, 1);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1)
    ).to.revertedWith(
      "Stats: cannot consume more speed than currently available."
    );
  });

  it("should forge an item correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await this.stats.connect(this.receiver).assignPoints(id, 0, 2, 0);
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
    const id = await this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.forge.connect(this.receiver).forge(id, 1, 1)
    ).to.revertedWith(
      "Forge: the forge trying to use is not available for use."
    );
  });

  it("should forge and fill all 3 forges", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.experience.assignExperience(id, 50000);
    await this.ard.setApprovalForAll(this.forge.address, true);
    await this.stats.assignPoints(id, 0, 10, 0);
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

    expect(await this.gold.balanceOf(id)).to.eq(0);
    expect(await this.wood.balanceOf(id)).to.eq(0);
  });

  it("should fail trying to claim forges before cooldown", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.forge.claim(id, 1)).to.revertedWith(
      "Forge: the forge trying to use is not available for claim."
    );
    await expect(this.forge.claim(id, 2)).to.revertedWith(
      "Forge: the forge trying to use is not available for claim."
    );
    await expect(this.forge.claim(id, 3)).to.revertedWith(
      "Forge: the forge trying to use is not available for claim."
    );
  });

  it("should claim correctly all forges", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
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
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.forge.claim(id, 1)).to.revertedWith(
      "Forge: the forge trying to use is not available for claim."
    );
    await expect(this.forge.claim(id, 2)).to.revertedWith(
      "Forge: the forge trying to use is not available for claim."
    );
    await expect(this.forge.claim(id, 3)).to.revertedWith(
      "Forge: the forge trying to use is not available for claim."
    );
  });

  it("should be able to forge and claim again", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
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
});
