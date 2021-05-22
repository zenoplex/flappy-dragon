use crate::constants::{SCREEN_HEIGHT, TRANSPARENT};
use crate::player::Player;
use bracket_lib::prelude::{BTerm, RandomNumberGenerator, WHITE};

pub struct Obstacle {
    pub x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: u32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(SCREEN_HEIGHT / 5, SCREEN_HEIGHT / 2),
            size: i32::max(2, 10 - score as i32),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, WHITE, TRANSPARENT, 179);
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, WHITE, TRANSPARENT, 179);
        }
    }

    pub fn is_hit(&mut self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = (player.y as i32) < self.gap_y - half_size;
        let player_below_gap = (player.y as i32) > self.gap_y + half_size;

        does_x_match && (player_above_gap || player_below_gap)
    }
}
