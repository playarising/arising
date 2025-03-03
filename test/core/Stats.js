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

    const Levels = await ethers.getContractFactory("Levels");
    const levels = await Levels.deploy();
    await levels.deployed();
    await levels.initialize();
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
    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy();
    await this.experience.deployed();
    await this.experience.initialize(this.civ.address, levels.address);
    const Items = await ethers.getContractFactory("Items");
    this.items = await Items.deploy();
    await this.items.deployed();
    await this.items.initialize;
    const Equipment = await ethers.getContractFactory("Equipment");
    this.equipment = await Equipment.deploy();
    await this.equipment.deployed();
    await this.equipment.initialize(
      this.civ.address,
      this.experience.address,
      this.items.address,
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
  });

  it("should deploy everything correctly", async () => {
    expect(await this.stats.owner()).to.eq(this.owner.address);
  });

  it("should change the refresh cooldown", async () => {
    expect(await this.stats.REFRESH_COOLDOWN_SECONDS()).to.eq(86400);
    await this.stats.setRefreshCooldown(1);
    expect(await this.stats.REFRESH_COOLDOWN_SECONDS()).to.eq(1);
    await this.stats.setRefreshCooldown(86400);
  });

  it("should fail when trying to set the refresh cooldown from non owner", async () => {
    await expect(
      this.stats.connect(this.receiver).setRefreshCooldown(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should not be able to assign points when paused", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.stats.paused()).to.eq(false);
    await this.stats.pause();
    expect(await this.stats.paused()).to.eq(true);
    await expect(
      this.stats.assignPoints(id, { might: 1, speed: 1, intellect: 1 }),
    ).to.revertedWith("Pausable: paused");
    await this.stats.unpause();
    expect(await this.stats.paused()).to.eq(false);
  });

  it("should not be able pause when not owner", async () => {
    await expect(this.stats.connect(this.receiver).pause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
    await expect(this.stats.connect(this.receiver).unpause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should return base and pool stats correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
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
    const id = await this.civ.getTokenID(1, 1);
    let available = 6;
    for (let i = 0; i < 150; i++) {
      expect(await this.experience.getLevel(id)).to.eq(i);
      await this.experience.assignExperience(id, next_level_experience[i]);
      available++;
      expect(await this.stats.getAvailablePoints(id)).to.eq(available);
    }
  });

  it("should fail assign points from non allowed address", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.stats
        .connect(this.receiver)
        .assignPoints(id, { might: 2, speed: 2, intellect: 2 }),
    ).to.revertedWith(
      "Stats: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail assign more points than allowed", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.stats.assignPoints(id, { might: 52, speed: 52, intellect: 53 }),
    ).to.revertedWith("Stats: assignPoints() too many points selected.");
  });

  it("should fail assign points to not minted token", async () => {
    await expect(
      this.stats.assignPoints(
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001",
        { might: 52, speed: 52, intellect: 53 },
      ),
    ).to.revertedWith("Civilizations: exists() invalid civilization id.");
    await expect(
      this.stats.assignPoints(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000005",
        { might: 52, speed: 52, intellect: 53 },
      ),
    ).to.revertedWith("Stats: onlyAllowed() token not minted.");
  });

  it("should assign points correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.stats.getAvailablePoints(id)).to.eq(156);
    await this.stats.assignPoints(id, { might: 52, speed: 52, intellect: 52 });
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
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.stats
        .connect(this.receiver)
        .sacrifice(id, { might: 1, speed: 1, intellect: 1 }),
    ).revertedWith(
      "Stats: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail to perform a free refresh from non allowed", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.stats.connect(this.receiver).refresh(id)).to.revertedWith(
      "Stats: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should be able to sacrifice base stats with allowance and approval", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.stats.sacrifice(id, { might: 1, speed: 1, intellect: 1 });
    let base = await this.stats.getBaseStats(id);
    let pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(51);
    expect(pool.speed).to.eq(51);
    expect(pool.intellect).to.eq(51);
    await this.collection.approve(this.receiver.address, 1);
    await this.stats
      .connect(this.receiver)
      .sacrifice(id, { might: 1, speed: 1, intellect: 1 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(50);
    expect(pool.speed).to.eq(50);
    expect(pool.intellect).to.eq(50);
    await this.collection.setApprovalForAll(this.allowed.address, true);
    await this.stats
      .connect(this.allowed)
      .sacrifice(id, { might: 1, speed: 1, intellect: 1 });
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
    const id = await this.civ.getTokenID(1, 1);
    await this.stats.consume(id, { might: 49, speed: 49, intellect: 49 });
    await expect(
      this.stats.consume(id, { might: 1, speed: 0, intellect: 0 }),
    ).revertedWith("Stats: consume() not enough might.");
    await expect(
      this.stats.consume(id, { might: 0, speed: 1, intellect: 0 }),
    ).revertedWith("Stats: consume() not enough speed.");
    await expect(
      this.stats.consume(id, { might: 0, speed: 0, intellect: 1 }),
    ).revertedWith("Stats: consume() not enough intellect.");
  });

  it("should perform a free refresh correctly and prevent a second refresh instantly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.stats.refresh(id))
      .to.emit(this.stats, "ChangedPoints")
      .withArgs(id, [49, 49, 49], [49, 49, 49]);
    const pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(49);
    expect(pool.speed).to.eq(49);
    expect(pool.intellect).to.eq(49);
    await expect(this.stats.refresh(id)).to.revertedWith(
      "Stats: refresh() not enough time has passed to refresh pool.",
    );
  });

  it("should perform a free refresh when time is available", async () => {
    const id = await this.civ.getTokenID(1, 1);
    const nextTime = await this.stats.getNextRefreshTime(id);
    await expect(
      this.stats.consume(id, { might: 45, speed: 45, intellect: 45 }),
    )
      .to.emit(this.stats, "ChangedPoints")
      .withArgs(id, [49, 49, 49], [4, 4, 4]);
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
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.stats.sacrifice(id, { might: 50, speed: 0, intellect: 0 }),
    ).revertedWith("Stats: sacrifice() not enough might.");
    await expect(
      this.stats.sacrifice(id, { might: 0, speed: 50, intellect: 0 }),
    ).revertedWith("Stats: sacrifice() not enough speed.");
    await expect(
      this.stats.sacrifice(id, { might: 0, speed: 0, intellect: 50 }),
    ).revertedWith("Stats: sacrifice() not enough intellect.");
  });

  it("should fail to use a refresh with token with no balance", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: refreshWithToken() not enough refresh tokens balance.",
    );
  });

  it("should fail to use a refresh with token with no allowance", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.refresher.mintFree(this.owner.address, 6);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: refreshWithToken() not enough refresh tokens allowance.",
    );
  });

  it("should be able to refresh using the refresher token", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.refresher.balanceOf(this.owner.address)).to.eq(6);
    await this.stats.consume(id, { might: 45, speed: 45, intellect: 45 });
    let pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(4);
    expect(pool.speed).to.eq(4);
    expect(pool.intellect).to.eq(4);
    await expect(this.stats.refresh(id)).to.revertedWith(
      "Stats: refresh() not enough time has passed to refresh pool.",
    );
    await this.refresher.approve(this.stats.address, 1000);
    await this.stats.refreshWithToken(id);
    pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(24);
    expect(pool.speed).to.eq(24);
    expect(pool.intellect).to.eq(24);
  });

  it("should be not be able to use more than one refresher for a day", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: refreshWithToken() no more refresh with tokens available.",
    );
  });

  it("should be able to use a new refresh for the next day", async () => {
    const id = await this.civ.getTokenID(1, 1);
    let time = await this.stats.getNextRefreshWithTokenTime(id);
    await ethers.provider.send("evm_mine", [time.toNumber()]);
    await this.refresher.mintFree(this.owner.address, 5);
    await this.stats.refreshWithToken(id);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: refreshWithToken() no more refresh with tokens available.",
    );
  });

  it("should fail to use a vitalizer with no tokens", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.stats.vitalize(id, { might: 1, speed: 0, intellect: 0 }),
    ).to.revertedWith("Stats: vitalize() not enough vitalizer tokens balance.");
  });

  it("should fail to use a vitalizer with no allowance", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.vitalizer.mintFree(this.owner.address, 10);
    await expect(
      this.stats.vitalize(id, { might: 1, speed: 0, intellect: 0 }),
    ).to.revertedWith(
      "Stats: vitalize() not enough vitalizer tokens allowance.",
    );
  });

  it("should fail to use a vitalizer when trying to upgrade more than one stat or empty", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.vitalizer.mintFree(this.owner.address, 10);
    await this.vitalizer.approve(this.stats.address, 10);
    await expect(
      this.stats.vitalize(id, { might: 1, speed: 1, intellect: 0 }),
    ).to.revertedWith("Stats: vitalize() too many points to recover.");
    await expect(
      this.stats.vitalize(id, { might: 0, speed: 0, intellect: 0 }),
    ).to.revertedWith("Stats: vitalize() too many points to recover.");
  });

  it("should use vitalizer correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.stats.vitalize(id, { might: 1, speed: 0, intellect: 0 });
    await this.stats.consume(id, { might: 20, speed: 20, intellect: 20 });
    let base = await this.stats.getBaseStats(id);
    let pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(49);
    expect(base.intellect).to.eq(49);
    expect(pool.might).to.eq(25);
    expect(pool.speed).to.eq(24);
    expect(pool.intellect).to.eq(24);
    await this.stats.vitalize(id, { might: 0, speed: 1, intellect: 0 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(49);
    expect(pool.might).to.eq(25);
    expect(pool.speed).to.eq(25);
    expect(pool.intellect).to.eq(24);
    await this.stats.vitalize(id, { might: 0, speed: 0, intellect: 1 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(50);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(25);
    expect(pool.speed).to.eq(25);
    expect(pool.intellect).to.eq(25);
    await this.stats.vitalize(id, { might: 1, speed: 0, intellect: 0 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(50);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(26);
    expect(pool.speed).to.eq(25);
    expect(pool.intellect).to.eq(25);
    await this.stats.vitalize(id, { might: 0, speed: 1, intellect: 0 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(50);
    expect(pool.might).to.eq(26);
    expect(pool.speed).to.eq(26);
    expect(pool.intellect).to.eq(25);
    await this.stats.vitalize(id, { might: 0, speed: 0, intellect: 1 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(51);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(26);
    expect(pool.speed).to.eq(26);
    expect(pool.intellect).to.eq(26);
    await this.stats.vitalize(id, { might: 1, speed: 0, intellect: 0 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(52);
    expect(base.speed).to.eq(51);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(27);
    expect(pool.speed).to.eq(26);
    expect(pool.intellect).to.eq(26);
    await this.stats.vitalize(id, { might: 0, speed: 1, intellect: 0 });
    base = await this.stats.getBaseStats(id);
    pool = await this.stats.getPoolStats(id);
    expect(base.might).to.eq(52);
    expect(base.speed).to.eq(52);
    expect(base.intellect).to.eq(51);
    expect(pool.might).to.eq(27);
    expect(pool.speed).to.eq(27);
    expect(pool.intellect).to.eq(26);
    await this.stats.vitalize(id, { might: 0, speed: 0, intellect: 1 });
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
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.stats.vitalize(id, { might: 0, speed: 0, intellect: 1 }),
    ).to.revertedWith("Stats: vitalize() not enough sacrificed points.");
  });

  it("should not change pool values when sacrifice and pool has less than max", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.stats.sacrifice(id, { might: 1, speed: 1, intellect: 1 });
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
    const id = await this.civ.getTokenID(1, 1);
    let time = await this.stats.getNextRefreshWithTokenTime(id);
    await ethers.provider.send("evm_mine", [time.toNumber()]);
    await this.refresher.mintFree(this.owner.address, 5);
    await this.stats.refreshWithToken(id);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: refreshWithToken() no more refresh with tokens available.",
    );

    time = await this.stats.getNextRefreshWithTokenTime(id);
    await ethers.provider.send("evm_mine", [time.toNumber()]);
    await this.refresher.mintFree(this.owner.address, 5);
    await this.stats.refreshWithToken(id);
    await expect(this.stats.refreshWithToken(id)).to.revertedWith(
      "Stats: refreshWithToken() no more refresh with tokens available.",
    );

    const base = await this.stats.getBaseStats(id);
    const pool = await this.stats.getPoolStats(id);
    expect(pool.might).to.eq(base.might);
    expect(pool.speed).to.eq(base.speed);
    expect(pool.intellect).to.eq(base.intellect);
  });
});
