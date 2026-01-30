use crate::ball::{Ball};
use crate::physics;
use macroquad::prelude::*;

pub struct Simulation {
    pub balls: Vec<Ball>,
}

impl Simulation {
    pub fn new() -> Self {
        Self { balls: vec![] }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn reset(&mut self) {
        self.balls.clear();
    }

    pub fn update(&mut self, dt: f32) {
        for ball in &mut self.balls {
            physics::update_ball(ball, dt);
        }

        self.handle_collisions();
    }

    pub fn draw(&self) {
        for ball in &self.balls {
            ball.draw();
        }
        draw_text(&format!("Balls: {}", self.balls.len()), 10.0, 20.0, 20.0, WHITE);
        draw_text("Click & Drag: Spawn Ball", 550.0, 20.0, 20.0, WHITE);
        draw_text("Space: Reset", 550.0, 40.0, 20.0, WHITE);
        draw_text("Escape: Close Sim", 550.0, 60.0, 20.0, WHITE);
        
    }

    pub fn spawn_ball(&mut self, position: Vec2, velocity: Vec2) {
        let radius = rand::gen_range(10.0, 30.0);
        let color = Color::new(
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            1.0,
        );
        let ball = Ball::new(position, velocity, radius, color);
        self.add_ball(ball);
    }

    pub fn handle_collisions(&mut self) {
        let len = self.balls.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let (left, right) = self.balls.split_at_mut(j);
                let ball1 = &mut left[i];
                let ball2 = &mut right[0];
                physics::handle_ball_collision(ball1, ball2);
            }
        }
    }
}