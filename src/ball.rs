use macroquad::prelude::*;

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub color: Color,
}

impl Ball {
    pub fn new(position: Vec2, velocity: Vec2, radius: f32, color: Color) -> Self {
        let mass = (radius * radius * std::f32::consts::PI) * 2.0; // Mass proportional to area
        Self {
            position,
            velocity,
            radius,
            mass,
            color,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}