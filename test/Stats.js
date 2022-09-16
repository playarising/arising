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
    const [owner, receiver, allowed] = await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;
    this.allowed = allowed;

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("10"));
    await this.mock.deployed();

    const Refresher = await ethers.getContractFactory("Refresher");
    this.refresher = await Refresher.deploy(
      this.mock.address,
      ethers.utils.parseEther("1")
    );
    await this.refresher.deployed();

    const Vitalizer = await ethers.getContractFactory("Vitalizer");
    this.vitalizer = await Vitalizer.deploy(
      this.mock.address,
      ethers.utils.parseEther("1")
    );
    await this.vitalizer.deployed();

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
    await this.civ.setInitialized(true);

    await this.ard.transferOwnership(this.civ.address);
    await this.civ.mint(this.ard.address);
    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();

    const Stats = await ethers.getContractFactory("Stats");
    this.stats = await Stats.deploy(this.civ.address, this.experience.address);
    await this.stats.deployed();

    await this.stats.setRefreshToken(this.refresher.address);
    await this.stats.setVitalizerToken(this.vitalizer.address);
  });

  it("should deploy everything correctly", async () => {
    expect(await this.stats.owner()).to.eq(this.owner.address);
  });

  it("should return base and pool stats correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    const pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(0);
    expect(pool.speed).to.eq(0);
    expect(pool.intellect).to.eq(0);
    const base = await this.stats.getBaseStats(id);
    expect(base.might).to.eq(0);
    expect(base.speed).to.eq(0);
    expect(base.intellect).to.eq(0);
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

  it("should change the refresher token", async () => {
    expect(await this.stats.refresher()).to.eq(this.refresher.address);
    await this.stats.setRefreshToken(this.mock.address);
    expect(await this.stats.refresher()).to.eq(this.mock.address);
    await this.stats.setRefreshToken(this.refresher.address);
  });

  it("should change the vitalizer token", async () => {
    expect(await this.stats.vitalizer()).to.eq(this.vitalizer.address);
    await this.stats.setVitalizerToken(this.mock.address);
    expect(await this.stats.vitalizer()).to.eq(this.mock.address);
    await this.stats.setVitalizerToken(this.vitalizer.address);
  });

  it("should fail change the refresher token from non owner", async () => {
    await expect(
      this.stats.connect(this.receiver).setRefreshToken(this.mock.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail change the vitalizer token from non owner", async () => {
    await expect(
      this.stats.connect(this.receiver).setVitalizerToken(this.mock.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail assign points from non allowed address", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(
      this.stats.connect(this.receiver).assignPoints(id, 2, 2, 2)
    ).to.revertedWith("Stats: msg.sender is not allowed to access this token.");
  });

  it("should fail assign more points than allowed", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.stats.assignPoints(id, 52, 52, 53)).to.revertedWith(
      "Stats: can't assign more points than available."
    );
  });

  it("should fail assign points to not minted token", async () => {
    await expect(
      this.stats.assignPoints(
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001",
        52,
        52,
        53
      )
    ).to.revertedWith("Civilizations: id of the civilization is not valid.");
    await expect(
      this.stats.assignPoints(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
        52,
        52,
        53
      )
    ).to.revertedWith("Stats: can't get access to a non minted token.");
  });

  it("should assign points correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    expect(await this.stats.getAvailablePoints(id)).to.eq(156);
    await this.stats.assignPoints(id, 52, 52, 52);
    const pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(52);
    expect(pool.speed).to.eq(52);
    expect(pool.intellect).to.eq(52);
    const base = await this.stats.getBaseStats(id);
    expect(base.might).to.eq(52);
    expect(base.speed).to.eq(52);
    expect(base.intellect).to.eq(52);
    expect(await this.stats.getAvailablePoints(id)).to.eq(0);
  });

  it("should fail to sacrifice from non allowed", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(
      this.stats.connect(this.receiver).sacrifice(id, 1, 1, 1)
    ).revertedWith("Stats: msg.sender is not allowed to access this token.");
  });

  it("should fail to perform a free refresh from non allowed", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.stats.connect(this.receiver).refresh(id)).to.revertedWith(
      "Stats: msg.sender is not allowed to access this token."
    );
  });

  it("should be able to sacrifice base stats with allowance and approval", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.stats.sacrifice(id, 1, 1, 1);
    let base = await this.stats.getBaseStats(id);
    let pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(51);
    expect(pool.speed).to.eq(51);
    expect(pool.intellect).to.eq(51);
    await this.ard.approve(this.receiver.address, 1);
    await this.stats.connect(this.receiver).sacrifice(id, 1, 1, 1);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(50);
    expect(pool.speed).to.eq(50);
    expect(pool.intellect).to.eq(50);
    await this.ard.setApprovalForAll(this.allowed.address, true);
    await this.stats.connect(this.allowed).sacrifice(id, 1, 1, 1);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(49);
    expect(base.speed).to.eq(49);
    expect(base.intellect).to.eq(49);
    expect(pool.might).to.eq(49);
    expect(pool.speed).to.eq(49);
    expect(pool.intellect).to.eq(49);
  });

  it("should fail consume when pool is empty", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.stats.consume(id, 49, 49, 49);
    await expect(this.stats.consume(id, 1, 0, 0)).revertedWith(
      "Stats: cannot consume more might than currently available"
    );
    await expect(this.stats.consume(id, 0, 1, 0)).revertedWith(
      "Stats: cannot consume more speed than currently available"
    );
    await expect(this.stats.consume(id, 0, 0, 1)).revertedWith(
      "Stats: cannot consume more intellect than currently available"
    );
  });

  it("should perform a free refresh correctly and prevent a second refresh instantly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.stats.refresh(id);
    const pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(49);
    expect(pool.speed).to.eq(49);
    expect(pool.intellect).to.eq(49);
    await expect(this.stats.refresh(id)).to.revertedWith(
      "Stats: not enough time has passed to refresh pool"
    );
  });

  it("should perform a free refresh when time is available", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    const nextTime = await this.stats.getNextRefreshTime(id);
    await this.stats.consume(id, 45, 45, 45);
    let pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(4);
    expect(pool.speed).to.eq(4);
    expect(pool.intellect).to.eq(4);
    await ethers.provider.send("evm_mine", [nextTime.toNumber()]);
    await this.stats.refresh(id);
    pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(49);
    expect(pool.speed).to.eq(49);
    expect(pool.intellect).to.eq(49);
  });

  it("should fail to sacrifice more points than available", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.stats.sacrifice(id, 50, 0, 0)).revertedWith(
      "Stats: cannot sacrifice more might than currently available"
    );
    await expect(this.stats.sacrifice(id, 0, 50, 0)).revertedWith(
      "Stats: cannot sacrifice more speed than currently available"
    );
    await expect(this.stats.sacrifice(id, 0, 0, 50)).revertedWith(
      "Stats: cannot sacrifice more intellect than currently available"
    );
  });

  it("should fail to use a refresh with token with no balance", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: not enough refresh tokens balance to perform a refresh."
    );
  });

  it("should fail to use a refresh with token with no allowance", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.refresher.mintFree(this.owner.address, 6);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: not enough refresh tokens allowance to perform a refresh."
    );
  });

  it("should be able to refresh using the refresher token", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    expect(await this.refresher.balanceOf(this.owner.address)).to.eq(6);
    await this.stats.consume(id, 45, 45, 45);
    let pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(4);
    expect(pool.speed).to.eq(4);
    expect(pool.intellect).to.eq(4);
    await expect(this.stats.refresh(id)).to.revertedWith(
      "Stats: not enough time has passed to refresh pool"
    );
    await this.refresher.approve(this.stats.address, 1000);
    await this.stats.refreshWithToken(id);
    pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(24);
    expect(pool.speed).to.eq(24);
    expect(pool.intellect).to.eq(24);
  });

  it("should be not be able to use more than one refresher", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: already used a refresher for this day."
    );
  });

  it("should be able to use a new refresh for the next day", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    let time = await this.stats.getNextRefreshWithTokenTime(id);
    await ethers.provider.send("evm_mine", [time.toNumber()]);
    await this.refresher.mintFree(this.owner.address, 5);
    await this.stats.refreshWithToken(id);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: already used a refresher for this day."
    );
  });

  it("should fail to use a vitalizer with no tokens", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.stats.consumeVitalizer(id, 1, 0, 0)).to.revertedWith(
      "Stats: not enough vitalizer tokens balance to perform a vitalize."
    );
  });

  it("should fail to use a vitalizer with no allowance", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.vitalizer.mintFree(this.owner.address, 10);
    await expect(this.stats.consumeVitalizer(id, 1, 0, 0)).to.revertedWith(
      "Stats: not enough vitalizer tokens allowance to perform a vitalize."
    );
  });

  it("should fail to use a vitalizer when trying to upgrade more than one stat or empty", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.vitalizer.mintFree(this.owner.address, 10);
    await this.vitalizer.approve(this.stats.address, 10);
    await expect(this.stats.consumeVitalizer(id, 1, 1, 0)).to.revertedWith(
      "Stats: vitalizer should increase one point for a single stat."
    );
    await expect(this.stats.consumeVitalizer(id, 0, 0, 0)).to.revertedWith(
      "Stats: vitalizer should increase one point for a single stat."
    );
  });

  it("should use vitalizer correctly", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.stats.consumeVitalizer(id, 1, 0, 0);
    await this.stats.consume(id, 20, 20, 20);
    let base = await this.stats.getBaseStats(id);
    let pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(49);
    expect(base.intellect).to.eq(49);
    expect(pool.might).to.eq(25);
    expect(pool.speed).to.eq(24);
    expect(pool.intellect).to.eq(24);
    await this.stats.consumeVitalizer(id, 0, 1, 0);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(49);
    expect(pool.might).to.eq(25);
    expect(pool.speed).to.eq(25);
    expect(pool.intellect).to.eq(24);
    await this.stats.consumeVitalizer(id, 0, 0, 1);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(25);
    expect(pool.speed).to.eq(25);
    expect(pool.intellect).to.eq(25);
    await this.stats.consumeVitalizer(id, 1, 0, 0);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(26);
    expect(pool.speed).to.eq(25);
    expect(pool.intellect).to.eq(25);
    await this.stats.consumeVitalizer(id, 0, 1, 0);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(26);
    expect(pool.speed).to.eq(26);
    expect(pool.intellect).to.eq(25);
    await this.stats.consumeVitalizer(id, 0, 0, 1);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(26);
    expect(pool.speed).to.eq(26);
    expect(pool.intellect).to.eq(26);
    await this.stats.consumeVitalizer(id, 1, 0, 0);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(52);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(27);
    expect(pool.speed).to.eq(26);
    expect(pool.intellect).to.eq(26);
    await this.stats.consumeVitalizer(id, 0, 1, 0);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(52);
    expect(base.speed).to.eq(52);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(27);
    expect(pool.speed).to.eq(27);
    expect(pool.intellect).to.eq(26);
    await this.stats.consumeVitalizer(id, 0, 0, 1);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(52);
    expect(base.speed).to.eq(52);
    expect(base.intellect).to.eq(52);
    expect(pool.might).to.eq(27);
    expect(pool.speed).to.eq(27);
    expect(pool.intellect).to.eq(27);
  });

  it("should fail to use a vitalizer without sacrificed points", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await expect(this.stats.consumeVitalizer(id, 0, 0, 1)).to.revertedWith(
      "Stats: user doesn't have sacrificed points to recover"
    );
  });

  it("should not change pool values when sacrifice and pool has less than max", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.stats.sacrifice(id, 1, 1, 1);
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(27);
    expect(pool.speed).to.eq(27);
    expect(pool.intellect).to.eq(27);
  });

  it("should be able to use a new refresh until pool is equal to base", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    let time = await this.stats.getNextRefreshWithTokenTime(id);
    await ethers.provider.send("evm_mine", [time.toNumber()]);
    await this.refresher.mintFree(this.owner.address, 5);
    await this.stats.refreshWithToken(id);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: already used a refresher for this day."
    );

    time = await this.stats.getNextRefreshWithTokenTime(id);
    await ethers.provider.send("evm_mine", [time.toNumber()]);
    await this.refresher.mintFree(this.owner.address, 5);
    await this.stats.refreshWithToken(id);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: already used a refresher for this day."
    );

    const base = await this.stats.getBaseStats(id);
    const pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(base.might);
    expect(pool.speed).to.eq(base.speed);
    expect(pool.intellect).to.eq(base.intellect);
  });
});
