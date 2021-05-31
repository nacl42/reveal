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
mod action;

use effect::{TextEffect, ScaleText};
use tileset::{Tileset, Pattern};
use terrain::{TerrainKind, Terrain, TerrainFeature, TerrainMap};
use actor::{Actor, ActorId}; //, ActorKind, ActorId, ActorMap};
use point::{Point, Rectangle};
use item::{ItemKind}; //ItemId, Item, ItemKind, ItemMap};
use world::{World, ViewportMode, adjust_viewport};
use item::Item;
use action::Action;

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
        TerrainKind::Grass => 1,
        TerrainKind::Path => 2,
        TerrainKind::Water => 3,
        TerrainKind::Wall => 4,
        //Sand => 5,
        TerrainKind::Hedge => 6,
        TerrainKind::ThickGrass => 10,
        TerrainKind::StoneFloor => 11,
        TerrainKind::ShallowWater => 12,
        // Grate => 13,
        TerrainKind::Door(_) => 14,
        TerrainKind::Window => 15,
        _ => 0,
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


pub struct MapRenderAssets {
    tile_width: f32,
    tile_height: f32,
    tileset_terrain: Option<Tileset>,
    tileset_features: Option<Tileset>,
    tileset_actors: Option<Tileset>,
    tileset_items: Option<Tileset>
}

