use std::fmt::{self, Write};

lazy_static! {
    pub static ref WU_XING: Vec<WuXing> = vec![
        WuXing::Jin,
        WuXing::Mu,
        WuXing::Shui,
        WuXing::Huo,
        WuXing::Tu
    ];
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum WuXing {
    Jin,
    Mu,
    Shui,
    Huo,
    Tu,
}

impl fmt::Display for WuXing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Jin => f.write_char('金'),
            Self::Mu => f.write_char('木'),
            Self::Shui => f.write_char('水'),
            Self::Huo => f.write_char('火'),
            Self::Tu => f.write_char('土'),
        }
    }
}
