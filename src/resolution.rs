use clap::{builder::PossibleValue, ValueEnum};


#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Resolutions {
    NONE = -1,
    _8k = 4320,
    _4k = 2160,
    _2k = 1556,
    _1080p = 1080,
    _720p = 720,
    _480p = 480,
}
impl Resolutions {
    fn to_str(&self) -> &'static str {
        match self {
            Resolutions::NONE => "none",
            Resolutions::_8k => "8k",
            Resolutions::_4k => "4k",
            Resolutions::_2k => "2k",
            Resolutions::_1080p => "1080p",
            Resolutions::_720p => "720p",
            Resolutions::_480p => "480p",
        }
    }
}


impl ToString for Resolutions {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl ValueEnum for Resolutions {
    fn value_variants<'a>() -> &'a [Self] {
        return &[Self::NONE, Self::_480p, Self::_720p, Self::_1080p, Self::_2k, Self::_4k, Self::_8k]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {

        return Some(PossibleValue::new(self.to_str()));
    }
}