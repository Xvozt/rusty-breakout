use macroquad::prelude::*;

#[macroquad::main("Breakout")]
async fn main() {
    let mut paddle_pos = vec2(screen_width() / 2.0 - 60.0, screen_height() - 50.0);
    let paddle_size = vec2(120.0, 20.0);
    let paddle_speed = 500.0;

    let ball_size = vec2(20.0, 20.0);
    let mut ball_pos = paddle_pos + vec2(paddle_size.x / 2.0 - ball_size.x / 2.0, -ball_size.y);
    let mut ball_velocity = vec2(0.0, 0.0);
    let ball_speed = 350.0;

    let mut ball_stuck_to_paddle = true;

    loop {
        if is_key_down(KeyCode::Left) && paddle_pos.x > 0.0 {
            paddle_pos.x -= paddle_speed * get_frame_time();
        }
        if is_key_down(KeyCode::Right) && paddle_pos.x < screen_width() - paddle_size.x {
            paddle_pos.x += paddle_speed * get_frame_time();
        }

        if ball_stuck_to_paddle && is_key_pressed(KeyCode::Space) {
            ball_velocity = vec2(ball_speed, -ball_speed);
            ball_stuck_to_paddle = false;
        }

        if ball_stuck_to_paddle {
            ball_pos = paddle_pos + vec2(paddle_size.x / 2.0 - ball_size.x / 2.0, -ball_size.y);
        } else {
            ball_pos += ball_velocity * get_frame_time();
        }

        clear_background(WHITE);

        draw_rectangle(
            paddle_pos.x,
            paddle_pos.y,
            paddle_size.x,
            paddle_size.y,
            BLUE,
        );

        draw_rectangle(ball_pos.x, ball_pos.y, ball_size.x, ball_size.y, RED);

        next_frame().await;
    }
}
