# Codex Reference

Generated from tier/item tables. Values shown per tier.

## Quests
- Total quests: 83 entries, level-ordered, no gaps larger than 3 levels.
- Reward policy: raw resources only (ores/essence/leather/wood) plus modest gold/XP; no refined bars/ingots/planks from quests.
- Energy & stats: higher tiers scale base energy and stat gates; efficiency bonus still applies for high stats.
- Tier coverage:
  - 1–20: wood/bronze/early leather loops plus jobs/raids (caravan guard, forest ambush, bronze smuggler/skirmish).
  - 21–50: iron/steel/silver/gold progression with jobs, patrols, vanguard skirmishes, smuggler runs.
  - 51–90: platinum/adamantine/cobalt fillers and raids (prospector clashes, bastions, recon).
  - 91–120: angelic/crimson/fateful farms, jobs, raids for essences.
  - 121–140: fateful/altair hunts, sieges, and lines to finish the curve.
- Lookup: `quest_by_id(id)` in `codex/quests.rs` (IDs match sorted order).

## Equipment
### Wood (min lvl 1, mult 1.0)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/3/0/2/0/-1 |
| SmallShield | LeftHand | False | 0/2/0/1/0/0 |
| Sword | Hand | False | 5/1/0/0/0/2 |
| Dagger | Hand | False | 4/0/0/0/0/3 |
| Bow | Hand | True | 5/0/0/0/3/1 |
| Crossbow | Hand | True | 6/0/0/0/2/1 |
| Spear | Hand | True | 6/1/0/0/1/1 |
| Lance | Hand | True | 7/1/0/0/1/0 |
| Halberd | Hand | True | 8/2/0/0/1/-1 |
| Rod | Hand | False | 0/0/6/1/0/1 |
| Staff | Hand | True | 0/0/8/2/0/0 |
| Rifle | Hand | True | 7/0/0/0/4/0 |
| Trident | Hand | True | 7/2/0/1/1/0 |
| Warhammer | Hand | True | 9/1/0/0/0/-2 |
| Quiver | Back | False | 2/0/0/0/2/2 |
| Chest | Chest | False | 0/5/0/2/0/0 |
| Legs | Legs | False | 0/3/0/1/0/1 |
| Boots | Boots | False | 0/2/0/1/0/1 |
| Helm | Helmet | False | 0/2/0/2/0/0 |
| Ring | Ring | False | 2/0/1/0/0/1 |
| Amulet | Amulet | False | 0/0/3/1/0/0 |
| Cloak | Back | False | 0/1/0/1/1/1 |

### Bronze (min lvl 10, mult 1.3)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/4/0/3/0/-1 |
| SmallShield | LeftHand | False | 0/3/0/1/0/0 |
| Sword | Hand | False | 6/1/0/0/0/3 |
| Dagger | Hand | False | 5/0/0/0/0/4 |
| Bow | Hand | True | 6/0/0/0/4/1 |
| Crossbow | Hand | True | 8/0/0/0/3/1 |
| Spear | Hand | True | 8/1/0/0/1/1 |
| Lance | Hand | True | 9/1/0/0/1/0 |
| Halberd | Hand | True | 10/3/0/0/1/-1 |
| Rod | Hand | False | 0/0/8/1/0/1 |
| Staff | Hand | True | 0/0/10/3/0/0 |
| Rifle | Hand | True | 9/0/0/0/5/0 |
| Trident | Hand | True | 9/3/0/1/1/0 |
| Warhammer | Hand | True | 12/1/0/0/0/-3 |
| Quiver | Back | False | 3/0/0/0/3/3 |
| Chest | Chest | False | 0/6/0/3/0/0 |
| Legs | Legs | False | 0/4/0/1/0/1 |
| Boots | Boots | False | 0/3/0/1/0/1 |
| Helm | Helmet | False | 0/3/0/3/0/0 |
| Ring | Ring | False | 3/0/1/0/0/1 |
| Amulet | Amulet | False | 0/0/4/1/0/0 |
| Cloak | Back | False | 0/1/0/1/1/1 |

