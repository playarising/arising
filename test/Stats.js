const { expect } = require("chai");
const { ethers } = require("hardhat");

const next_level_experience = [
  1000, 1020, 1040, 1061, 1082, 1117, 1153, 1190, 1228, 1267, 1308, 1350, 1393,
  1438, 1488, 1536, 1585, 1636, 1688, 1742, 1777, 1813, 1849, 1886, 1924, 1962,
  2001, 2041, 2082, 2124, 2166, 2209, 2253, 2298, 2378, 2461, 2547, 2636, 2728,
  2823, 2913, 3006, 3102, 3201, 3303, 3369, 3436, 3505, 3575, 3647, 3764, 3884,
  4008, 4136, 4268, 4353, 4440, 4529, 4620, 4712, 4863, 5019, 5180, 5346, 5517,
  5694, 5876, 6064, 6258, 6458, 6684, 6918, 7160, 7411, 7670, 7938, 8216, 8504,
  8802, 9110, 9402, 9703, 10013, 10333, 10664, 10877, 11095, 11317, 11543,
  11774, 12151, 12540, 12941, 13355, 13782, 14223, 14678, 15148, 15633, 16133,
  16456, 16785, 17121, 17463, 17812, 18168, 18531, 18902, 19280, 19666, 20354,
  21066, 21803, 22566, 23356, 24173, 25019, 25895, 26801, 27739, 28627, 29543,
  30488, 31464, 32471, 33120, 33782, 34458, 35147, 35850, 36567, 37298, 38044,
  38805, 39581, 40848, 42155, 43504, 44896, 46467, 48093, 49776, 51518, 53321,
  55187, 57119, 59118, 61187, 63329, 65546, 100000,
];

describe("Stats", () => {
  before(async () => {
    const [owner, receiver, authority] = await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;
    this.authority = authority;

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("10"));
    await this.mock.deployed();

    const Refresher = await ethers.getContractFactory("Refresher");
    this.refresher = await Refresher.deploy(
      this.mock.address,
      ethers.utils.parseEther("1")
    );
    await this.refresher.deployed();

    const Levels = await ethers.getContractFactory("Levels");
    const levels = await Levels.deploy();
    await levels.deployed();

    const Ard = await ethers.getContractFactory("Ard");
    this.ard = await Ard.deploy();
    await this.ard.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy(
      ethers.utils.parseEther("1"),
      receiver.address
    );
    await this.civ.deployed();
    await this.civ.addCivilization(this.ard.address);
    await this.civ.setInitialized();

    await this.ard.transferOwnership(this.civ.address);
    await this.civ.mint(this.ard.address, {
      value: ethers.utils.parseEther("1"),
    });
    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();

    const Stats = await ethers.getContractFactory("Stats");
    this.stats = await Stats.deploy(this.civ.address, this.experience.address);
    await this.stats.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.stats.owner()).to.eq(this.owner.address);
  });

  it("should return base and pool stats correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    const pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(0);
    expect(pool.speed).to.eq(0);
    expect(pool.intelect).to.eq(0);
    const base = await this.stats.getBaseStats(id);
    expect(base.might).to.eq(0);
    expect(base.speed).to.eq(0);
    expect(base.intelect).to.eq(0);
  });

  it("should return the amount of available points correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    let available = 6;
    for (let i = 0; i < 150; i++) {
      expect(await this.experience.getLevel(id)).to.eq(i);
      await this.experience.assignExperience(id, next_level_experience[i]);
      available++;
      expect(await this.stats.getAvailablePoints(id)).to.eq(available);
    }
  });
});
