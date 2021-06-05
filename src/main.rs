use macroquad::prelude::*;

mod actor;
mod effect;
mod terrain;
mod point;
mod item;
mod world;
mod flake;
mod id;
mod idmap;
mod action;
mod render;

use effect::{TextEffect, ScaleText};
use terrain::{TerrainKind, Terrain, TerrainFeature, TerrainMap};
use actor::{Actor, ActorId}; //, ActorKind, ActorId, ActorMap};
use point::{Point, Rectangle, PointSet};
use item::{ItemKind}; //ItemId, Item, ItemKind, ItemMap};
use world::{World, ViewportMode, adjust_viewport};
use item::Item;
use action::{Action, GuiAction};
use render::{Map, Tileset, Pattern, TerrainRenderer, ItemRenderer, ActorRenderer};

use std::collections::{VecDeque};

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


// TBR
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

// TBR
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
    let sep = vec2(1.0, 1.0);  // TBR
        
    // render target for map drawing   // TBR
    let map_size = vec2(
        (tiles_x as f32 * (assets.tile_width + sep.x)) as f32,
        (tiles_y as f32 * (assets.tile_height + sep.y)) as f32
    );

    // TBR
    if (target.texture.width() != map_size.x) ||
        (target.texture.height() != map_size.y) {
            *target = render_target(map_size.x as u32, map_size.y as u32);
        }
    target.texture.set_filter(FilterMode::Nearest);

    // TBR
    // set camera, so that drawing operations act
    // on the texture
    let mut camera = Camera2D::from_display_rect(
        Rect::new(0.0, 0.0, map_size.x, map_size.y));
    camera.render_target = Some(*target);
    set_camera(&camera);

    // TBR
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

    // TBR
    // draw texture on screen
    set_default_camera();
}


struct MainState {
    quit: bool,
    last_input: f64,
    end_of_turn: f64,
    is_bw: bool,
    show_inventory: bool,
    show_help: bool,
    show_status: bool,
    draw_fov: bool,
    viewport: Rectangle,
    border_size: Point,
    messages: VecDeque<String>,
    egui_has_focus: bool,
    material_vignette: Material,
    material_bw: Material,
    main_map_target: RenderTarget,
    mini_map_target: RenderTarget,
    params: TextParams,
    params_info: TextParams,
    main_map_render_assets: MapRenderAssets,
    mini_map_render_assets: MapRenderAssets,
    main_map: Map,
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

