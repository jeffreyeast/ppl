//  This module holds debugging aids

use strum_macros::{EnumString, EnumIter, Display};
use strum::IntoEnumIterator;

use super::options::Options;



#[derive(Clone, Debug, Display, Eq, Hash, PartialEq, EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum DebugOption {
    BuiltinFunctions,
    DataConversion,
    Execution,
    Format,
    Function,
    Lex,
    NodeInvocation,
    Parse,
    StackUsage,
    ValueStack,
}

impl DebugOption {
    pub fn new() -> Options<DebugOption> {
        let mut options = Options::new();
        options.init(&mut DebugOption::iter() as &mut dyn Iterator<Item = DebugOption>);
        options
    }
}
