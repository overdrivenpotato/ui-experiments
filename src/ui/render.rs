use super::{Length, Percentage, Unit};

#[derive(Debug, Default, Clone, Copy)]
pub struct Opacity(pub Percentage);

#[derive(Debug, Default, Clone, Copy)]
pub struct Depth(pub Length);

#[derive(Debug, Default, Clone, Copy)]
pub struct Translation {
    pub x: Unit,
    pub y: Unit,
    pub z: Unit,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Angle(f32);

#[derive(Debug, Default, Clone, Copy)]
pub struct Rotation {
    pub axis: [Unit; 3],
    pub angle: Angle,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Scale(pub Percentage);

#[derive(Debug, Default, Clone, Copy)]
pub struct Render {
    pub translation: Translation,
    pub rotation: Rotation,
    pub scale: Scale,
    pub depth: Depth,
    pub opacity: Opacity,
}