fn render_map(target: &mut RenderTarget,
              world: &World,
              off_x: i32, off_y: i32,
              tiles_x: i32, tiles_y: i32,
              assets: &MapRenderAssets)
{
    let sep = vec2(0.0, 0.0);
        
    // render target for map drawing
    let map_size = vec2(
        (tiles_x as f32 * (assets.tile_width + sep.x)) as f32,
        (tiles_y as f32 * (assets.tile_height + sep.y)) as f32
    );

    if (target.texture.width() != map_size.x) ||
        (target.texture.height() != map_size.y) {
            *target = render_target(map_size.x as u32, map_size.y as u32);
        }
    target.texture.set_filter(FilterMode::Nearest);

    // set camera, so that drawing operations act
    // on the texture
    let mut camera = Camera2D::from_display_rect(
        Rect::new(0.0, 0.0, map_size.x, map_size.y));
    camera.render_target = Some(*target);
    set_camera(&camera);

    // draw map onto texture
    clear_background(BLACK);

    // background
    let mut py = 0.0;
    for y in 0..tiles_y {
        let mut px = 0.0;
        for x in 0..tiles_x {
            let tile_xy = Point::from((x as i32 + off_x, y as i32 + off_y));
                
            // draw terrain
            if let Some(terrain) = world.terrain.get(&tile_xy) {
                // draw terrain base tile
                if let Some(tileset) = &assets.tileset_terrain {
                    let index = terrain_class_index(&terrain);
                    if let Some(&source) = tileset.sources.get(index) {
                        draw_texture_ex(
                            tileset.texture,
                            px, py, WHITE,
                            DrawTextureParams {
                                dest_size: Some(Vec2::new(assets.tile_width, assets.tile_height)),
                                source: Some(source),
                                ..Default::default()
                            }
                        )
                    }
                }

                // draw terrain feature (if present)
                if let Some(tileset) = &assets.tileset_features {
                    if let Some(index) = terrain_feature_index(&terrain) {
                        if let Some(&source) = tileset.sources.get(index) {
                            draw_texture_ex(
                                tileset.texture,
                                px, py, WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(assets.tile_width, assets.tile_height)),
                                    source: Some(source),
                                    ..Default::default()
                                }
                            )
                        }
                    }
                }
                    
                // draw items
                if let Some(tileset) = &assets.tileset_items {
                    let items = world.item_ids_at(&tile_xy);
                    for index in items {                        
                        let mut tileset_index = 0;
                        if let Some(item) = world.items.get(&index) {
                            tileset_index = item_index(&item);
                        };
                    
                        if let Some(&source) =
                            tileset.sources.get(tileset_index)
                        {
                            draw_texture_ex(
                                tileset.texture,
                                px, py, WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(assets.tile_width, assets.tile_height)),
                                    source: Some(source),
                                    ..Default::default()
                                }
                            )
                        }
                    }
                }

                // draw actors
                if let Some(tileset) = &assets.tileset_actors {
                    for _ in world.actors.iter()
                        .filter(|(_, actor)| actor.pos == tile_xy) {
                            let index = 2; // TODO: get index from actor
                            if let Some(&source) = tileset.sources.get(index) {
                                draw_texture_ex(
                                    tileset.texture,
                                    px, py, WHITE,
                                    DrawTextureParams {
                                        dest_size: Some(Vec2::new(assets.tile_width, assets.tile_height)),
                                        source: Some(source),
                                        ..Default::default()
                                    }
                                )
                            }
                        }
                }
            }                
            px += assets.tile_width + sep.x;
        }
        py += assets.tile_height + sep.y;
    }
    // draw texture on screen
    set_default_camera();
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("You are in a cave and there is no light.");

    println!("Press <q> to quit and <t> to scale text!");
    println!("Try <b> to switch color vision.");
    println!("Move player with <A>, <S>, <D>, <W>.");
    println!("Center map on player using <C>!");
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
    let mut main_map_target = render_target(0, 0);
    let mut mini_map_target = render_target(0, 0);
    
    // sample text effect (proof of concept)
    let mut effects: Vec<Box<dyn TextEffect>> = vec!();

    let (width, height) = (32.0, 32.0);
    let pattern = Pattern::Matrix {
        width, height,
        columns: 10, rows: 10
    };
    let main_map_render_assets = MapRenderAssets {
        tile_width: width,
        tile_height: height,
        tileset_terrain: Tileset::new(
            "assets/terrain32.png", &pattern).await.ok(),
        tileset_features: Tileset::new(
            "assets/features32.png", &pattern).await.ok(),
        tileset_items: Tileset::new(
            "assets/items32.png", &pattern).await.ok(),
        tileset_actors: Tileset::new(
            "assets/actors32.png", &pattern).await.ok(),
    };    

    // For now, the mini map uses the identical tileset as the main
    // map. We could use the tiny tileset2.png, but as long as
    // rendering speed is ok, it is much easier to maintain only one
    // tileset.
    let mini_map_render_assets = MapRenderAssets {
        tile_width: 4.0,
        tile_height: 4.0,
        tileset_terrain: Tileset::new(
            "assets/terrain32.png", &pattern
        ).await.ok(),
        tileset_features: None,
        tileset_items: None,
        tileset_actors: None
    };

    let (vh, vw) = (
        (screen_height()/(height as f32)) as i32,
        (screen_width()/(width as f32)) as i32
    );
    let mut viewport = Rectangle::from((0, 0, vw, vh));
    let border_size = Point::from((2, 2));
    
    // the World contains the actual game data
    // all of the above will be moved into the World, one by one
    let mut world = World::new();
    world.populate_world();

    // main loop
    let mut last_update = get_time();
    const DELTA: f64 = 0.01;
    let (title_x, title_y) = (10.0, 42.0);

    let mut actions: Vec<Action> = vec!();
    
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
                if viewport.y1 > 0 {
                    viewport.y1 -= 1;
                    viewport.y2 -= 1;
                }
            }

            if is_key_down(KeyCode::Left) {
                if viewport.x1 > 0 {
                    viewport.x1 -= 1;
                    viewport.x2 -= 1;
                }
            }

            if is_key_down(KeyCode::Right) {
                viewport.x1 += 1;
                viewport.x2 += 1;
            }

            if is_key_down(KeyCode::Down) {
                viewport.y1 += 1;
                viewport.y2 += 1;
            }

            // C => Center Viewport
            if is_key_pressed(KeyCode::C) {
                adjust_viewport(
                    &mut viewport,
                    &border_size,
                    &world.player_pos(),
                    ViewportMode::Center
                );
            }
            
            // ASDW => move player
            let player_id = world.player_id();
  
            if is_key_pressed(KeyCode::A) {
                if let Some(move_action) =
                    world::move_by(&world, &player_id, -1, 0, true) {
                        actions.push(move_action);
                        // TODO: action::end_turn()
                    }
            }

            if is_key_pressed(KeyCode::W) {
                if let Some(move_action) =
                    world::move_by(&world, &player_id, 0, -1, true) {
                        actions.push(move_action);
                        // TODO: action::end_turn()
                    }
            }

            if is_key_pressed(KeyCode::D) {
                if let Some(move_action) =
                    world::move_by(&world, &player_id, 1, 0, true) {
                        actions.push(move_action);
                        // TODO: action::end_turn()
                    }
            }

            if is_key_pressed(KeyCode::S) {
                if let Some(move_action) =
                    world::move_by(&world, &player_id, 0, 1, true) {
                        actions.push(move_action);
                        // TODO: action::end_turn()
                    }
            }

            let actors = &mut world.actors;
            
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
                        println!("picking up {}", item.description());
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
        render_map(
            &mut main_map_target,
            &world,
            viewport.x1, viewport.y1,
            viewport.width(), viewport.height(),
            &main_map_render_assets
        );

        render_map(
            &mut mini_map_target,
            &world,
            viewport.x1, viewport.y1,
            64,40,
            &mini_map_render_assets
        );

        // select material (this is just a toy function for testing)
        match is_bw {
            false => gl_use_material(material_vignette),
            true => gl_use_material(material_bw)
        };

        //let base = vec2(10.0, 70.0);
        let base = vec2(0.0, 0.0);
        let mut map_size = vec2(0.0, 0.0);
        let texture = main_map_target.texture;
        map_size = vec2(texture.width(), texture.height());

        draw_texture_ex(
            texture,
            base.x,
            base.y,
            WHITE,
            DrawTextureParams {
                flip_y: true, // this is a temporary workaround
                dest_size: Some(map_size),
                ..Default::default()
            }
        );

        // draw mini map
        let texture = mini_map_target.texture;
        let topleft =
            base +
            vec2(main_map_render_assets.tile_width * viewport.width() as f32, 0.0) +
            vec2(20.0, 0.0);
        let mini_map_size = vec2(texture.width(), texture.height());
        draw_texture_ex(
            texture, topleft.x, topleft.y, WHITE,
            DrawTextureParams {
                flip_y: true,
                dest_size: Some(mini_map_size),
                ..Default::default()
            }
        );

        gl_use_default_material();

        // draw text with shadow
        if false {
            let mut params2 = params.clone();
            params2.color = LIGHTGRAY;

            draw_text_ex(
                "Reveal - Mystic Land of Magic and Adventure", title_x+1.0, title_y+1.0, params2
            );

            draw_text_ex(
                "Reveal - Mystic Land of Magic and Adventure", title_x, title_y, params
            );
        }

        // display status information
        if let Some(player) = world.actors.get(&world.player_id()) {
            // actor position
            let text = format!("{}, {}", player.pos.x, player.pos.y);
            let pos = vec2(20.0, screen_height() - 24.0 - 20.0);
            //let pos = base + Vec2::from((0.0, map_size.y + 24.0 + 10.0));
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

        // process game actions
        while actions.len() > 0 {
            match actions.pop().unwrap() {
                Action::Move {actor_id, pos} => {
                    if let Some(player) = world.actors.get_mut(&actor_id) {
                        player.pos = pos;
                    }
                    // TODO: update map
                },
                Action::MoveFollow {actor_id, pos, mode} => {
                    if let Some(player) = world.actors.get_mut(&actor_id) {
                        player.pos = pos;
                        adjust_viewport(
                            &mut viewport,
                            &border_size,
                            &player.pos,
                            mode
                        )
                    }
                }
            }
        }
        
        next_frame().await
    }
}


