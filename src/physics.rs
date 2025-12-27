use macroquad::prelude::*;
use crate::ball::{Ball};

pub fn apply_gravity(ball: &mut Ball, dt: f32) { // dt is the delta time
    ball.velocity.y += 200.0 * dt; // Gravity acceleration
}

pub fn handle_floor_collision(ball:&mut Ball) {
    let floor_y = screen_height();
    if ball.position.y + ball.radius > floor_y {
        ball.position.y = floor_y - ball.radius;
        ball.velocity.y *= -0.8; // Reverse and dampen velocity
    }
}

pub fn handle_ceiling_collision(ball:&mut Ball) {
    if ball.position.y - ball.radius < 0.0 {
        ball.position.y = ball.radius;
        ball.velocity.y *= -0.8;
    }
}

pub fn handle_wall_collision(ball: &mut Ball) {
    let screen_w = screen_width();
    if ball.position.x - ball.radius < 0.0 {
        ball.position.x = ball.radius;
        ball.velocity.x *= -0.8;
    } else if ball.position.x + ball.radius > screen_w {
        ball.position.x = screen_w - ball.radius;
        ball.velocity.x *= -0.8;
    }
}

pub fn handle_ball_collision(ball1: &mut Ball, ball2: &mut Ball) {
    let delta = ball2.position - ball1.position;
    let dist = delta.length();
    let min_dist = ball1.radius + ball2.radius;

    if dist == 0.0 {
        // prevent exact overlap
        let separation = Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize() * 0.5;
        ball1.position -= separation;
        ball2.position += separation;
        return;
    }

    if dist < min_dist {
        // collision normal
        let normal = delta / dist;

        // Separate balls to remove overlap completely
        let overlap = min_dist - dist;
        let total_mass = ball1.mass + ball2.mass;
        ball1.position -= normal * (overlap * ball2.mass / total_mass);
        ball2.position += normal * (overlap * ball1.mass / total_mass);

        // Relative velocity along normal
        let rel_vel = ball2.velocity - ball1.velocity;
        let vel_along_normal = rel_vel.dot(normal);

        if vel_along_normal > 0.0 {
            return; // balls moving apart
        }

        // Elastic collision with mass
        let impulse = (2.0 * vel_along_normal) / total_mass;
        ball1.velocity += normal * impulse * ball2.mass;
        ball2.velocity -= normal * impulse * ball1.mass;
    }
}


pub fn update_ball(ball: &mut Ball, dt: f32) {
    ball.position += ball.velocity * dt;
    apply_gravity(ball, dt);
    handle_floor_collision(ball);
    handle_wall_collision(ball);
    handle_ceiling_collision(ball);
}