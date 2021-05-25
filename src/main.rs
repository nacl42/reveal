use macroquad::prelude::*;

mod actor;
mod effect;
mod tileset;
mod terrain;
mod point;
mod item;
mod world;
mod flake;
mod id;
mod idmap;

use effect::{TextEffect, ScaleText};
use tileset::{Tileset, Pattern};
use terrain::{TerrainKind, Terrain, TerrainFeature, TerrainMap};
use actor::{Actor}; //, ActorKind, ActorId, ActorMap};
use point::Point;
use item::{ItemKind}; //ItemId, Item, ItemKind, ItemMap};
use world::World;
use item::Item;

const CRT_FRAGMENT_SHADER: &'static str = include_str!("shaders/vignette_fragment.glsl");
const CRT_VERTEX_SHADER: &'static str = include_str!("shaders/vignette_vertex.glsl");
const BW_FRAGMENT_SHADER: &'static str = include_str!("shaders/bw_fragment.glsl");


fn window_conf() -> Conf {
    Conf {
        window_title: "Reveal".to_owned(),
        window_width: 1024,
        window_height: 800,
        ..Default::default()
    }
}


fn terrain_class_index(tile: &Terrain) -> usize {
    match tile.kind {
        TerrainKind::Grass => 0,
        TerrainKind::Hedge => 5,
        TerrainKind::StoneFloor => 11,
        TerrainKind::Path => 1,
        TerrainKind::ThickGrass => 10,
        TerrainKind::Water => 2,
        TerrainKind::Wall => 3,
        TerrainKind::ShallowWater => 12,
        TerrainKind::Door(_) => 14,
        TerrainKind::Window => 15,
        _ => 0
    }
}

fn terrain_feature_index(tile: &Terrain) -> Option<usize> {
    if let Some(feature) = &tile.feature {
        let index = match feature {
            TerrainFeature::Mushroom => 20,
            TerrainFeature::Flower(n) => (40 + (n % 4) as usize),
            TerrainFeature::Stones => 10,
            TerrainFeature::Waterlily => 30
        };
        Some(index)
    } else {
        None
    }
}

