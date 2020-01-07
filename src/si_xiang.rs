use std::convert::From;
use std::fmt;

use super::liang_yi::LiangYi;

lazy_static! {
    pub static ref SI_XIANG: Vec<SiXiang> = vec![
        SiXiang::lao_yang(),
        SiXiang::shoao_yang(),
        SiXiang::shao_yin(),
        SiXiang::lao_yin()
    ];
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SiXiang {
    pub up: LiangYi,
    pub down: LiangYi,
}

impl SiXiang {
    pub fn lao_yang() -> Self {
        Self {
            up: LiangYi::Yang,
            down: LiangYi::Yang,
        }
    }
    pub fn shoao_yang() -> Self {
        Self {
            up: LiangYi::Yin,
            down: LiangYi::Yang,
        }
    }
    pub fn shao_yin() -> Self {
        Self {
            up: LiangYi::Yang,
            down: LiangYi::Yin,
        }
    }
    pub fn lao_yin() -> Self {
        Self {
            up: LiangYi::Yin,
            down: LiangYi::Yin,
        }
    }
}

impl From<&SiXiang> for u32 {
    fn from(item: &SiXiang) -> Self {
        let i = 0x268C;

        match item.down {
            LiangYi::Yang => match item.up {
                LiangYi::Yang => i,
                LiangYi::Yin => i + 1,
            },
            LiangYi::Yin => match item.up {
                LiangYi::Yang => i + 2,
                LiangYi::Yin => i + 3,
            },
        }
    }
}

impl fmt::Display for SiXiang {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match std::char::from_u32(u32::from(self)) {
            Some(v) => write!(fmt, "{}", v),
            None => Err(fmt::Error::default()),
        }
    }
}

impl fmt::Debug for SiXiang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.down {
            LiangYi::Yang => match self.up {
                LiangYi::Yang => f.write_str("老陽"),
                LiangYi::Yin => f.write_str("少陽"),
            },
            LiangYi::Yin => match self.up {
                LiangYi::Yang => f.write_str("少陰"),
                LiangYi::Yin => f.write_str("老陰"),
            },
        }
    }
}
