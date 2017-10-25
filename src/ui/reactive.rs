#[derive(Debug)]
pub enum Cursor {
    Normal,
    Pointer,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor::Normal
    }
}

#[derive(Debug, Default)]
pub struct Reactive {
    pub cursor: Cursor,
}
