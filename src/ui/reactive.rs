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

#[derive(Clone, Default, Debug)]
pub struct Subset {
    pub shadow: super::shadow::Shadow,
    pub spacing: super::spacing::Spacing,
}

#[derive(Clone, Default, Debug)]
pub struct Keyframes {
    pub hover: Option<Subset>,
}

impl Keyframes {
    pub fn hover(&mut self, subset: Subset) {
        self.hover = Some(subset);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Reactive {
    pub cursor: Cursor,
    pub keyframes: Keyframes,
}

impl Reactive {
    pub fn cursor(&mut self, cursor: Cursor) {
        self.cursor = cursor;
    }
}
