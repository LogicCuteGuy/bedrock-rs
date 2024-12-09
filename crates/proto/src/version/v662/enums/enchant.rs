pub mod Enchant {
    use bedrockrs_macros::ProtoCodec;

    #[derive(ProtoCodec)]
    #[enum_repr(i8)]
    #[repr(i8)]
    pub enum Type {
        ArmorAll = 0,
        ArmorFire = 1,
        ArmorFall = 2,
        ArmorExplosive = 3,
        ArmorProjectile = 4,
        ArmorThorns = 5,
        WaterBreath = 6,
        WaterSpeed = 7,
        WaterAffinity = 8,
        WeaponDamage = 9,
        WeaponUndead = 10,
        WeaponArthropod = 11,
        WeaponKnockback = 12,
        WeaponFire = 13,
        WeaponLoot = 14,
        MiningEfficiency = 15,
        MiningSilkTouch = 16,
        MiningDurability = 17,
        MiningLoot = 18,
        BowDamage = 19,
        BowKnockback = 20,
        BowFire = 21,
        BowInfinity = 22,
        FishingLoot = 23,
        FishingLure = 24,
        FrostWalker = 25,
        Mending = 26,
        CurseBinding = 27,
        CurseVanishing = 28,
        TridentImpaling = 29,
        TridentRiptide = 30,
        TridentLoyalty = 31,
        TridentChanneling = 32,
        CrossbowMultishot = 33,
        CrossbowPiercing = 34,
        CrossbowQuickCharge = 35,
        SoulSpeed = 36,
        SwiftSneak = 37,
        NumEnchantments = 38,
        InvalidEnchantment = 39,
    }
}