use super::{Length, Quadruple};

#[derive(Debug, Default, Clone, Copy)]
pub struct Spacing {
    pub outer: Quadruple<Length>,
    pub inner: Quadruple<Length>,
}

impl Spacing {
    pub fn outer<T, U>(&mut self, t: T)
    where
        T: Into<(U, U, U, U)>,
        U: Into<Length>,
    {
        let (a, b, c, d) = t.into();

        self.outer = (a.into(), b.into(), c.into(), d.into()).into();
    }
}
