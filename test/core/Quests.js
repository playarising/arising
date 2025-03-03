const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Quests", () => {
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

    this.resource = await BaseFungibleItem.deploy();
    await this.resource.deployed();
    await this.resource.initialize("Ard: Resource", "WOOD", this.civ.address);
    const Quests = await ethers.getContractFactory("Quests");
    this.quests = await Quests.deploy();

    await this.quests.deployed();

    await this.quests.initialize(
      this.civ.address,
      this.experience.address,
      this.stats.address,
    );
    await this.resource.addAuthority(this.quests.address);
    await this.experience.addAuthority(this.quests.address);
  });

  it("should not be able to start a quest when paused", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.quests.paused()).to.eq(false);
    await this.quests.pause();
    expect(await this.quests.paused()).to.eq(true);
    await expect(
      this.quests.startQuest(id, 1, { might: 1, speed: 1, intellect: 1 }),
    ).to.revertedWith("Pausable: paused");
    await this.quests.unpause();
    expect(await this.quests.paused()).to.eq(false);
  });

  it("should not be able pause when not owner", async () => {
    await expect(this.quests.connect(this.receiver).pause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
    await expect(this.quests.connect(this.receiver).unpause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should fail disabling a quest when no owner", async () => {
    await expect(
      this.quests.connect(this.receiver).disableQuest(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail enabling a quest when no owner", async () => {
    await expect(
      this.quests.connect(this.receiver).enableQuest(1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail disabling a quest when it doesn't exists", async () => {
    await expect(this.quests.disableQuest(1)).to.revertedWith(
      "Quests: disableQuest() invalid quest id.",
    );
  });

  it("should fail enabling a quest when it doesn't exists", async () => {
    await expect(this.quests.enableQuest(1)).to.revertedWith(
      "Quests: enableQuest() invalid quest id.",
    );
  });

  it("should fail adding a quest when no owner", async () => {
    await expect(
      this.quests
        .connect(this.receiver)
        .addQuest(
          "test",
          "test 2",
          0,
          [],
          [],
          1,
          { might: 1, speed: 1, intellect: 1 },
          1,
          1,
        ),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail adding a quest when materials and amount doesn't match", async () => {
    await expect(
      this.quests.addQuest(
        "test",
        "test 2",
        0,
        [this.resource.address],
        [],
        1,
        { might: 1, speed: 1, intellect: 1 },
        1,
        1,
      ),
    ).to.revertedWith("Quest: addQuest() materials and amounts not match.");
  });

  it("should enable a quest correctly", async () => {
    await this.quests.addQuest(
      "test",
      "test 2",
      0,
      [this.resource.address],
      [10],
      100,
      { might: 7, speed: 8, intellect: 5 },
      3600,
      1,
    );

    const quest = await this.quests.getQuest(1);
    expect(quest.available).to.eq(true);
  });

  it("should enable and disable the quest correctly", async () => {
    let quest = await this.quests.getQuest(1);
    expect(quest.available).to.eq(true);
    await expect(this.quests.disableQuest(1))
      .to.emit(this.quests, "DisableQuest")
      .withArgs(1);
    quest = await this.quests.getQuest(1);
    expect(quest.available).to.eq(false);
    await expect(this.quests.enableQuest(1))
      .to.emit(this.quests, "EnableQuest")
      .withArgs(1);
    quest = await this.quests.getQuest(1);
    expect(quest.available).to.eq(true);
  });

  it("should fail when trying to start a quest for a non owned token", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.quests
        .connect(this.receiver)
        .startQuest(id, 1, { might: 5, speed: 5, intellect: 5 }),
    ).to.revertedWith(
      "Quests: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail when trying to start a quest for a non minted token", async () => {
    await expect(
      this.quests.startQuest(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000005",
        1,
        { might: 5, speed: 5, intellect: 5 },
      ),
    ).to.revertedWith("Quests: onlyAllowed() token not minted.");
  });

  it("should fail start an invalid quest", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.quests.startQuest(id, 2, { might: 1, speed: 1, intellect: 1 }),
    ).to.revertedWith("Quests: startQuest() invalid quest id.");
  });

  it("should fail to start a not available quest", async () => {
    await this.quests.disableQuest(1);
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.quests.startQuest(id, 1, { might: 1, speed: 1, intellect: 1 }),
    ).to.revertedWith("Quests: startQuest() quest is not available.");
    await this.quests.enableQuest(1);
  });

  it("should fail start a quest with not enough level", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.quests.startQuest(id, 1, { might: 1, speed: 1, intellect: 1 }),
    ).to.revertedWith("Quests: startQuest() not enough level.");
  });

  it("should start a quest correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.experience.assignExperience(id, 10000);
    await this.stats.assignPoints(id, { might: 5, speed: 5, intellect: 5 });
    await this.collection.setApprovalForAll(this.quests.address, true);
    await this.quests.startQuest(id, 1, { might: 3, speed: 2, intellect: 2 });
    const quest = await this.quests.getCharacterCurrentQuest(id);
    expect(quest.last_quest_id).to.eq(1);
    expect(quest.claimed_reward).to.eq(false);
  });

  it("should fail when trying to start a quest with the last quest not claimed", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.quests.startQuest(id, 1, { might: 1, speed: 1, intellect: 1 }),
    ).to.revertedWith("Quest: startQuest() not available for quest.");
  });

  it("should fail when trying to claim a quest that is not ready", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.quests.claimQuest(id)).to.revertedWith(
      "Quest: claimQuest() not available to claim.",
    );
  });

  it("should claim the quest successfully", async () => {
    const id = await this.civ.getTokenID(1, 1);
    let quest = await this.quests.getCharacterCurrentQuest(id);
    await ethers.provider.send("evm_mine", [quest.cooldown.toNumber()]);
    await this.quests.claimQuest(id);
    expect(await this.resource.balanceOf(id)).to.eq(3);
    expect(await this.experience.getExperience(id)).to.eq(10035);
    quest = await this.quests.getCharacterCurrentQuest(id);
    expect(quest.claimed_reward).to.eq(true);
  });

  it("should add a quest complete it and claim with 100% fulfillment", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.quests.addQuest(
        "test",
        "test 2",
        0,
        [this.resource.address],
        [10],
        100,
        { might: 2, speed: 2, intellect: 2 },
        3600,
        1,
      ),
    )
      .to.emit(this.quests, "AddQuest")
      .withArgs(2, "test", "test 2");
    await this.quests.startQuest(id, 2, { might: 2, speed: 2, intellect: 2 });
    const quest = await this.quests.getCharacterCurrentQuest(id);
    await ethers.provider.send("evm_mine", [quest.cooldown.toNumber()]);
    await this.quests.claimQuest(id);
    expect(await this.resource.balanceOf(id)).to.eq(13);
    expect(await this.experience.getExperience(id)).to.eq(10135);
  });

  it("should fail when trying to update a quest that doesn't exist", async () => {
    await expect(
      this.quests.updateQuest({
        id: 5,
        name: "test",
        description: "test 2",
        quest_type: 0,
        gold_reward: 10,
        resources_reward: [this.resource.address],
        resources_amounts: [10],
        experience_reward: 100,
        stats_cost: { might: 2, speed: 2, intellect: 2 },
        cooldown: 3600,
        level_required: 1,
        available: true,
      }),
    ).to.revertedWith("Quests: updateQuest() invalid quest id.");
  });

  it("should update a quest correctly", async () => {
    let quest = await this.quests.getQuest(1);
    expect(quest.stats_cost.might).to.eq(7);
    await this.quests.updateQuest({
      id: 1,
      name: "test",
      description: "test 2",
      quest_type: 0,
      gold_reward: 10,
      resources_reward: [this.resource.address],
      resources_amounts: [10],
      experience_reward: 100,
      stats_cost: { might: 5, speed: 2, intellect: 2 },
      cooldown: 3600,
      level_required: 1,
      available: true,
    });
    quest = await this.quests.getQuest(1);
    expect(quest.stats_cost.might).to.eq(5);
  });
});
