//! A Pattern defines e.g. how the source images are taken from a Tileset.
//! The Pattern::Matrix defines a rectangular matrix of source rects
//! width a given `width` and `height` for each rect.
//!


use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub enum Pattern {
    Matrix { rows: u16, cols: u16, width: f32, height: f32 },
    MatrixWithGaps { rows: u16, cols: u16, width: f32, height: f32, sep_x: f32, sep_y: f32 },
}

impl Pattern {
    #[allow(dead_code)]
    pub fn width(&self) -> f32 {
        match self {
            Pattern::Matrix { width, .. } => width.clone(),
            Pattern::MatrixWithGaps { width, sep_x, .. } => width + sep_x,
        }
    }

    #[allow(dead_code)]
    pub fn height(&self) -> f32 {
        match self {
            Pattern::Matrix { height, .. } => height.clone(),
            Pattern::MatrixWithGaps { height, sep_y, .. } => height + sep_y,
        }
    }

    pub fn all_rects(&self) -> Vec<Rect> {
        let mut rects: Vec<Rect> = vec!();
        match self {
            &Pattern::Matrix { rows, cols, width, height } => {
                for y in 0..rows {
                    for x in 0..cols {
                        rects.push(Rect::new(
                            x as f32 * width, y as f32 * height,
                            width, height
                        ));
                    }
                }
            },
            &Pattern::MatrixWithGaps { rows, cols, width, height, sep_x, sep_y } => {
                for y in 0..rows {
                    for x in 0..cols {
                        rects.push(Rect::new(
                            x as f32 * (width+sep_x), y as f32 * (height+sep_y),
                            width, height
                        ));
                    }
                }
            }

        }
        rects
    }
}

