use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub const AMENOMA_KAGEUCHI_STATIC_DATA: WeaponStaticData = WeaponStaticData {
    weapon_type: WeaponType::Sword,
    weapon_sub_stat: WeaponSubStatFamily::ATK120,
    weapon_base: WeaponBaseATKFamily::ATK454,
    star: 4
};