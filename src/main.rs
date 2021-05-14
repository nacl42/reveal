use macroquad::prelude::*;
use macroquad::{input, window};

mod effect;
use effect::{TextEffect, ScaleText};

mod tileset;
use tileset::{Tileset, Pattern};

mod layer;
use layer::{Layer, generate_layer};

fn window_conf() -> Conf {
    Conf {
        window_title: "Reveal".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("You are in a cave and there is no light.");

    println!("Press <q> to quit and <s> to scale text!");
    println!("...and of course <up>, <down>, <left>, <right> to move the map!");
    
    // load assets
    let font = load_ttf_font("assets/DejaVuSerif.ttf").await;
    let mut params = TextParams {
        font,
        font_size: 24,
        color: RED,
        ..Default::default()
    };

    // sample text effect (proof of concept)
    let mut effects: Vec<Box<dyn TextEffect>> = vec!();

    // pattern, tileset, layer
    let (width, height) = (32.0, 32.0);
    let pattern = Pattern::Matrix {
        width, height,
        rows: 2, columns: 2
    };
    let tileset = Tileset::new("assets/tileset32.png", &pattern).await.unwrap();
    let layer = generate_layer();
    let (mut off_x, mut off_y) = (0, 0);
    
    // main loop
    let mut last_update = get_time();
    const DELTA: f64 = 0.05;
    let (x, mut y) = (10.0, 42.0);
    
    loop {
        // update, if necessary
        if get_time() - last_update > DELTA {
            last_update = get_time();
            //y += 1.0;

            if is_key_down(KeyCode::Q) {
                println!("GOODBYE");
                break;
            }

            if is_key_down(KeyCode::Up) {
                if (off_y > 0) {
                    off_y -= 1;
                }
            }

            if is_key_down(KeyCode::Left) {
                if (off_x > 0) {
                    off_x -= 1;
                }
            }

            if is_key_down(KeyCode::Right) {
                off_x += 1;
            }

            if is_key_down(KeyCode::Down) {
                off_y += 1;
            }

            if is_key_down(KeyCode::S) {
                if effects.len() == 0 {
                    effects.push(Box::new(ScaleText::new()));
                }
            }
            
        }

        // update and apply effects
        effects.iter_mut().for_each(|e| e.step());
        effects.retain(|e| e.is_alive());
        //println!("#effects = {}", effects.len());
        effects.iter().for_each(|e| e.apply(&mut params));

        // redraw
        clear_background(WHITE);

        // draw map
        let (base_x, base_y) = (10.0, 70.0);
        let (sep_x, sep_y) = (0.0, 0.0);
        let (tiles_x, tiles_y) = (16, 12);
        let mut py = base_y;
        
        for y in 0..tiles_y {
            let mut px = base_x;
            for x in 0..tiles_x {
                if let Some(index) = layer.get(
                    &(x as i16 + off_x, y as i16 + off_y)
                ) {
                    match tileset.sources.get(*index) {
                        Some(&source) => {
                            let mut params = DrawTextureParams {
                                dest_size: Some(Vec2::new(width, height)),
                                source: Some(source),
                                ..Default::default()
                            };

                            draw_texture_ex(
                                tileset.texture, px, py, WHITE,
                                params
                            )
                        },
                        _ => {}
                    }
                };
                px += width + sep_x;
            }
            py += height + sep_y;
        }

        // draw text
        draw_text_ex(
            "Reveal - Mystic Land of Magic and Adventure", x, y, params
        );

        next_frame().await
    }
}
