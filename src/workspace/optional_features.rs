//  This module manages optional functionality

use strum_macros::{EnumString, EnumIter, Display};
use strum::IntoEnumIterator;

use super::options::Options;



#[derive(Clone, Debug, Display, Eq, Hash, PartialEq, EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum Feature {
    LineNames,
    StringEscapes,
}


impl Feature {
    pub fn new() -> Options<Feature> {
        let mut options = Options::new();
        options.init(&mut Feature::iter() as &mut dyn Iterator<Item = Feature>);
        options
    }
}