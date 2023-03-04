#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UciFormatOptions {
    pub chess960: bool,
    pub wdl: bool,
}