fn item_index(item: &Item) -> usize {
    match item.kind {
        ItemKind::Money(_) => 1,
        ItemKind::Wand => 2
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    println!("You are in a cave and there is no light.");

    println!("Press <q> to quit and <t> to scale text!");
    println!("Try <b> to switch color vision.");
    println!("Move player with <A>, <S>, <D>, <W>.");
    println!("List inventory with <I>, pick up items with <P>.");
    println!("...and of course <up>, <down>, <left>, <right> to move the map!");
    
    // load assets
    let font = load_ttf_font("assets/DejaVuSerif.ttf").await.unwrap();
    let mut params = TextParams {
        font,
        font_size: 24,
        color: RED,
        ..Default::default()
    };

    let params_info = TextParams {
        font,
        font_size: 16,
        color: WHITE,
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

    // pattern, tileset
    let (width, height) = (32.0, 32.0);
    let pattern = Pattern::Matrix { width, height, columns: 10, rows: 2 };
    let tileset_terrain = Tileset::new("assets/tileset32.png", pattern).await.unwrap();

    // feature tileset
    let pattern = Pattern::Matrix { width, height, columns: 10, rows: 9 };
    let tileset_features = Tileset::new(
        "assets/features32.png", pattern
    ).await.unwrap();

    // item tileset
    let pattern = Pattern::Matrix { width, height, columns: 10, rows: 3 };
    let tileset_items = Tileset::new("assets/items32.png", pattern).await.unwrap();

    // actor tileset
    let pattern = Pattern::Matrix { width, height, columns: 10, rows: 1 };
    let tileset_actors = Tileset::new("assets/actors32.png", pattern).await.unwrap();

    let (mut off_x, mut off_y) = (0, 0);

    // the World contains the actual game data
    // all of the above will be moved into the World, one by one
    let mut world = World::new();
    world.populate_world();

    // main loop
    let mut last_update = get_time();
    const DELTA: f64 = 0.01;
    let (title_x, title_y) = (10.0, 42.0);
    
    loop {
        // update, if necessary
        if get_time() - last_update > DELTA {
            last_update = get_time();
            //y += 1.0;

            // Q => quit
            if is_key_down(KeyCode::Q) {
                println!("GOODBYE");
                break;
            }

            // I => inventory
            if is_key_pressed(KeyCode::I) {
                println!("Inventory:");
                if let Some(player) = world.actors.get(&world.player_id()) {
                    for (n, item_id) in player.inventory.iter().enumerate() {
                        if let Some(item) = world.items.get(item_id) {
                            println!("{} - {}", n, item.description());
                        }
                    }
                }
            }

            // B => switch black/white and color mode
            if is_key_pressed(KeyCode::B) {
                println!("switching color vision");
                is_bw = !is_bw;
            }

            // arrows keys => scroll map
            if is_key_down(KeyCode::Up) {
                if off_y > 0 {
                    off_y -= 1;
                }
            }

            if is_key_down(KeyCode::Left) {
                if off_x > 0 {
                    off_x -= 1;
                }
            }

            if is_key_down(KeyCode::Right) {
                off_x += 1;
            }

            if is_key_down(KeyCode::Down) {
                off_y += 1;
            }

            // ASDW => move player
            fn move_if_not_blocked<P>(player: &mut Actor, offset: P, terrain: &TerrainMap)
            where P: Into<Point>
            {
                let new_pos = player.pos + offset.into();
                if let Some(terrain) = terrain.get(&new_pos) {
                    if !terrain.is_blocking() {
                        player.pos = new_pos;
                    }
                }
            }
            let player_id = world.player_id();
            let actors = &mut world.actors;
  
            if is_key_pressed(KeyCode::A) {
                if let Some(mut player) = actors.get_mut(&player_id) {
                    move_if_not_blocked(&mut player, (-1, 0), &world.terrain);
                }
            }
            if is_key_pressed(KeyCode::W) {
                if let Some(mut player) = actors.get_mut(&player_id) {
                    move_if_not_blocked(&mut player, (0, -1), &world.terrain);
                }
            }
            if is_key_pressed(KeyCode::D) {
                if let Some(mut player) = actors.get_mut(&player_id) {
                    move_if_not_blocked(&mut player, (1, 0), &world.terrain);
                }
            }
            if is_key_pressed(KeyCode::S) {
                if let Some(mut player) = actors.get_mut(&player_id) {
                    move_if_not_blocked(&mut player, (0, 1), &world.terrain);
                }
            }

            
            // T => show off text effect
            if is_key_pressed(KeyCode::T) {
                if effects.len() == 0 {
                    effects.push(Box::new(ScaleText::new()));
                }
            }

            // P => pick up items
            if is_key_pressed(KeyCode::P) {
                let player_id = world.player_id();
                if let Some(player) = world.actors.get(&player_id) {
                    for id in &world.item_ids_at(&player.pos) {
                        let item = world.items.get_mut(&id).unwrap();
                        println!("pickung up {}", item.description());
                        item.owner = Some(player_id);
                        item.pos = None;
                        world.actors.get_mut(&player_id).unwrap()
                            .inventory.push(*id);
                    }                    
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
        
        // render target for map drawing
        let map_size = vec2(
            (tiles_x as f32 * (width + sep.x)) as f32,
            (tiles_y as f32 * (height + sep.y)) as f32
        );

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
                let tile_xy = Point::from((x as i32 + off_x, y as i32 + off_y));
                
                // draw tile
                if let Some(terrain) = world.terrain.get(&tile_xy) {

                    // draw background
                    let index = terrain_class_index(&terrain);
                    if let Some(&source) = tileset_terrain.sources.get(index) {
                        draw_texture_ex(
                            tileset_terrain.texture,
                            px, py, WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(width, height)),
                                source: Some(source),
                                ..Default::default()
                            }
                        )
                    }

                    // draw feature (if present)
                    if let Some(index) = terrain_feature_index(&terrain) {
                        if let Some(&source) = tileset_features.sources.get(index) {
                            draw_texture_ex(
                                tileset_features.texture,
                                px, py, WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(width, height)),
                                    source: Some(source),
                                    ..Default::default()
                                }
                            )
                        }
                    }
                    
                    // draw items
                    let items = world.item_ids_at(&tile_xy);
                    for index in items {                        
                        let mut tileset_index = 0;
                        if let Some(item) = world.items.get(&index) {
                            tileset_index = item_index(&item);
                        };

                        if let Some(&source) =
                            tileset_items.sources.get(tileset_index)
                        {
                            draw_texture_ex(
                                tileset_items.texture,
                                px, py, WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(width, height)),
                                    source: Some(source),
                                    ..Default::default()
                                }
                            )
                        }
                    }

                    // draw actors
                    for _ in world.actors.iter()
                        .filter(|(_, actor)| actor.pos == tile_xy) {
                            let index = 2; // TODO: get index from actor
                            if let Some(&source) = tileset_actors.sources.get(index) {
                                draw_texture_ex(
                                    tileset_actors.texture,
                                    px, py, WHITE,
                                    DrawTextureParams {
                                        dest_size: Some(Vec2::new(width, height)),
                                        source: Some(source),
                                        ..Default::default()
                                    }
                                )
                            }
                        }
                }                
                px += width + sep.x;
            }
            py += height + sep.y;
        }

        // draw texture on screen
        set_default_camera();

        // select material (this is just a toy function for testing)
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

        // draw text with shadow
        let mut params2 = params.clone();
        params2.color = LIGHTGRAY;

        draw_text_ex(
            "Reveal - Mystic Land of Magic and Adventure", title_x+1.0, title_y+1.0, params2
        );

        draw_text_ex(
            "Reveal - Mystic Land of Magic and Adventure", title_x, title_y, params
        );

        // display status information
        if let Some(player) = world.actors.get(&world.player_id()) {
            // actor position
            let text = format!("{}, {}", player.pos.x, player.pos.y);
            let pos = base + Vec2::from((0.0, map_size.y + 24.0 + 10.0));
            draw_text_ex(&text, pos.x, pos.y, params_info);

            // names of items at spot
            let ids = world.item_ids_at(&player.pos);
            let names = ids.iter()
                .map(|id| world.items.get(id).unwrap())
                .map(|item| item.description())
                .collect::<Vec<String>>();
            let text = names.join(", ");
            if text.len() > 0 {
                let pos = pos + Vec2::from((80.0, 0.0));
                draw_text_ex(&text, pos.x, pos.y, params_info);

                let text = "use <p> to pick up the items";
                let pos = pos + Vec2::from((0.0, 30.0));
                draw_text_ex(&text, pos.x, pos.y, params_info);
            }
        }

        next_frame().await
    }
}


