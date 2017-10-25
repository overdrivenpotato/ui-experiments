use ::{Length, Percentage, Unit};

#[derive(Debug, Default)]
pub struct Opacity(pub Percentage);

#[derive(Debug, Default)]
pub struct Depth(pub Length);

#[derive(Debug, Default)]
pub struct Translation {
    pub x: Unit,
    pub y: Unit,
    pub z: Unit,
}

#[derive(Debug, Default)]
pub struct Angle(f32);

#[derive(Debug, Default)]
pub struct Rotation {
    pub axis: [Unit; 3],
    pub angle: Angle,
}

#[derive(Debug, Default)]
pub struct Scale(pub Percentage);

#[derive(Debug, Default)]
pub struct Render {
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
    pub depth: Depth,
    pub opacity: Opacity,
}
