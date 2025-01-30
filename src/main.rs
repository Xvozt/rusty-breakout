use macroquad::{input::KeyCode, prelude::*};

const PLAYER_SIZE: Vec2 = vec2(150f32, 40f32);
const PLAYER_SPEED: f32 = 700f32;
const BRICK_SIZE: Vec2 = vec2(100f32, 40f32);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }

    pub fn update(&mut self, dt: f32) {
        let horizontal_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += horizontal_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
}

struct Brick {
    rect: Rect,
}

impl Brick {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BRICK_SIZE.x, BRICK_SIZE.y),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GREEN);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), -1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }

        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }

        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }
}

#[macroquad::main("Breakout")]
async fn main() {
    let mut player = Player::new();
    let mut bricks = Vec::new();
    let mut balls = Vec::new();

    let (width, height) = (6, 5);
    let padding = 5f32;
    let brick_total_size = BRICK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (brick_total_size.x * width as f32)) * 0.5f32,
        50f32,
    );

    for i in 0..width * height {
        let brick_x = (i % width) as f32 * brick_total_size.x;
        let brick_y = (i / width) as f32 * brick_total_size.y;
        bricks.push(Brick::new(board_start_pos + vec2(brick_x, brick_y)));
    }

    balls.push(Ball::new(vec2(
        screen_width() * 0.5f32 - BALL_SIZE * 0.5f32,
        screen_height() - 100f32 - PLAYER_SIZE.y,
    )));

    loop {
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new(vec2(
                screen_width() * 0.5f32 - BALL_SIZE * 0.5f32,
                screen_height() - 100f32 - PLAYER_SIZE.y,
            )));
        }
        player.update(get_frame_time());
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }
        clear_background(WHITE);
        player.draw();
        for brick in bricks.iter() {
            brick.draw();
        }

        for ball in balls.iter() {
            ball.draw();
        }
        next_frame().await;
    }
}