### Iron (min lvl 20, mult 1.6)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/5/0/3/0/-2 |
| SmallShield | LeftHand | False | 0/3/0/2/0/0 |
| Sword | Hand | False | 8/2/0/0/0/3 |
| Dagger | Hand | False | 6/0/0/0/0/5 |
| Bow | Hand | True | 8/0/0/0/5/2 |
| Crossbow | Hand | True | 10/0/0/0/3/2 |
| Spear | Hand | True | 10/2/0/0/2/2 |
| Lance | Hand | True | 11/2/0/0/2/0 |
| Halberd | Hand | True | 13/3/0/0/2/-2 |
| Rod | Hand | False | 0/0/10/2/0/2 |
| Staff | Hand | True | 0/0/13/3/0/0 |
| Rifle | Hand | True | 11/0/0/0/6/0 |
| Trident | Hand | True | 11/3/0/2/2/0 |
| Warhammer | Hand | True | 14/2/0/0/0/-3 |
| Quiver | Back | False | 3/0/0/0/3/3 |
| Chest | Chest | False | 0/8/0/3/0/0 |
| Legs | Legs | False | 0/5/0/2/0/2 |
| Boots | Boots | False | 0/3/0/2/0/2 |
| Helm | Helmet | False | 0/3/0/3/0/0 |
| Ring | Ring | False | 3/0/2/0/0/2 |
| Amulet | Amulet | False | 0/0/5/2/0/0 |
| Cloak | Back | False | 0/2/0/2/2/2 |

### Steel (min lvl 31, mult 2.1)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/6/0/4/0/-2 |
| SmallShield | LeftHand | False | 0/4/0/2/0/0 |
| Sword | Hand | False | 10/2/0/0/0/4 |
| Dagger | Hand | False | 8/0/0/0/0/6 |
| Bow | Hand | True | 10/0/0/0/6/2 |
| Crossbow | Hand | True | 13/0/0/0/4/2 |
| Spear | Hand | True | 13/2/0/0/2/2 |
| Lance | Hand | True | 15/2/0/0/2/0 |
| Halberd | Hand | True | 17/4/0/0/2/-2 |
| Rod | Hand | False | 0/0/13/2/0/2 |
| Staff | Hand | True | 0/0/17/4/0/0 |
| Rifle | Hand | True | 15/0/0/0/8/0 |
| Trident | Hand | True | 15/4/0/2/2/0 |
| Warhammer | Hand | True | 19/2/0/0/0/-4 |
| Quiver | Back | False | 4/0/0/0/4/4 |
| Chest | Chest | False | 0/10/0/4/0/0 |
| Legs | Legs | False | 0/6/0/2/0/2 |
| Boots | Boots | False | 0/4/0/2/0/2 |
| Helm | Helmet | False | 0/4/0/4/0/0 |
| Ring | Ring | False | 4/0/2/0/0/2 |
| Amulet | Amulet | False | 0/0/6/2/0/0 |
| Cloak | Back | False | 0/2/0/2/2/2 |

### Silver (min lvl 40, mult 2.6)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/8/0/5/0/-3 |
| SmallShield | LeftHand | False | 0/5/0/3/0/0 |
| Sword | Hand | False | 13/3/0/0/0/5 |
| Dagger | Hand | False | 10/0/0/0/0/8 |
| Bow | Hand | True | 13/0/0/0/8/3 |
| Crossbow | Hand | True | 16/0/0/0/5/3 |
| Spear | Hand | True | 16/3/0/0/3/3 |
| Lance | Hand | True | 18/3/0/0/3/0 |
| Halberd | Hand | True | 21/5/0/0/3/-3 |
| Rod | Hand | False | 0/0/16/3/0/3 |
| Staff | Hand | True | 0/0/21/5/0/0 |
| Rifle | Hand | True | 18/0/0/0/10/0 |
| Trident | Hand | True | 18/5/0/3/3/0 |
| Warhammer | Hand | True | 23/3/0/0/0/-5 |
| Quiver | Back | False | 5/0/0/0/5/5 |
| Chest | Chest | False | 0/13/0/5/0/0 |
| Legs | Legs | False | 0/8/0/3/0/3 |
| Boots | Boots | False | 0/5/0/3/0/3 |
| Helm | Helmet | False | 0/5/0/5/0/0 |
| Ring | Ring | False | 5/0/3/0/0/3 |
| Amulet | Amulet | False | 0/0/8/3/0/0 |
| Cloak | Back | False | 0/3/0/3/3/3 |

