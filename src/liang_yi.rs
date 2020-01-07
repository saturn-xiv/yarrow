use std::convert::From;
use std::fmt::{self, Write};

lazy_static! {
    pub static ref LIANG_YI: Vec<LiangYi> = vec![LiangYi::Yang, LiangYi::Yin];
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LiangYi {
    Yin,
    Yang,
}

impl From<&LiangYi> for u32 {
    fn from(item: &LiangYi) -> Self {
        let i = 0x268A;
        match item {
            LiangYi::Yang => i,
            LiangYi::Yin => i + 1,
        }
    }
}

impl fmt::Display for LiangYi {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match std::char::from_u32(u32::from(self)) {
            Some(v) => write!(fmt, "{}", v),
            None => Err(fmt::Error::default()),
        }
    }
}

impl fmt::Debug for LiangYi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yang => f.write_char('陽'),
            Self::Yin => f.write_char('陰'),
        }
    }
}
