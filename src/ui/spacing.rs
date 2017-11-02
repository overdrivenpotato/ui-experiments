use super::{Length, Quadruple};

#[derive(Debug, Default, Clone, Copy)]
pub struct Spacing {
    pub outer: Quadruple<Length>,
    pub inner: Quadruple<Length>,
}
