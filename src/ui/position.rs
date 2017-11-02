use super::Unit;

#[derive(Debug, Clone, Copy)]
pub enum VerticalHook {
    Top(Unit),
    Bottom(Unit),
}

#[derive(Debug, Clone, Copy)]
pub enum HorizontalHook {
    Left(Unit),
    Right(Unit),
}

#[derive(Debug, Clone, Copy)]
pub enum SingleHook {
    Top(Unit),
    Bottom(Unit),
    Left(Unit),
    Right(Unit),
}

#[derive(Debug, Clone, Copy)]
pub enum Hook {
    Single(SingleHook),
    Double {
        vertical: VerticalHook,
        horizontal: HorizontalHook,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Passthrough,
    Anchor,
    Hook(Hook),
}

impl Default for Position {
    fn default() -> Self {
        Position::Passthrough
    }
}
