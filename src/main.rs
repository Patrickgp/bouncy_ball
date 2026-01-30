mod ball;
mod physics;
mod simulation;

use macroquad::prelude::*;
use ball::Ball;
use simulation::Simulation;

#[macroquad::main("Bouncing Balls Simulation")]
async fn main() {
    let mut sim = Simulation::new();

    // Add an initial ball
    sim.add_ball(Ball::new(vec2(100.0, 100.0), vec2(50.0, 0.0), 20.0, GREEN));

    // Track if mouse is dragging
    let mut drag_start: Option<Vec2> = None;

    loop {
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            sim.reset();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            drag_start = Some(mouse_position().into());
        }

        if is_mouse_button_released(MouseButton::Left) {
            if let Some(start) = drag_start {
                let end:Vec2 = mouse_position().into();
                let velocity = (end - start) * 5.0; // Scale velocity
                sim.spawn_ball(start, velocity);
            }
            drag_start = None;
        }

        sim.update(dt);

        clear_background(BLACK);
        sim.draw();

        if let Some(start) = drag_start {
            let end: Vec2 = mouse_position().into();
            draw_line(start.x, start.y, end.x, end.y, 2.0, WHITE);
        }

        next_frame().await;
    }
}