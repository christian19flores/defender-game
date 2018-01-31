extern crate graphics;
extern crate opengl_graphics;

use graphics::{Context, ellipse, Transformed};
use opengl_graphics::GlGraphics;

use color;
use super::GameObject;

pub struct Bullet {
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

const BULLET_SIZE: f64 = 5.0;

impl Bullet {
    pub fn new(x: f64, y: f64) -> Bullet {
        return Bullet { x, y, size: BULLET_SIZE };
    }

    pub fn radius(&self) -> f64 {
        return self.size / 2.0;
    }
}

impl GameObject for Bullet {
    fn animate(&mut self, dt: f64) {
        self.x += 1.0;
    }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        let transform = ctxt.transform.trans(self.x, self.y);
        ellipse(color::WHITE, [0.0, 0.0, self.size, self.size], transform, gl);
    }
}