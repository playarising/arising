import { PublicKey } from "@solana/web3.js";

export const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const CIVILIZATION_KEYS = [
  "ard",
  "hartenn",
  "ikarans",
  "zhand",
  "shinkari",
  "tarki",
] as const;
export type CivilizationKey = (typeof CIVILIZATION_KEYS)[number];

type CivilizationVariantMap = {
  [K in CivilizationKey]: Record<K, {}>;
};

export type CivilizationVariant = CivilizationVariantMap[CivilizationKey];

export type CivilizationConfig = {
  key: CivilizationKey;
  index: number;
  display: string;
  variant: CivilizationVariant;
  stats: CoreStatsShape;
  attributes: AttributesShape;
};

export type CoreStatsShape = {
  might: number;
  speed: number;
  intellect: number;
};

export type AttributesShape = {
  atk: number;
  def: number;
  magAtk: number;
  magDef: number;
  range: number;
  rate: number;
};

export type CollectionSetup = {
  mint: PublicKey;
  metadata: PublicKey;
  edition: PublicKey;
};

export const BASE_ENERGY = 10;

export const CIVILIZATIONS: CivilizationConfig[] = [
  {
    key: "ard",
    index: 0,
    display: "Ard",
    variant: { ard: {} },
    stats: { might: 3, speed: 2, intellect: 1 },
    attributes: { atk: 2, def: 1, magAtk: 0, magDef: 0, range: 0, rate: 0 },
  },
  {
    key: "hartenn",
    index: 1,
    display: "Hartenn",
    variant: { hartenn: {} },
    stats: { might: 3, speed: 1, intellect: 2 },
    attributes: { atk: 1, def: 0, magAtk: 0, magDef: 2, range: 0, rate: 0 },
  },
  {
    key: "ikarans",
    index: 2,
    display: "I'karans",
    variant: { ikarans: {} },
    stats: { might: 1, speed: 3, intellect: 2 },
    attributes: { atk: 1, def: 0, magAtk: 0, magDef: 0, range: 1, rate: 1 },
  },
  {
    key: "zhand",
    index: 3,
    display: "Zhand",
    variant: { zhand: {} },
    stats: { might: 1, speed: 2, intellect: 3 },
    attributes: { atk: 0, def: 0, magAtk: 2, magDef: 0, range: 0, rate: 1 },
  },
  {
    key: "shinkari",
    index: 4,
    display: "Shinkari",
    variant: { shinkari: {} },
    stats: { might: 2, speed: 3, intellect: 1 },
    attributes: { atk: 1, def: 0, magAtk: 1, magDef: 0, range: 0, rate: 1 },
  },
  {
    key: "tarki",
    index: 5,
    display: "Tark'i",
    variant: { tarki: {} },
    stats: { might: 3, speed: 2, intellect: 1 },
    attributes: { atk: 2, def: 0, magAtk: 0, magDef: 0, range: 0, rate: 1 },
  },
];
