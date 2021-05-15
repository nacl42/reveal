use macroquad::prelude::*;
//use macroquad::{input, window};

mod effect;
use effect::{TextEffect, ScaleText};

mod tileset;
use tileset::{Tileset, Pattern};

mod layer;
use layer::{generate_layer};

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
    let font = load_ttf_font("assets/DejaVuSerif.ttf").await.unwrap();
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
                if off_y > 0 {
                    off_y -= 1;
                }
            } else if is_key_down(KeyCode::Left) {
                if off_x > 0 {
                    off_x -= 1;
                }
            } else if is_key_down(KeyCode::Right) {
                off_x += 1;
            } else if is_key_down(KeyCode::Down) {
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

        // --- map drawing --
        let base = vec2(10.0, 70.0);
        let sep = vec2(0.0, 0.0);
        let (tiles_x, tiles_y) = (16, 12);
        
        // EXPERIMENTAL: render map to texture, not to screen
        let map_size = vec2(
            (tiles_x as f32 * (width + sep.x)) as f32,
            (tiles_y as f32 * (height + sep.y)) as f32
        );
        let render_target = render_target(
            map_size.x as u32,
            map_size.y as u32
        );
        render_target.texture.set_filter(FilterMode::Nearest);

        let mut camera = Camera2D::from_display_rect(
            Rect::new(0.0, 0.0, map_size.x, map_size.y));
        camera.render_target = Some(render_target);
        set_camera(&camera);

        // draw map onto texture
        let mut py = 0.0;
        for y in 0..tiles_y {
            let mut px = 0.0;
            for x in 0..tiles_x {
                if let Some(index) = layer.get(
                    &(x as i16 + off_x, y as i16 + off_y)
                ) {
                    match tileset.sources.get(*index) {
                        Some(&source) => {
                            draw_texture_ex(
                                tileset.texture, px, py, WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(width, height)),
                                    source: Some(source),
                                    ..Default::default()
                                }
                            )
                        },
                        _ => {}
                    }
                };
                px += width + sep.x;
            }
            py += height + sep.y;
        }

        // draw texture on screen
        set_default_camera();

        draw_texture_ex(
            render_target.texture,
            base.x,
            base.y,
            WHITE,
            DrawTextureParams {
                flip_y: true, // this is a temporary workaround
                ..Default::default()
            }
        );

        // draw text
        draw_text_ex(
            "Reveal - Mystic Land of Magic and Adventure", x, y, params
        );

        next_frame().await
    }
}
