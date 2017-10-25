use ::{Length, Unit};

#[derive(Debug, Copy, Clone)]
pub enum Flex {
    Push,
    Limit(Unit),
}

#[derive(Debug, Copy, Clone)]
pub struct Sizing {
    pub target: Unit,
    pub min: Flex,
    pub max: Flex,
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: Sizing,
    pub height: Sizing,
}

impl Default for Size {
    fn default() -> Self {
        let sizing = Sizing {
            target: Unit::Length(Length(0.0)),
            min: Flex::Limit(Unit::Length(Length(0.0))),
            max: Flex::Limit(Unit::Length(Length(0.0))),
        };

        Size {
            width: sizing,
            height: sizing,
        }
    }
}
