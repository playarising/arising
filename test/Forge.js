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
});
