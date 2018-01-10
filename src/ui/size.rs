use super::{Length, Unit};

#[derive(Debug, Clone, Copy)]
pub enum Flex {
    Push,
    Limit(Unit),
}

#[derive(Debug, Clone, Copy)]
pub struct Sizing {
    pub target: Unit,
    pub min: Flex,
    pub max: Flex,
}

impl Sizing {
    pub fn target(&mut self, target: Unit) {
        self.target = target;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: Sizing,
    pub height: Sizing,
}

impl Default for Size {
    fn default() -> Self {
        let sizing = Sizing {
            target: Unit::spx(0.0),
            min: Flex::Limit(Unit::spx(0.0)),
            max: Flex::Limit(Unit::spx(1.0 / 0.0)),
        };

        Size {
            width: sizing,
            height: sizing,
        }
    }
}