    // F => hide/show field of view
    if is_key_pressed(KeyCode::F) {
        actions.push(Action::GUI(GuiAction::HideShowFOV));
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


/// process egui events
fn render_and_update_egui(ld: &mut MainState, world: &World) {

    ld.egui_has_focus = false;
    
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
                        ui.label(format!("position: {}, {}",
                                         player.pos.x,
                                         player.pos.y
                        ));
                        ui.label(format!("viewport: {}, {}, {}, {}",
                                         ld.viewport.x1,
                                         ld.viewport.y1,
                                         ld.viewport.x2,
                                         ld.viewport.y2
                        ));
                        ui.label(format!("game time: {}",
                                         world.time
                        ));
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
                    ui.label("f - show/hide field of view");
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
}




#[derive(Debug)]
pub enum MainStateError {
    Foo
}

impl MainState {
    async fn new() -> Result<MainState, MainStateError> {
        // load assets
        let font = load_ttf_font("assets/DejaVuSerif.ttf").await;
        let params = TextParams {
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

        // the map render target will be initialised in the main loop
        let main_map_target = render_target(0, 0); // TBR
        let mini_map_target = render_target(0, 0); // TBR
        
        let (width, height) = (32.0, 32.0); // TBR
        let pattern = Pattern::Matrix { // KEEP
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
            tileset_terrain: Tileset::new("assets/terrain32.png", &pattern).await.ok(),
            tileset_features: None,
            tileset_items: None,
            tileset_actors: None
        };

        let vw = (screen_width()/(width as f32)) as i32;
        let vh = (screen_height()/(height as f32)) as i32;
        let viewport = Rectangle::from((0, 0, vw, vh));

        // EXPERIMENTAL
        let mut main_map = Map::new(32.0, 32.0, Point::new(vw, vh));
        main_map.add_layer(Box::new(TerrainRenderer {
            terrains: Tileset::new("assets/terrain32.png", &pattern).await.unwrap(),
            features: Tileset::new("assets/features32.png", &pattern).await.unwrap(),
        }));
        main_map.add_layer(Box::new(ItemRenderer {
            tileset: Tileset::new("assets/items32.png", &pattern).await.unwrap()
        }));
        main_map.add_layer(Box::new(ActorRenderer {
            tileset: Tileset::new("assets/actors32.png", &pattern).await.unwrap()
        }));

        //let mut mini_map = Map::new(2.0, 2.0);
        
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

        let state = MainState {
            quit: false,
            last_input: get_time(),
            end_of_turn: 0.0,
            is_bw: false,
            show_inventory: true,
            show_help: true,
            show_status: true,
            draw_fov: false,
            viewport,
            border_size: Point::from((10, 10)),
            messages: VecDeque::new(),
            egui_has_focus: false,
            material_vignette,
            material_bw,
            main_map_target,
            mini_map_target,
            params,
            params_info,
            main_map_render_assets,
            mini_map_render_assets,
            main_map
        };

        Ok(state)
    }

    /// process game actions
    fn update(&mut self, world: &mut World, actions: &mut Vec<Action>) {
        while actions.len() > 0 {
            match actions.pop().unwrap() {
                Action::Quit => {
                    self.quit = true;
                },
                Action::EndTurn => {
                    self.end_of_turn = get_time();
                    world.time += 1;
                    // TODO: move NPC
                },
                Action::Ouch => {
                    self.messages.push_front("Ouch!".into());
                    self.end_of_turn = get_time();
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
                            &mut self.viewport,
                            &self.border_size,
                            &player.pos,
                            mode
                        )
                    }
                },
                Action::PickUp { actor_id, items } => {
                    for item_id in items {
                        // remove object position and set owner to 0
                        if let Some(item) = world.items.get_mut(&item_id) {
                            self.messages.push_front(
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
                        self.viewport.y1 += dy;
                        self.viewport.y2 += dy;
                        //}
                    };

                    if dx != 0 {
                        //if viewport.x1 + dx > 0 {
                        self.viewport.x1 += dx;
                        self.viewport.x2 += dx;
                        //}
                    }
                },
                Action::CenterViewport => {
                    adjust_viewport(
                        &mut self.viewport,
                        &self.border_size,
                        &world.player_pos(),
                        ViewportMode::Center
                    );
                },
                Action::GUI(GuiAction::TestBW) => {
                    self.is_bw = !self.is_bw;
                },
                Action::GUI(GuiAction::HideShowInventory) => {
                    self.show_inventory = !self.show_inventory;
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
                    self.show_help = !self.show_help;
                },
                Action::GUI(GuiAction::HideShowStatus) => {
                    self.show_status = !self.show_status;
                },
                Action::GUI(GuiAction::HideShowFOV) => {
                    self.draw_fov = !self.draw_fov;
                }
            }
        }
    }
    
    fn render(&mut self, world: &World) {
        clear_background(BLACK);

        // EXPERIMENTAL
        self.main_map.render_to_target(&world, &self.viewport);
        
        // --- map drawing --
        render_map(
            &mut self.main_map_target,
            &world,
            self.viewport.x1, self.viewport.y1,
            self.viewport.width(), self.viewport.height(),
            &self.main_map_render_assets
        );

        // select material (this is just a toy function for testing)
        match self.is_bw {
            false => gl_use_material(self.material_vignette),
            true => gl_use_material(self.material_bw)
        };

        //let base = vec2(10.0, 70.0);
        let base = vec2(0.0, 0.0);
        let mut map_size = vec2(0.0, 0.0);
        let texture = self.main_map_target.texture;
        map_size = vec2(texture.width(), texture.height());

        // EXPERIMENTAL
        //        draw_texture_ex(texture, base.x, base.y, WHITE,
        // TODO: do we copy the texture here?
        draw_texture_ex(*self.main_map.texture(), base.x, base.y, WHITE,
                        DrawTextureParams {
                            flip_y: true, // this is a temporary workaround
                            //dest_size: Some(map_size),
                            dest_size: Some(self.main_map.target_size()),
                            ..Default::default()
                        }
        );

        // EXPERIMENTAL: highlight certain tiles by surrounding
        // them with a red rectangle
        if self.draw_fov {
            let mut points = PointSet::new();
            let p = world.player_pos();
            points.insert(p.clone());
            points.insert(p.clone() + (0, 1).into());
            points.insert(p.clone() + (0, 2).into());
            points.insert(p.clone() + (0, -1).into());
            points.insert(p.clone() + (0, -2).into());
            points.insert(p.clone() + (1, 0).into());
            points.insert(p.clone() + (2, 0).into());
            points.insert(p.clone() + (-1, 0).into());
            points.insert(p.clone() + (-2, 0).into());
            points.insert(p.clone() + (-1, -1).into());
            points.insert(p.clone() + (1, -1).into());
            points.insert(p.clone() + (-1, 1).into());
            points.insert(p.clone() + (1, 1).into());
            for p in points.iter() {
                let x = (p.x - self.viewport.x1) as f32 * 32.0 + base.x;
                let y = (p.y - self.viewport.y1) as f32 * 32.0 + base.y;
                draw_rectangle_lines(x, y, 32.0, 32.0, 4.0, RED);
            }
        };
        
        // draw mini map
        render_map(
            &mut self.mini_map_target,
            &world,
            self.viewport.x1, self.viewport.y1,
            48,20,
            &self.mini_map_render_assets
        );

        let texture = self.mini_map_target.texture;
        let topleft = base + vec2(
            screen_width() - self.mini_map_target.texture.width() - 10.0,
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
                draw_text_ex(&text, pos.x, pos.y, self.params_info);
            
                let text = "use <p> to pick up the items";
                let pos = pos + Vec2::from((0.0, 30.0));
                draw_text_ex(&text, pos.x, pos.y, self.params_info);
            }
        }

        // display messages
        let mut msg_params = self.params_info.clone();
        let max_messages = 5;
        let mut pos = vec2(20.0, 20.0);
        let yoffset = 1.1 * msg_params.font_size as f32;
    
        for message in self.messages.iter().take(max_messages) {
            draw_text_ex(&message, pos.x, pos.y, msg_params);
            msg_params.color.a *= 0.7; // blend out color
            pos.y += yoffset;
        };
    }
}



#[macroquad::main(window_conf)]
async fn main() {
    // TODO: parse command line arguments, e.g. --fullscreen

    let mut state = MainState::new().await.unwrap();

    // the World contains the actual game data
    let mut world = World::new();
    world.populate_world();

    // main loop
    let mut actions: Vec<Action> = vec!();

    adjust_viewport(
        &mut state.viewport,
        &state.border_size,
        &world.player_pos(),
        ViewportMode::Center
    );

    state.messages.push_front("Welcome to the Land of Mystery...".into());

    while !state.quit {

        render_and_update_egui(&mut state, &world);

        // update, if necessary
        if !state.egui_has_focus
            && (get_time() - state.last_input > DELTA_UPDATE)
            && (get_time() - state.end_of_turn > DELTA_TURN)
        {
            state.last_input = get_time();
            actions.extend(read_input(&world));
        }

        state.update(&mut world, &mut actions);
        state.render(&world);
        state.messages.truncate(10);  // flush very old messages

        egui_macroquad::draw();
        
        next_frame().await
    }
}


