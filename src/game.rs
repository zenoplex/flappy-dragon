use crate::constants::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::obstacle::Obstacle;
use crate::player::Player;
use bracket_lib::prelude::{BTerm, GameState, VirtualKeyCode, BLACK, CYAN, NAVY, RED, YELLOW};

#[derive(Debug)]
enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: u32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.end(ctx),
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            player: Player::new(5, SCREEN_HEIGHT / 2),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, "Welcome to Flappy Dragon");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score {}", self.score));
        if self.player.y as i32 > SCREEN_HEIGHT || self.obstacle.is_hit(&self.player) {
            self.mode = GameMode::End;
        }
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
    }

    fn end(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, RED, BLACK, "You are dead");
        ctx.print_centered(6, &format!("You earned {} points", &self.score));
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Again");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(5, SCREEN_HEIGHT / 2);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
}
