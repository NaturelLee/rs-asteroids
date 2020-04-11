mod asteroid;
mod blast;
pub mod font;
pub mod geometry;
pub mod iter;
mod level;
pub mod motion;
mod particle;
mod player;
mod util;

pub use asteroid::Asteroid;
pub use blast::Blast;
pub use level::Level;
pub use particle::{Dispersion, Particle};
pub use player::{Controls, Player};

use geometry::Size;

pub struct Game {
    level: Level,
}

impl Game {
    pub fn bounds() -> Size {
        Size {
            width: 1200.0,
            height: 900.0,
        }
    }

    pub fn new() -> Self {
        Game {
            level: Level::new(1, Game::bounds()),
        }
    }

    pub fn step(&mut self, dt: f64, controls: Controls) -> () {
        if dt <= 0.0 {
            return ();
        }
        self.level.step(dt, controls);
    }

    pub fn player(&self) -> &Option<Player> {
        &self.level.player
    }
    pub fn asteroids(&self) -> &Vec<Asteroid> {
        &self.level.asteroids
    }
    pub fn blasts(&self) -> &Vec<Blast> {
        &self.level.blasts
    }
    pub fn particles(&self) -> &Vec<Particle> {
        &self.level.particles
    }
}
