use macroquad::prelude::*;
//use macroquad::{input, window};

mod effect;
use effect::{TextEffect, ScaleText};

mod tileset;
use tileset::{Tileset, Pattern};

mod layer;

use std::collections::HashMap;
use maplit::hashmap;

mod tile;
use tile::{TileKind, Tile, TileFeature};

fn window_conf() -> Conf {
    Conf {
        window_title: "Reveal".to_owned(),
        window_width: 1024,
        window_height: 800,
        ..Default::default()
    }
}


const CRT_FRAGMENT_SHADER: &'static str =
    include_str!("shaders/vignette_fragment.glsl");

const CRT_VERTEX_SHADER: &'static str =
    include_str!("shaders/vignette_vertex.glsl");

const BW_FRAGMENT_SHADER: &'static str =
    include_str!("shaders/bw_fragment.glsl");


fn tile_class_index(tile: &Tile) -> usize {
    match tile.kind {
        TileKind::Grass => 0,
        TileKind::Hedge => 5,
        TileKind::StoneFloor => 11,
        TileKind::Path => 1,
        TileKind::ThickGrass => 10,
        TileKind::Water => 2,
        TileKind::Wall => 3,
        TileKind::ShallowWater => 12,
        TileKind::Door(_) => 14,
        TileKind::Window => 15,
        _ => 0
    }
}

fn tile_feature_index(tile: &Tile) -> Option<usize> {
    if let Some(feature) = &tile.feature {
        let index = match feature {
            TileFeature::Mushroom => 20,
            TileFeature::Flower(n) => (40 + (n % 4) as usize),
            TileFeature::Stones => 10,
            TileFeature::Waterlily => 30
        };
        Some(index)
    } else {
        None
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("You are in a cave and there is no light.");

    println!("Press <q> to quit and <s> to scale text!");
    println!("Try <b> to switch color vision.");
    println!("...and of course <up>, <down>, <left>, <right> to move the map!");
    
    // load assets
    let font = load_ttf_font("assets/DejaVuSerif.ttf").await.unwrap();
    let mut params = TextParams {
        font,
        font_size: 24,
        color: RED,
        ..Default::default()
    };

    let material_vignette = load_material(
        CRT_VERTEX_SHADER,
        CRT_FRAGMENT_SHADER,
        Default::default()
    ).unwrap();

    let material_bw = load_material(
        CRT_VERTEX_SHADER,
        BW_FRAGMENT_SHADER,
        Default::default()
    ).unwrap();

    let mut is_bw = false;

    // the map render target will be initialised in the main loop
    let mut target: Option<RenderTarget> = None;
        
    // sample text effect (proof of concept)
    let mut effects: Vec<Box<dyn TextEffect>> = vec!();

    // pattern, tileset, layer
    let (width, height) = (32.0, 32.0);
    let pattern = Pattern::Matrix {
        width, height,
        columns: 10,
        rows: 2, 
    };
    let tileset = Tileset::new("assets/tileset32.png", pattern).await.unwrap();

    // feature tileset
    let pattern = Pattern::Matrix {
        width, height,
        columns: 10,
        rows: 9, 
    };
    let tileset_features = Tileset::new(
        "assets/features32.png", pattern
    ).await.unwrap();

    // item tileset
    let pattern = Pattern::Matrix {
        width, height,
        columns: 10,
        rows: 3,
    };
    let tileset_items = Tileset::new("assets/items32.png", pattern).await.unwrap();

    // item map (just an example)
    let item_places: HashMap<_, Vec<_>> = hashmap! {
        (5, 8) => vec![5, 6],
        (6, 8) => vec![5],
        (7, 9) => vec![2],
        (20, 10) => vec![3]
    };
    
    let layer = layer::read_tile_layer_from_file("assets/sample.layer").unwrap();
    let (mut off_x, mut off_y) = (0, 0);
    
    // main loop
    let mut last_update = get_time();
    const DELTA: f64 = 0.01;
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

            if is_key_pressed(KeyCode::B) {
                println!("switching color vision");
                is_bw = !is_bw;
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
        clear_background(BLACK);

        // --- map drawing --
        let base = vec2(10.0, 70.0);
        let sep = vec2(0.0, 0.0);
        let (tiles_x, tiles_y) = (32, 20);
        
        // EXPERIMENTAL: render map to texture, not to screen
        let map_size = vec2(
            (tiles_x as f32 * (width + sep.x)) as f32,
            (tiles_y as f32 * (height + sep.y)) as f32
        );

        // render target for map drawing
        if target.is_none() {
            let _target = render_target(
                map_size.x as u32,
                map_size.y as u32
            );
            _target.texture.set_filter(FilterMode::Nearest);
            target = Some(_target);
        };

        // set camera, so that drawing operations act
        // on the texture
        let mut camera = Camera2D::from_display_rect(
            Rect::new(0.0, 0.0, map_size.x, map_size.y));
        camera.render_target = Some(target.unwrap());
        set_camera(&camera);

        // draw map onto texture
        clear_background(BLACK);

        // background
        let mut py = 0.0;
        for y in 0..tiles_y {
            let mut px = 0.0;
            for x in 0..tiles_x {
                let tile_xy = (x as i16 + off_x, y as i16 + off_y);
                
                // draw tile
                if let Some(tile) = layer.get(&tile_xy) {

                    // draw background
                    let index = tile_class_index(&tile);
                    match tileset.sources.get(index) {
                        Some(&source) => {
                            draw_texture_ex(
                                tileset.texture, px, py, WHITE,
                                DrawTextureParams {
                                    dest_size: Some(
                                        Vec2::new(width, height)
                                    ),
                                    source: Some(source),
                                    ..Default::default()
                                }
                            )
                        },
                        _ => {}
                    }

                    // draw feature (if present)
                    if let Some(index) = tile_feature_index(&tile) {
                        match tileset_features.sources.get(index) {
                            Some(&source) => {
                                draw_texture_ex(
                                    tileset_features.texture, px, py, WHITE,
                                    DrawTextureParams {
                                        dest_size: Some(
                                            Vec2::new(width, height)
                                        ),
                                        source: Some(source),
                                        ..Default::default()
                                    }
                                )
                            },
                            _ => {}
                        }
                    };
                    
                    // draw items
                    if let Some(indices) = item_places.get(&tile_xy) {
                        for index in indices {
                            match tileset_items.sources.get(*index) {
                                Some(&source) => {
                                    draw_texture_ex(
                                        tileset_items.texture, px, py, WHITE,
                                        DrawTextureParams {
                                            dest_size: Some(Vec2::new(width, height)),
                                            source: Some(source),
                                            ..Default::default()
                                        }
                                    )
                                },
                                _ => {}
                            }
                        }
                    }
                }                
                px += width + sep.x;
            }
            py += height + sep.y;
        }

        // draw texture on screen
        set_default_camera();

        
        match is_bw {
            false => gl_use_material(material_vignette),
            true => gl_use_material(material_bw)
        };

        draw_texture_ex(
            target.unwrap().texture,
            base.x,
            base.y,
            WHITE,
            DrawTextureParams {
                flip_y: true, // this is a temporary workaround
                dest_size: Some(map_size),
                ..Default::default()
            }
        );

        gl_use_default_material();

        // draw text
        let mut params2 = params.clone();
        params2.color = LIGHTGRAY;

        draw_text_ex(
            "Reveal - Mystic Land of Magic and Adventure", x+1.0, y+1.0, params2
        );

        draw_text_ex(
            "Reveal - Mystic Land of Magic and Adventure", x, y, params
        );

        next_frame().await
    }
}