### Gold (min lvl 50, mult 3.2)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/10/0/6/0/-3 |
| SmallShield | LeftHand | False | 0/6/0/3/0/0 |
| Sword | Hand | False | 16/3/0/0/0/6 |
| Dagger | Hand | False | 13/0/0/0/0/10 |
| Bow | Hand | True | 16/0/0/0/10/3 |
| Crossbow | Hand | True | 19/0/0/0/6/3 |
| Spear | Hand | True | 19/3/0/0/3/3 |
| Lance | Hand | True | 22/3/0/0/3/0 |
| Halberd | Hand | True | 26/6/0/0/3/-3 |
| Rod | Hand | False | 0/0/19/3/0/3 |
| Staff | Hand | True | 0/0/26/6/0/0 |
| Rifle | Hand | True | 22/0/0/0/13/0 |
| Trident | Hand | True | 22/6/0/3/3/0 |
| Warhammer | Hand | True | 29/3/0/0/0/-6 |
| Quiver | Back | False | 6/0/0/0/6/6 |
| Chest | Chest | False | 0/16/0/6/0/0 |
| Legs | Legs | False | 0/10/0/3/0/3 |
| Boots | Boots | False | 0/6/0/3/0/3 |
| Helm | Helmet | False | 0/6/0/6/0/0 |
| Ring | Ring | False | 6/0/3/0/0/3 |
| Amulet | Amulet | False | 0/0/10/3/0/0 |
| Cloak | Back | False | 0/3/0/3/3/3 |

### Platinum (min lvl 61, mult 3.9)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/12/0/8/0/-4 |
| SmallShield | LeftHand | False | 0/8/0/4/0/0 |
| Sword | Hand | False | 20/4/0/0/0/8 |
| Dagger | Hand | False | 16/0/0/0/0/12 |
| Bow | Hand | True | 20/0/0/0/12/4 |
| Crossbow | Hand | True | 23/0/0/0/8/4 |
| Spear | Hand | True | 23/4/0/0/4/4 |
| Lance | Hand | True | 27/4/0/0/4/0 |
| Halberd | Hand | True | 31/8/0/0/4/-4 |
| Rod | Hand | False | 0/0/23/4/0/4 |
| Staff | Hand | True | 0/0/31/8/0/0 |
| Rifle | Hand | True | 27/0/0/0/16/0 |
| Trident | Hand | True | 27/8/0/4/4/0 |
| Warhammer | Hand | True | 35/4/0/0/0/-8 |
| Quiver | Back | False | 8/0/0/0/8/8 |
| Chest | Chest | False | 0/20/0/8/0/0 |
| Legs | Legs | False | 0/12/0/4/0/4 |
| Boots | Boots | False | 0/8/0/4/0/4 |
| Helm | Helmet | False | 0/8/0/8/0/0 |
| Ring | Ring | False | 8/0/4/0/0/4 |
| Amulet | Amulet | False | 0/0/12/4/0/0 |
| Cloak | Back | False | 0/4/0/4/4/4 |

