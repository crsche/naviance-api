use std::ops::RangeInclusive;

use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

pub fn sat_to_act(mut sat: u32) -> u32 {
    sat /= 10;
    // I'm too lazy to find a formula for this
    // Also, fuck the college board
    match sat {
        157..=160 => 36,
        153..=156 => 35,
        149..=152 => 34,
        145..=148 => 33,
        142..=144 => 32,
        139..=141 => 31,
        136..=138 => 30,
        133..=135 => 29,
        130..=132 => 28,
        126..=129 => 27,
        123..=125 => 26,
        120..=122 => 25,
        116..=119 => 24,
        113..=115 => 23,
        110..=112 => 22,
        106..=109 => 21,
        103..=105 => 20,
        99..=102 => 19,
        96..=98 => 18,
        92..=95 => 17,
        88..=91 => 16,
        83..=87 => 15,
        78..=82 => 14,
        73..=77 => 13,
        69..=72 => 12,
        65..=68 => 11,
        62..=64 => 10,
        59..=61 => 9,
        _ => unreachable!(),
    }
}

pub fn act_to_sat(act: u32) -> RangeInclusive<u32> {
    match act {
        36 => 1570..=1600,
        35 => 1530..=1560,
        34 => 1490..=1520,
        33 => 1450..=1480,
        32 => 1420..=1440,
        31 => 1390..=1410,
        30 => 1360..=1380,
        29 => 1330..=1350,
        28 => 1300..=1320,
        27 => 1260..=1290,
        26 => 1230..=1250,
        25 => 1200..=1220,
        24 => 1160..=1190,
        23 => 1130..=1150,
        22 => 1100..=1120,
        21 => 1060..=1090,
        20 => 1030..=1050,
        19 => 990..=1020,
        18 => 960..=980,
        17 => 920..=950,
        16 => 880..=910,
        15 => 830..=870,
        14 => 780..=820,
        13 => 730..=770,
        12 => 690..=720,
        11 => 650..=680,
        10 => 620..=640,
        9 => 590..=610,
        _ => unreachable!(),
    }
}

pub fn none_if_zero<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<i64>::deserialize(deserializer)? {
        Some(0) => Ok(None),
        other => Ok(other),
        // Ok(0) => None,
        // None => None,
        // Ok(n) => Ok(Some(n)),
        // Err(e) => Err(e),
    }
}

pub fn none_if_empty_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer)? {
        Some(s) if s.is_empty() => Ok(None),
        other => Ok(other),
    }
}
// pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     match u8::deserialize(deserializer)? {
//         0 => Ok(false),
//         1 => Ok(true),
//         other => Err(de::Error::invalid_value(
//             Unexpected::Unsigned(other as u64),
//             &"zero or one",
//         )),
//     }
// }

pub fn bool_from_int_opt<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<u8>::deserialize(deserializer)? {
        None => Ok(None),
        Some(0) => Ok(Some(false)),
        Some(1) => Ok(Some(true)),
        Some(other) => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}
