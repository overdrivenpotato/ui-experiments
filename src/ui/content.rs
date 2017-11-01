use super::{Double, Percentage};

#[derive(Debug)]
pub enum AlignMode {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignMode {
    fn default() -> Self {
        AlignMode::Start
    }
}

#[derive(Debug, Default)]
pub struct Align(pub Double<AlignMode>);

#[derive(Debug)]
pub enum Direction {
    Forwards,
    Backwards,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Forwards
    }
}

#[derive(Debug)]
pub enum Flow {
    Row(Direction),
    Column(Direction),
}

impl Default for Flow {
    fn default() -> Flow {
        Flow::Row(Default::default())
    }
}

#[derive(Debug)]
pub struct Scale(pub Percentage);

impl Default for Scale {
    fn default() -> Scale {
        Scale(Default::default())
    }
}

#[derive(Debug, Default)]
pub struct Content {
    pub scale: Scale,
    pub align: Align,
    pub flow: Flow,
}