### Adamantine (min lvl 70, mult 4.7)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/14/0/9/0/-5 |
| SmallShield | LeftHand | False | 0/9/0/5/0/0 |
| Sword | Hand | False | 24/5/0/0/0/9 |
| Dagger | Hand | False | 19/0/0/0/0/14 |
| Bow | Hand | True | 24/0/0/0/14/5 |
| Crossbow | Hand | True | 28/0/0/0/9/5 |
| Spear | Hand | True | 28/5/0/0/5/5 |
| Lance | Hand | True | 33/5/0/0/5/0 |
| Halberd | Hand | True | 38/9/0/0/5/-5 |
| Rod | Hand | False | 0/0/28/5/0/5 |
| Staff | Hand | True | 0/0/38/9/0/0 |
| Rifle | Hand | True | 33/0/0/0/19/0 |
| Trident | Hand | True | 33/9/0/5/5/0 |
| Warhammer | Hand | True | 42/5/0/0/0/-9 |
| Quiver | Back | False | 9/0/0/0/9/9 |
| Chest | Chest | False | 0/24/0/9/0/0 |
| Legs | Legs | False | 0/14/0/5/0/5 |
| Boots | Boots | False | 0/9/0/5/0/5 |
| Helm | Helmet | False | 0/9/0/9/0/0 |
| Ring | Ring | False | 9/0/5/0/0/5 |
| Amulet | Amulet | False | 0/0/14/5/0/0 |
| Cloak | Back | False | 0/5/0/5/5/5 |

### Cobalt (min lvl 80, mult 5.6)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/17/0/11/0/-6 |
| SmallShield | LeftHand | False | 0/11/0/6/0/0 |
| Sword | Hand | False | 28/6/0/0/0/11 |
| Dagger | Hand | False | 22/0/0/0/0/17 |
| Bow | Hand | True | 28/0/0/0/17/6 |
| Crossbow | Hand | True | 34/0/0/0/11/6 |
| Spear | Hand | True | 34/6/0/0/6/6 |
| Lance | Hand | True | 39/6/0/0/6/0 |
| Halberd | Hand | True | 45/11/0/0/6/-6 |
| Rod | Hand | False | 0/0/34/6/0/6 |
| Staff | Hand | True | 0/0/45/11/0/0 |
| Rifle | Hand | True | 39/0/0/0/22/0 |
| Trident | Hand | True | 39/11/0/6/6/0 |
| Warhammer | Hand | True | 50/6/0/0/0/-11 |
| Quiver | Back | False | 11/0/0/0/11/11 |
| Chest | Chest | False | 0/28/0/11/0/0 |
| Legs | Legs | False | 0/17/0/6/0/6 |
| Boots | Boots | False | 0/11/0/6/0/6 |
| Helm | Helmet | False | 0/11/0/11/0/0 |
| Ring | Ring | False | 11/0/6/0/0/6 |
| Amulet | Amulet | False | 0/0/17/6/0/0 |
| Cloak | Back | False | 0/6/0/6/6/6 |

### Angelic (min lvl 91, mult 6.6)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/20/0/13/0/-7 |
| SmallShield | LeftHand | False | 0/13/0/7/0/0 |
| Sword | Hand | False | 33/7/0/0/0/13 |
| Dagger | Hand | False | 26/0/0/0/0/20 |
| Bow | Hand | True | 33/0/0/0/20/7 |
| Crossbow | Hand | True | 40/0/0/0/13/7 |
| Spear | Hand | True | 40/7/0/0/7/7 |
| Lance | Hand | True | 46/7/0/0/7/0 |
| Halberd | Hand | True | 53/13/0/0/7/-7 |
| Rod | Hand | False | 0/0/40/7/0/7 |
| Staff | Hand | True | 0/0/53/13/0/0 |
| Rifle | Hand | True | 46/0/0/0/26/0 |
| Trident | Hand | True | 46/13/0/7/7/0 |
| Warhammer | Hand | True | 59/7/0/0/0/-13 |
| Quiver | Back | False | 13/0/0/0/13/13 |
| Chest | Chest | False | 0/33/0/13/0/0 |
| Legs | Legs | False | 0/20/0/7/0/7 |
| Boots | Boots | False | 0/13/0/7/0/7 |
| Helm | Helmet | False | 0/13/0/13/0/0 |
| Ring | Ring | False | 13/0/7/0/0/7 |
| Amulet | Amulet | False | 0/0/20/7/0/0 |
| Cloak | Back | False | 0/7/0/7/7/7 |

