//! The InventoryRenderer serves as a prototype for a complex widget.

use macroquad::prelude::*;

pub struct InventoryWidget {
    destinations: Vec<Rect>
}

impl InventoryWidget {
    pub fn new(pos: Vec2) -> Self {
        let (width, height, sep_x, sep_y) = (32.0, 32.0, 2.0, 2.0);
        let (rows, cols) = (2, 5);
        
        let mut destinations = Vec::<Rect>::new();
        let mut rect = Rect::new(pos.x, pos.y, width, height);

        for _y in 0..rows {
            for _x in 0..cols {
                destinations.push(rect);
                rect.x += width + sep_x;
            }
            rect.x = pos.x;
            rect.y += height + sep_y;
        }

        Self {
            destinations
        }
    }

    pub fn render(&self) {
        for rect in self.destinations.iter() {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 4.0, GRAY);
        }
    }
}
