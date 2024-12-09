use bedrockrs_macros::ProtoCodec;

#[derive(ProtoCodec, Clone, Debug)]
pub enum ParticleType {
    Undefined = 0,
    Bubble = 1,
    BubbleManual = 2,
    Crit = 3,
    BlockForceField = 4,
    Smoke = 5,
    Explode = 6,
    Evaporation = 7,
    Flame = 8,
    CandleFlame = 9,
    Lava = 10,
    LargeSmoke = 11,
    RedDust = 12,
    RisingBorderDust = 13,
    IconCrack = 14,
    SnowballPoof = 15,
    LargeExplode = 16,
    HugeExplosion = 17,
    BreezeWindExplosion = 18,
    MobFlame = 19,
    Heart = 20,
    Terrain = 21,
    TownAura = 22,
    Portal = 23,
    MobPortal = 24,
    WaterSplash = 25,
    WaterSplashManual = 26,
    WaterWake = 27,
    DripWater = 28,
    DripLava = 29,
    DripHoney = 30,
    StalactiteDripWater = 31,
    StalactiteDripLava = 32,
    FallingDust = 33,
    MobSpell = 34,
    MobSpellAmbient = 35,
    MobSpellInstantaneous = 36,
    Ink = 37,
    Slime = 38,
    RainSplash = 39,
    VillagerAngry = 40,
    VillagerHappy = 41,
    EnchantingTable = 42,
    TrackingEmitter = 43,
    Note = 44,
    WitchSpell = 45,
    CarrotBoost = 46,
    MobAppearance = 47,
    EndRod = 48,
    DragonBreath = 49,
    Spit = 50,
    Totem = 51,
    Food = 52,
    FireworksStarter = 53,
    Fireworks = 54,
    FireworksOverlay = 55,
    BalloonGas = 56,
    ColoredFlame = 57,
    Sparkler = 58,
    Conduit = 59,
    BubbleColumnUp = 60,
    BubbleColumnDown = 61,
    Sneeze = 62,
    ShulkerBullet = 63,
    Bleach = 64,
    DragonDestroyBlock = 65,
    MyceliumDust = 66,
    FallingBorderDust = 67,
    CampfireSmoke = 68,
    CampfireSmokeTall = 69,
    DragonBreathFire = 70,
    DragonBreathTrail = 71,
    BlueFlame = 72,
    Soul = 73,
    ObsidianTear = 74,
    PortalReverse = 75,
    Snowflake = 76,
    VibrationSignal = 77,
    SculkSensorRedstone = 78,
    SporeBlossomShower = 79,
    SporeBlossomAmbient = 80,
    Wax = 81,
    ElectricSpark = 82,
    Shriek = 83,
    SculkSoul = 84,
    SonicExplosion = 85,
    BrushDust = 86,
    CherryLeaves = 87,
    DustPlume = 88,
    WhiteSmoke = 89,
    VaultConnection = 90,
    WindExplosion = 91,
    Count = 92,
}