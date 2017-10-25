use ::{EdgeMode, Length, Quadruple, Color, Unit};

#[derive(Debug, Default)]
pub struct Border {
    pub mode: EdgeMode,
    pub width: Length,
    pub color: Color,
    pub radius: Quadruple<Unit>,
}
