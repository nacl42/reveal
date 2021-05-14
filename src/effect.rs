use macroquad::prelude::*;

pub trait TextEffect {
    fn apply(&self, params: &mut TextParams);
    fn step(&mut self);
    fn is_alive(&self) -> bool { true }
}


pub struct ScaleText {
    initial_size: u16,
    scale: f32,
    step_factor: f32,
    alive: bool
}

impl ScaleText {
    pub fn new() -> ScaleText {
        ScaleText {
            initial_size: 24,
            scale: 1.0,
            step_factor: 1.05,
            alive: true                
        }
    }
}

impl TextEffect for ScaleText {
    fn apply(&self, params: &mut TextParams) {
        params.font_size = (self.initial_size as f32 * self.scale) as u16;
    }

    fn step(&mut self) {
        let new_scale = self.scale * self.step_factor;
        if new_scale > 2.0 {
            self.step_factor = 0.95;
            self.scale = new_scale;
        } else if new_scale < 1.0 {
            self.alive = false
        } else {
            self.scale = new_scale
        }
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}
