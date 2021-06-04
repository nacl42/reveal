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
use action::{Action, GuiAction};

use std::collections::VecDeque;

const CRT_FRAGMENT_SHADER: &'static str = include_str!("shaders/vignette_fragment.glsl");
const CRT_VERTEX_SHADER: &'static str = include_str!("shaders/vignette_vertex.glsl");
const BW_FRAGMENT_SHADER: &'static str = include_str!("shaders/bw_fragment.glsl");

const DELTA_UPDATE: f64 = 0.01;
const DELTA_TURN: f64 = 0.1;

fn window_conf() -> Conf {
    Conf {
        window_title: "Reveal".to_owned(),
        window_width: 1024,
        window_height: 800,
        fullscreen: true,
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


fn read_input(world: &World) -> Vec<Action> {

    let mut actions = Vec::<Action>::new();

    // Q => quit
    if is_key_down(KeyCode::Q) {
        println!("GOODBYE");
        actions.push(Action::Quit);
    }
    
    // B => switch black/white and color mode
    if is_key_pressed(KeyCode::B) {
        println!("switching color vision");
        actions.push(Action::GUI(GuiAction::TestBW));
    }

    // shift + arrows keys => scroll map
    if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
        if is_key_down(KeyCode::Up) {
            actions.push(Action::MoveViewport{ dx: 0, dy: -1 });
        }

        if is_key_down(KeyCode::Left) {
            actions.push(Action::MoveViewport{ dx: -1, dy: 0 });
        }

        if is_key_down(KeyCode::Right) {
            actions.push(Action::MoveViewport{ dx: 1, dy: 0 });
        }

        if is_key_down(KeyCode::Down) {
            actions.push(Action::MoveViewport{ dx: 0, dy: 1 });
        }
    } else {
        // cursor keys (w/o shift) => move player
        let player_id = world.player_id();
  
        if is_key_down(KeyCode::Left) {
            if let Some(move_action) =
                world::move_by(&world, &player_id, -1, 0, true) {
                    actions.push(move_action);
                    actions.push(Action::EndTurn);
                } else {
                    actions.push(Action::Ouch);
                }
        }
    
        if is_key_down(KeyCode::Up) {
            if let Some(move_action) =
                world::move_by(&world, &player_id, 0, -1, true) {
                    actions.push(move_action);
                    actions.push(Action::EndTurn);
                } else {
                    actions.push(Action::Ouch);
                }
        }

        if is_key_down(KeyCode::Right) {
            if let Some(move_action) =
                world::move_by(&world, &player_id, 1, 0, true) {
                    actions.push(move_action);
                    actions.push(Action::EndTurn);
                } else {
                    actions.push(Action::Ouch);
                }
        }
    
        if is_key_down(KeyCode::Down) {
            if let Some(move_action) =
                world::move_by(&world, &player_id, 0, 1, true) {
                    actions.push(move_action);
                    actions.push(Action::EndTurn);
                } else {
                    actions.push(Action::Ouch);
                }
        }
    }
    
    // C => Center Viewport
    if is_key_pressed(KeyCode::C) {
        actions.push(Action::CenterViewport);
    }            

    // I => hide/show inventory
    if is_key_pressed(KeyCode::I) {
        actions.push(Action::GUI(GuiAction::HideShowInventory));
    }

    // H => hide/show help
    if is_key_pressed(KeyCode::H) {
        actions.push(Action::GUI(GuiAction::HideShowHelp));
    }
    
        // T => show off text effect
        // AGAIN:
    // if is_key_pressed(KeyCode::T) {
    //     if effects.len() == 0 {
    //         effects.push(Box::new(ScaleText::new()));
    //     }
    // }

    // S => switch player status window
    if is_key_pressed(KeyCode::S) {
        actions.push(Action::GUI(GuiAction::HideShowStatus));
    }

    // P => pick up items
    if is_key_pressed(KeyCode::P) {
        let player_id = world.player_id();
        if let Some(player) = world.actors.get(&player_id) {
            let items = world.item_ids_at(&player.pos);
            actions.push(Action::PickUp {
                actor_id: player_id,
                items
            });
        };
    }

    actions
}


#[macroquad::main(window_conf)]
async fn main() {
    println!("You are in a cave and there is no light.");

    println!("Press <q> to quit and <t> to scale text!");
    println!("Try <b> to switch color vision.");
    println!("Move player with cursor keys.");
    println!("Scroll map with shift + cursor keys.");
    println!("Center map on player using <C>!");
    println!("List inventory with <I>, pick up items with <P>.");
    println!("Show/hide help window with <H>, show/hide status window with <S>");

    // TODO: parse command line arguments, e.g. --fullscreen
    
    // load assets
    let font = load_ttf_font("assets/DejaVuSerif.ttf").await;
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


    // the map render target will be initialised in the main loop
    let mut main_map_target = render_target(0, 0);
    let mut mini_map_target = render_target(0, 0);
    

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


    //effects: Vec<Box<dyn TextEffect>>,

    // the World contains the actual game data
    // all of the above will be moved into the World, one by one
    let mut world = World::new();
    world.populate_world();

    // main loop
    let (title_x, title_y) = (10.0, 42.0);

    let mut player_name: String = String::from("Sir Lancelot");

    let mut actions: Vec<Action> = vec!();
    
    let (vh, vw) = (
        (screen_height()/(height as f32)) as i32,
        (screen_width()/(width as f32)) as i32
    );

    struct LoopData {
        quit: bool,
        last_input: f64,
        end_of_turn: f64,
        is_bw: bool,
        show_inventory: bool,
        show_help: bool,
        show_status: bool,
        viewport: Rectangle,
        border_size: Point,
        messages: VecDeque<String>
    };

    let mut ld = LoopData {
        quit: false,
        last_input: get_time(),
        end_of_turn: 0.0,
        is_bw: false,
        show_inventory: true,
        show_help: true,
        show_status: true,
        viewport: Rectangle::from((0, 0, vw, vh)),
        border_size: Point::from((10, 10)),
        messages: VecDeque::new()
    };

    adjust_viewport(
        &mut ld.viewport,
        &ld.border_size,
        &world.player_pos(),
        ViewportMode::Center
    );

    ld.messages.push_front("Welcome to the Land of Mystery...".into());

    // REPL:
    // read - read input
    // eval - perform actions
    // print - draw gui
    
    while !ld.quit {

        // process egui events
        let mut egui_has_focus = false;
        egui_macroquad::ui(|egui_ctx| {

            // status window
            if ld.show_status {
                egui::Window::new("player")
                    .default_pos([0.0, screen_height()])
                    .resizable(false)
                    .collapsible(false)
                    .show(egui_ctx, |ui| {
                        // actor position
                        if let Some(player) = world.actors.get(&world.player_id()) {
                            ui.label(format!("position: {}, {}", player.pos.x, player.pos.y));
                            ui.label(format!("game time: {}", world.time));
                        }
                    });
            };
            
            // help window
            if ld.show_help {
                egui::Window::new("help")
                    .default_pos([screen_width(), 0.0])
                    .resizable(false)
                    .collapsible(false)
                    .show(egui_ctx, |ui| {
                        ui.label("arrow keys - move around");
                        ui.label("i - show/hide inventory");
                        ui.label("p - pick up items");
                        ui.label("c - center viewport");
                        ui.label("shift + arrow keys - scroll map");
                        ui.label("h - show/hide help");
                        ui.label("s - show/hide status");
                        ui.label("q - quit");
                    });
            };
            
            if ld.show_inventory {
                egui::Window::new("You carry the following items:")
                    .default_pos([screen_width(), screen_height()])
                    .resizable(false)
                    .collapsible(false)
                    .show(egui_ctx, |ui| {
                        //ui.label("You carry the following items:");
                        // let response = ui.add(
                        //     egui::TextEdit::singleline(&mut player_name)
                        //         .hint_text("Enter your name here")
                        // );
                        // egui_has_focus |= response.has_focus();

                        //ui.separator();
                        if let Some(player) = &world.actors.get(&world.player_id()) {
                            for (n, item_id) in player.inventory.iter().enumerate() {
                                if let Some(item) = &world.items.get(&item_id) {
                                    ui.label(format!("{n} - {text}", n=n+1, text=item.description()));
                                }
                            }
                        }
                    });
            }
        });

        // update, if necessary
        if !egui_has_focus
            && (get_time() - ld.last_input > DELTA_UPDATE)
            && (get_time() - ld.end_of_turn > DELTA_TURN)
        {
            ld.last_input = get_time();
            actions.extend(read_input(&world));
        }

        // update and apply effects
        // AGAIN:
        // effects.iter_mut().for_each(|e| e.step());
        // effects.retain(|e| e.is_alive());
        // effects.iter().for_each(|e| e.apply(&mut params));

        // process game actions
        while actions.len() > 0 {
            match actions.pop().unwrap() {
                Action::Quit => {
                    ld.quit = true;
                },
                Action::EndTurn => {
                    ld.end_of_turn = get_time();
                    world.time += 1;
                    // TODO: move NPC
                },
                Action::Ouch => {
                    ld.messages.push_front("Ouch!".into());
                    ld.end_of_turn = get_time();
                },
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
                            &mut ld.viewport,
                            &ld.border_size,
                            &player.pos,
                            mode
                        )
                    }
                },
                Action::PickUp { actor_id, items } => {
                    for item_id in items {
                        // remove object position and set owner to 0
                        if let Some(item) = world.items.get_mut(&item_id) {
                            ld.messages.push_front(
                                format!("You pick up {}.", item.description())
                            );
                            item.owner = Some(actor_id);
                            item.pos = None;
                            world.actors.get_mut(&actor_id).unwrap()
                                .inventory.push(item_id.clone());
                        }                    
                    }                    
                },
                Action::MoveViewport { dx, dy } => {
                    if dy != 0 {
                        //if viewport.y1 + dy > 0 {
                            ld.viewport.y1 += dy;
                            ld.viewport.y2 += dy;
                        //}
                    };

                    if dx != 0 {
                        //if viewport.x1 + dx > 0 {
                            ld.viewport.x1 += dx;
                            ld.viewport.x2 += dx;
                        //}
                    }
                },
                Action::CenterViewport => {
                    adjust_viewport(
                        &mut ld.viewport,
                        &ld.border_size,
                        &world.player_pos(),
                        ViewportMode::Center
                    );
                },
                Action::GUI(GuiAction::TestBW) => {
                    ld.is_bw = !ld.is_bw;
                },
                Action::GUI(GuiAction::HideShowInventory) => {
                    ld.show_inventory = !ld.show_inventory;
                    println!("Inventory:");
                    if let Some(player) = world.actors.get(&world.player_id()) {
                        for (n, item_id) in player.inventory.iter().enumerate() {
                            if let Some(item) = world.items.get(item_id) {
                                println!("{} - {}", n, item.description());
                            }
                        }
                    }
                },
                Action::GUI(GuiAction::HideShowHelp) => {
                    ld.show_help = !ld.show_help;
                },
                Action::GUI(GuiAction::HideShowStatus) => {
                    ld.show_status = !ld.show_status;
                }
            }
        }

        // draw
        clear_background(BLACK);

        // --- map drawing --
        render_map(
            &mut main_map_target,
            &world,
            ld.viewport.x1, ld.viewport.y1,
            ld.viewport.width(), ld.viewport.height(),
            &main_map_render_assets
        );

        // select material (this is just a toy function for testing)
        match ld.is_bw {
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
        
        render_map(
            &mut mini_map_target,
            &world,
            ld.viewport.x1, ld.viewport.y1,
            48,20,
            &mini_map_render_assets
        );

        let texture = mini_map_target.texture;
        let topleft = base + vec2(
            screen_width() - mini_map_target.texture.width() - 10.0,
            10.0
            //screen_height() - mini_map_target.texture.height() - 20.0
        );
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
            let pos = vec2(20.0, screen_height() - 24.0 - 20.0);
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

        // display messages
        let max_messages = 5;
        //let mut pos = vec2(20.0, screen_height() - 20.0);
        let mut pos = vec2(20.0, 20.0);
        let mut msg_params = params_info.clone();
        for message in ld.messages.iter().take(max_messages) {
            draw_text_ex(&message, pos.x, pos.y, msg_params);
            // move one line below (with some spacing)
            pos.y += (1.1 * msg_params.font_size as f32);
            // blend out color
            msg_params.color.a *= 0.7; 
        };

        // flush very old messages
        ld.messages.truncate(10);
        

        egui_macroquad::draw();
        
        next_frame().await
    }
}