### Crimson (min lvl 105, mult 7.8)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/23/0/16/0/-8 |
| SmallShield | LeftHand | False | 0/16/0/8/0/0 |
| Sword | Hand | False | 39/8/0/0/0/16 |
| Dagger | Hand | False | 31/0/0/0/0/23 |
| Bow | Hand | True | 39/0/0/0/23/8 |
| Crossbow | Hand | True | 47/0/0/0/16/8 |
| Spear | Hand | True | 47/8/0/0/8/8 |
| Lance | Hand | True | 55/8/0/0/8/0 |
| Halberd | Hand | True | 62/16/0/0/8/-8 |
| Rod | Hand | False | 0/0/47/8/0/8 |
| Staff | Hand | True | 0/0/62/16/0/0 |
| Rifle | Hand | True | 55/0/0/0/31/0 |
| Trident | Hand | True | 55/16/0/8/8/0 |
| Warhammer | Hand | True | 70/8/0/0/0/-16 |
| Quiver | Back | False | 16/0/0/0/16/16 |
| Chest | Chest | False | 0/39/0/16/0/0 |
| Legs | Legs | False | 0/23/0/8/0/8 |
| Boots | Boots | False | 0/16/0/8/0/8 |
| Helm | Helmet | False | 0/16/0/16/0/0 |
| Ring | Ring | False | 16/0/8/0/0/8 |
| Amulet | Amulet | False | 0/0/23/8/0/0 |
| Cloak | Back | False | 0/8/0/8/8/8 |

### Fateful (min lvl 121, mult 9.2)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/28/0/18/0/-9 |
| SmallShield | LeftHand | False | 0/18/0/9/0/0 |
| Sword | Hand | False | 46/9/0/0/0/18 |
| Dagger | Hand | False | 37/0/0/0/0/28 |
| Bow | Hand | True | 46/0/0/0/28/9 |
| Crossbow | Hand | True | 55/0/0/0/18/9 |
| Spear | Hand | True | 55/9/0/0/9/9 |
| Lance | Hand | True | 64/9/0/0/9/0 |
| Halberd | Hand | True | 74/18/0/0/9/-9 |
| Rod | Hand | False | 0/0/55/9/0/9 |
| Staff | Hand | True | 0/0/74/18/0/0 |
| Rifle | Hand | True | 64/0/0/0/37/0 |
| Trident | Hand | True | 64/18/0/9/9/0 |
| Warhammer | Hand | True | 83/9/0/0/0/-18 |
| Quiver | Back | False | 18/0/0/0/18/18 |
| Chest | Chest | False | 0/46/0/18/0/0 |
| Legs | Legs | False | 0/28/0/9/0/9 |
| Boots | Boots | False | 0/18/0/9/0/9 |
| Helm | Helmet | False | 0/18/0/18/0/0 |
| Ring | Ring | False | 18/0/9/0/0/9 |
| Amulet | Amulet | False | 0/0/28/9/0/0 |
| Cloak | Back | False | 0/9/0/9/9/9 |

### Altair (min lvl 135, mult 11.0)
| Item | Slot | Two-Hand | Attributes (atk/def/matk/mdef/range/rate) |
| --- | --- | --- | --- |
| BigShield | LeftHand | False | 0/33/0/22/0/-11 |
| SmallShield | LeftHand | False | 0/22/0/11/0/0 |
| Sword | Hand | False | 55/11/0/0/0/22 |
| Dagger | Hand | False | 44/0/0/0/0/33 |
| Bow | Hand | True | 55/0/0/0/33/11 |
| Crossbow | Hand | True | 66/0/0/0/22/11 |
| Spear | Hand | True | 66/11/0/0/11/11 |
| Lance | Hand | True | 77/11/0/0/11/0 |
| Halberd | Hand | True | 88/22/0/0/11/-11 |
| Rod | Hand | False | 0/0/66/11/0/11 |
| Staff | Hand | True | 0/0/88/22/0/0 |
| Rifle | Hand | True | 77/0/0/0/44/0 |
| Trident | Hand | True | 77/22/0/11/11/0 |
| Warhammer | Hand | True | 99/11/0/0/0/-22 |
| Quiver | Back | False | 22/0/0/0/22/22 |
| Chest | Chest | False | 0/55/0/22/0/0 |
| Legs | Legs | False | 0/33/0/11/0/11 |
| Boots | Boots | False | 0/22/0/11/0/11 |
| Helm | Helmet | False | 0/22/0/22/0/0 |
| Ring | Ring | False | 22/0/11/0/0/11 |
| Amulet | Amulet | False | 0/0/33/11/0/0 |
| Cloak | Back | False | 0/11/0/11/11/11 |

