use super::{Double, EdgeMode, Length};

#[derive(Debug, Default)]
pub struct Shadow {
    pub mode: EdgeMode,
    pub offset: Double<Length>,
    pub blur: Length,
    pub spread: Length,
}
