use std::convert::From;
use std::fmt::{self, Write};

use super::liang_yi::LiangYi;

lazy_static! {
    pub static ref XIAN_TIAN: Vec<BaGua> = vec![
        BaGua::qian(),
        BaGua::dui(),
        BaGua::li(),
        BaGua::zhen(),
        BaGua::xun(),
        BaGua::kan(),
        BaGua::gen(),
        BaGua::kun()
    ];
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaGua {
    pub top: LiangYi,
    pub middle: LiangYi,
    pub bottom: LiangYi,
}

impl BaGua {
    pub fn qian() -> Self {
        Self {
            top: LiangYi::Yang,
            middle: LiangYi::Yang,
            bottom: LiangYi::Yang,
        }
    }

    pub fn kun() -> Self {
        Self {
            top: LiangYi::Yin,
            middle: LiangYi::Yin,
            bottom: LiangYi::Yin,
        }
    }

    pub fn zhen() -> Self {
        Self {
            top: LiangYi::Yin,
            middle: LiangYi::Yin,
            bottom: LiangYi::Yang,
        }
    }

    pub fn gen() -> Self {
        Self {
            top: LiangYi::Yang,
            middle: LiangYi::Yin,
            bottom: LiangYi::Yin,
        }
    }

    pub fn li() -> Self {
        Self {
            top: LiangYi::Yang,
            middle: LiangYi::Yin,
            bottom: LiangYi::Yang,
        }
    }

    pub fn kan() -> Self {
        Self {
            top: LiangYi::Yin,
            middle: LiangYi::Yang,
            bottom: LiangYi::Yin,
        }
    }

    pub fn dui() -> Self {
        Self {
            top: LiangYi::Yin,
            middle: LiangYi::Yang,
            bottom: LiangYi::Yang,
        }
    }

    pub fn xun() -> Self {
        Self {
            top: LiangYi::Yang,
            middle: LiangYi::Yang,
            bottom: LiangYi::Yin,
        }
    }

    pub const SONG: &'static str =
        "乾三連，坤六斷，震仰盂，艮覆碗，離中虛，坎中滿，兌上缺，巽下斷。";
}

impl From<&BaGua> for u32 {
    fn from(item: &BaGua) -> Self {
        let i = 0x2630;

        match item.bottom {
            LiangYi::Yang => match item.middle {
                LiangYi::Yang => match item.top {
                    LiangYi::Yang => i,
                    LiangYi::Yin => i + 1,
                },
                LiangYi::Yin => match item.top {
                    LiangYi::Yang => i + 2,
                    LiangYi::Yin => i + 3,
                },
            },
            LiangYi::Yin => match item.middle {
                LiangYi::Yang => match item.top {
                    LiangYi::Yang => i + 4,
                    LiangYi::Yin => i + 5,
                },
                LiangYi::Yin => match item.top {
                    LiangYi::Yang => i + 6,
                    LiangYi::Yin => i + 7,
                },
            },
        }
    }
}

impl fmt::Display for BaGua {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match std::char::from_u32(u32::from(self)) {
            Some(v) => write!(fmt, "{}", v),
            None => Err(fmt::Error::default()),
        }
    }
}

impl fmt::Debug for BaGua {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.bottom {
            LiangYi::Yang => match self.middle {
                LiangYi::Yang => match self.top {
                    LiangYi::Yang => f.write_char('乾'),
                    LiangYi::Yin => f.write_char('兌'),
                },
                LiangYi::Yin => match self.top {
                    LiangYi::Yang => f.write_char('離'),
                    LiangYi::Yin => f.write_char('震'),
                },
            },
            LiangYi::Yin => match self.middle {
                LiangYi::Yang => match self.top {
                    LiangYi::Yang => f.write_char('巽'),
                    LiangYi::Yin => f.write_char('坎'),
                },
                LiangYi::Yin => match self.top {
                    LiangYi::Yang => f.write_char('艮'),
                    LiangYi::Yin => f.write_char('坤'),
                },
            },
        }
    }
}