## Recipes
### Base Forge
- Plank Forging: Wood x1 -> Plank (lvl 1, 1h, energy 5)
- Leather Tanning: Raw Leather x1 -> Leather (lvl 2, 1h15m, energy 6)

### Refining per Tier
- Wood Refining (id 2): Wood x1 -> Plank (lvl 1, 1h30m, energy 8)
- Bronze Refining (id 3): BronzeOre x1 -> BronzeBar (lvl 10, 1h30m, energy 8)
- Iron Refining (id 4): IronOre x1 -> IronIngot (lvl 20, 1h30m, energy 8)
- Steel Refining (id 5): SteelOre x1 -> SteelIngot (lvl 31, 1h30m, energy 8)
- Silver Refining (id 6): SilverOre x1 -> SilverIngot (lvl 40, 1h30m, energy 8)
- Gold Refining (id 7): GoldOre x1 -> GoldIngot (lvl 50, 1h30m, energy 8)
- Platinum Refining (id 8): PlatinumOre x1 -> PlatinumIngot (lvl 61, 1h30m, energy 8)
- Adamantine Refining (id 9): AdamantineOre x1 -> AdamantineIngot (lvl 70, 1h30m, energy 8)
- Cobalt Refining (id 10): CobaltOre x1 -> CobaltIngot (lvl 80, 1h30m, energy 8)
- Angelic Refining (id 11): AngelicEssence x1 -> AngelicIngot (lvl 91, 1h30m, energy 8)
- Crimson Refining (id 12): CrimsonEssence x1 -> CrimsonIngot (lvl 105, 1h30m, energy 8)
- Fateful Refining (id 13): FatefulEssence x1 -> FatefulIngot (lvl 121, 1h30m, energy 8)
- Altair Refining (id 14): AltairEssence x1 -> AltairIngot (lvl 135, 1h30m, energy 8)

### Crafting per Tier (all items share pattern)
For each tier item, materials = current refined (scaled by item offset), previous refined (scaled), leather (scaled), plus plank for non-Wood tiers; gold scales with tier/offset.
## Quests
| ID | Type | Name | Level | Cooldown (s) | Energy | Req (might/speed/intellect) |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Job | Caravan Guard | 1 | 3600 | 5 | 0/1/1 |
| 2 | Farm | Woodcutting | 1 | 3600 | 5 | 1/1/0 |
| 3 | Raid | Bandit Camp | 1 | 3600 | 7 | 1/0/1 |
| 4 | Raid | Scouting Drills | 1 | 2700 | 6 | 0/1/0 |
| 5 | Farm | Timber Stockpile | 2 | 4500 | 7 | 1/1/0 |
| 6 | Job | Artisan Support | 3 | 4800 | 8 | 0/1/1 |
| 7 | Farm | Hunting Grounds | 2 | 5400 | 6 | 1/1/0 |
| 9 | Farm | Mountain Prospecting | 3 | 5400 | 7 | 2/0/1 |
| 11 | Job | Merchant Escort | 2 | 4500 | 6 | 0/1/1 |
| 12 | Job | Royal Guard Duty | 3 | 5400 | 7 | 1/1/1 |
| 15 | Raid | Forest Ambush | 2 | 4500 | 6 | 1/1/0 |
| 16 | Raid | Ruined Keep | 3 | 5400 | 7 | 1/0/2 |
