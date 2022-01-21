use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::CharacterStaticData;
use crate::common::{ChangeAttribute, Element, StatName, WeaponType};

pub struct ZhongliSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub normal_dmg6: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],
    pub elemental_skill_dmg3: [f64; 15],
    pub elemental_skill_shield_base: [f64; 15],
    pub elemental_skill_shield_additional: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
}

pub const ZHONGLI_SKILL: ZhongliSkillType = ZhongliSkillType {
    normal_dmg1: [0.3077, 0.3327, 0.3578, 0.3936, 0.4186, 0.4472, 0.4866, 0.5259, 0.5653, 0.6082, 0.6574, 0.7153, 0.7731, 0.831, 0.8941],
    normal_dmg2: [0.3115, 0.3369, 0.3622, 0.3985, 0.4238, 0.4528, 0.4926, 0.5325, 0.5723, 0.6158, 0.6656, 0.7242, 0.7827, 0.8413, 0.9052],
    normal_dmg3: [0.3858, 0.4172, 0.4486, 0.4934, 0.5248, 0.5607, 0.61, 0.6594, 0.7087, 0.7626, 0.8242, 0.8968, 0.9693, 1.0418, 1.121],
    normal_dmg4: [0.4294, 0.4643, 0.4993, 0.5492, 0.5842, 0.6241, 0.679, 0.734, 0.7889, 0.8488, 0.9174, 0.9982, 1.0789, 1.1597, 1.2477],
    normal_dmg5: [0.1075, 0.1163, 0.125, 0.1375, 0.1463, 0.1563, 0.17, 0.1838, 0.1975, 0.2125, 0.2297, 0.2499, 0.2701, 0.2903, 0.3124],
    normal_dmg6: [0.545, 0.5893, 0.6337, 0.697, 0.7414, 0.7921, 0.8618, 0.9315, 1.0012, 1.0773, 1.1644, 1.2669, 1.3693, 1.4718, 1.5836],
    charged_dmg1: [1.1103, 1.2006, 1.291, 1.4201, 1.5105, 1.6138, 1.7558, 1.8978, 2.0398, 2.1947, 2.3722, 2.581, 2.7897, 2.9985, 3.2262],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [0.16, 0.172, 0.184, 0.2, 0.212, 0.224, 0.24, 0.256, 0.272, 0.288, 0.304, 0.32, 0.34, 0.36, 0.38],
    elemental_skill_dmg2: [0.32, 0.344, 0.368, 0.4, 0.424, 0.448, 0.48, 0.512, 0.544, 0.576, 0.608, 0.64, 0.68, 0.72, 0.76],
    elemental_skill_dmg3: [0.8, 0.86, 0.92, 1., 1.06, 1.12, 1.2, 1.28, 1.36, 1.44, 1.52, 1.6, 1.7, 1.8, 1.9],
    elemental_skill_shield_base: [1232.0, 1356.0, 1489.0, 1633.0, 1787.0, 1951.0, 2126.0, 2311.0, 2506.0, 2712.0, 2927.0, 3153.0, 3389.0, 3636.0, 3893.0],
    elemental_skill_shield_additional: [0.128, 0.1376, 0.1472, 0.16, 0.1696, 0.1792, 0.192, 0.2048, 0.2176, 0.2304, 0.2432, 0.256, 0.272, 0.288, 0.304],
    elemental_burst_dmg1: [4.0108, 4.4444, 4.878, 5.42, 5.9078, 6.3956, 7.046, 7.6964, 8.3468, 8.9972, 9.6476, 10.298, 10.84, 11.382, 11.924]
};

pub const ZHONGLI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    element:Element::Geo,
    hp: [1144, 2967, 3948, 5908, 6605, 7599, 8528, 9533, 10230, 11243, 11940, 12965, 13662, 14695],
    atk: [20, 51, 67, 101, 113, 130, 146, 163, 175, 192, 204, 222, 233, 251],
    def: [57, 149, 198, 297, 332, 382, 428, 479, 514, 564, 599, 651, 686, 738],
    sub_stat: CharacterSubStatFamily::Bonus288(StatName::GeoBonus),
    weapon_type: WeaponType::Polearm,
    star: 5
};

pub struct ZhongliEffect {
    pub has_talent2: bool,
}

impl ZhongliEffect {
    pub fn new(common_data: &CharacterCommonData) -> ZhongliEffect {
        ZhongliEffect {
            has_talent2: common_data.has_talent2
        }
    }
}

impl<T: Attribute> ChangeAttribute<T> for ZhongliEffect {
    fn change_attribute(&self, attribute: &mut T) {
        if self.has_talent2 {
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::ExtraDmgNormalAttack,
                Box::new(|x, _| x * 0.0139),
                Box::new(|grad, _x1, _x2| (grad * 0.0139, 0.0)),
                "钟离天赋：炊金馔玉"
            );
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::ExtraDmgChargedAttack,
                Box::new(|x, _| x * 0.0139),
                Box::new(|grad, _x1, _x2| (grad * 0.0139, 0.0)),
                "钟离天赋：炊金馔玉"
            );
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::ExtraDmgPlungingAttack,
                Box::new(|x, _| x * 0.0139),
                Box::new(|grad, _x1, _x2| (grad * 0.0139, 0.0)),
                "钟离天赋：炊金馔玉"
            );
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::ExtraDmgElementalSkill,
                Box::new(|x, _| x * 0.019),
                Box::new(|grad, _x1, _x2| (grad * 0.019, 0.0)),
                "钟离天赋：炊金馔玉"
            );
            attribute.add_edge1(
                AttributeName::HP,
                AttributeName::ExtraDmgElementalBurst,
                Box::new(|x, _| x * 0.33),
                Box::new(|grad, _x1, _x2| (grad * 0.33, 0.0)),
                "钟离天赋：炊金馔玉"
            );
        }
    }
}