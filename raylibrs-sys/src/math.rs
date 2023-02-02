use glam::Vec2;
use crate::Vector2;



impl From<Vector2> for Vec2 {
    fn from(value: Vector2) -> Self {
        Vec2 { x: value.x, y: value.y }
    }
}

impl From<Vec2> for Vector2 {
    fn from(value: Vec2) -> Self {
        Vector2 { x: value.x, y: value.y }
    }
}