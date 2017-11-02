#[derive(Debug, Clone)]
pub enum Cursor {
    Normal,
    Pointer,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor::Normal
    }
}

#[derive(Debug, Default, Clone)]
pub struct Reactive {
    pub cursor: Cursor,
}
