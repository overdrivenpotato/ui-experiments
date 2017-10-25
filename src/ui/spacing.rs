use ::{Length, Quadruple};

#[derive(Debug, Default)]
pub struct Spacing {
    pub outer: Quadruple<Length>,
    pub inner: Quadruple<Length>,
}
