use macroquad::prelude::*;

mod action;
mod actor;
mod demo_game;
mod flake;
mod idmap;
mod item;
mod point;
mod render;
mod terrain;
mod world;
extern crate rand;

use point::{Point, Rectangle};
use world::{World, ViewportMode, adjust_viewport, HighlightMode};
use action::{Action, GuiAction};
use render::{
    Map, Tileset, Pattern,
    TerrainLayer, ItemLayer, ActorLayer, HighlightLayer,
    InventoryWidget
};

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
        //fullscreen: true,
        ..Default::default()
    }
}


fn read_input_use_item(state: &MainState, world: &World) -> Vec<Action> {
    let mut actions = Vec::<Action>::new();

    if is_key_pressed(KeyCode::Escape) {
        println!("switching back to default mode");
        actions.push(
            Action::GUI(GuiAction::SwitchMode(MainInputMode::Default))
        );
    }

    // TODO: render cancel button
    
    // check if we are hovering over an item
    if false {
        if let Some(player) = &world.actors.get(&world.player_id()) {
            if let Some(item_id) =
                state.inventory_widget.screen_to_item_id(
                    &Vec2::from(mouse_position()), &player.inventory
                ) {
                    println!("Hovering over an inventory item");
                }
        }
    }

    // check if we have selected an item
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(player) = &world.actors.get(&world.player_id()) {
            if let Some(item_id) =
                state.inventory_widget.screen_to_item_id(
                    &Vec2::from(mouse_position()), &player.inventory
                ) {
                    if let Some(item) = world.items.get(item_id) {
                        println!("Using item {}", item.description());
                        actions.push(Action::UseItem {
                            target: world.player_id(),
                            item_id: *item_id
                        });
                    }
                }
        }
    } else if is_mouse_button_pressed(MouseButton::Right) {
        if let Some(player) = &world.actors.get(&world.player_id()) {
            if let Some(item_id) =
                state.inventory_widget.screen_to_item_id(
                    &Vec2::from(mouse_position()), &player.inventory
                ) {
                    if let Some(item) = world.items.get(item_id) {
                        println!("Dropping item {}", item.description());
                        actions.push(Action::DropItem {
                            item_id: *item_id
                        });
                    }
                }        
        }
    }
    
    actions
}

fn read_input_default(state: &MainState, world: &World) -> Vec<Action> {

    let mut actions = Vec::<Action>::new();

    // Q => quit
    if is_key_down(KeyCode::Q) {
        println!("GOODBYE");
        actions.push(Action::Quit);
    }

    // U => use item
    if is_key_pressed(KeyCode::U) {
        println!("switching to use item mode");
        actions.push(
            Action::GUI(GuiAction::SwitchMode(MainInputMode::UseItem))
        );
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
                .default_pos([screen_width(), screen_height() / 3.0])
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



struct MainState {
    quit: bool,
    last_input: f64,
    end_of_turn: f64,
    is_bw: bool,
    show_inventory: bool,
    show_help: bool,
    show_status: bool,
    viewport: Rectangle,
    border_size: Point,
    messages: VecDeque<String>,
    egui_has_focus: bool,
    material_vignette: Material,
    material_bw: Material,
    params_info: TextParams,
    main_map: Map,
    mini_map: Map,
    inventory_widget: InventoryWidget,
    item_tileset: Tileset,
    input_mode: MainInputMode
}

#[derive(Debug)]
pub enum MainStateError {
    Foo
}

#[derive(Debug)]
pub enum MainInputMode {
    Default,
    UseItem,
}

impl MainState {
    async fn new() -> Result<MainState, MainStateError> {
        // load assets
        let font = load_ttf_font("assets/DejaVuSerif.ttf").await;

        let params_info = TextParams {
            font,
            font_size: 16,
            color: WHITE,
            ..Default::default()
        };

        let (width, height) = (32.0, 32.0);
        let pattern = Pattern::Matrix {
            width, height,
            columns: 10, rows: 10
        };

        // TODO: screen_width() and screen_height() return only the
        // window's width and height, even if fullscreen mode is
        // enabled. An issue has been filed.
        let vw = (screen_width()/(width as f32)) as i32;
        let vh = (screen_height()/(height as f32)) as i32;
        let viewport = Rectangle::from((0, 0, vw, vh-3));

        let mut main_map = Map::new(width, height, Point::new(vw, vh));
        main_map.add_layer(Box::new(TerrainLayer {
            terrains: Tileset::new("assets/terrain32.png", &pattern).await.unwrap(),
            features: Tileset::new("assets/features32.png", &pattern).await.unwrap(),
        }));
        main_map.add_layer(Box::new(ItemLayer {
            tileset: Tileset::new("assets/items32.png", &pattern).await.unwrap()
        }));
        main_map.add_layer(Box::new(ActorLayer {
            tileset: Tileset::new("assets/actors32.png", &pattern).await.unwrap()
        }));

        main_map.add_layer(Box::new(HighlightLayer()));

        // TODO: share Layer, so that we do not need to allocate a texture
        // more than once.
        let mut mini_map = Map::new(4.0, 4.0, Point::new(vw, vh));
        mini_map.add_layer(Box::new(TerrainLayer {
            terrains: Tileset::new("assets/terrain32.png", &pattern).await.unwrap(),
            features: Tileset::new("assets/features32.png", &pattern).await.unwrap(),
        }));
        
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

        // inventory
        let inventory_widget = InventoryWidget::new(
            vec2(screen_width()/2.0, screen_height() - 130.0)
        );

        // TODO: share tilesets among different rendering widgets
        let item_tileset = Tileset::new(
            "assets/items32.png", &pattern
        ).await.unwrap();

        let state = MainState {
            quit: false,
            last_input: get_time(),
            end_of_turn: 0.0,
            is_bw: false,
            show_inventory: false,
            show_help: false,
            show_status: true,
            inventory_widget,
            item_tileset,
            viewport,
            border_size: Point::from((10, 10)),
            messages: VecDeque::new(),
            egui_has_focus: false,
            material_vignette,
            material_bw,
            params_info,
            main_map,
            mini_map,
            input_mode: MainInputMode::Default
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
                Action::UseItem { item_id, target } => {
                    world.use_item(&item_id, &target);
                    self.input_mode = MainInputMode::Default;
                },
                Action::DropItem { item_id } => {
                    world.drop_item(&item_id);
                    self.input_mode = MainInputMode::Default;
                }
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
                    if world.highlight_mode.is_none() {
                        world.highlight_mode = Some(HighlightMode::FOV);
                    } else {
                        world.highlight_mode = None;
                    }
                },
                Action::GUI(GuiAction::SwitchMode(mode)) => {
                    self.input_mode = mode;
                }
            }
        }
    }

    fn update_fov(&self, world: &mut World) {
        // EXPERIMENTAL: highlight certain tiles by surrounding
        // them with a red rectangle
        let from = world.player_pos();
        let points = &mut world.highlights;
        points.clear();

        // TODO: use map offset, not arbitrary number
        let pos = Vec2::from(mouse_position()) - vec2(0.0, 32.0); // - map offset
        if let Some(map_pos) = self.main_map.screen_to_tile(&pos) {
            let map_pos = map_pos + self.viewport.top_left();

            for point in from.line_to(&map_pos) {
                points.insert(point);
            }
        }
    }
   
    fn render(&mut self, world: &World) {
        clear_background(BLACK);

        // --- main map drawing --
        self.main_map.render_to_target(&world, &self.viewport.top_left());
        
        // select material for map depending on input mode
        match self.input_mode {
            MainInputMode::Default
                => gl_use_material(self.material_vignette),
            MainInputMode::UseItem
                => gl_use_material(self.material_bw)
        };

        //let base = vec2(10.0, 70.0);
        let base = vec2(0.0, 32.0);
        let texture = self.main_map.texture();

        // EXPERIMENTAL
        // TODO: do we copy the texture here?
        draw_texture_ex(
            *texture, base.x, base.y, WHITE,
            DrawTextureParams {
                flip_y: true, // this is a temporary workaround
                dest_size: Some(self.main_map.target_size()),
                ..Default::default()
            }
        );
        
        // draw mini map
        self.mini_map.render_to_target(&world, &self.viewport.top_left());

        let texture = self.mini_map.texture();
        let map_pos = base + vec2(
            screen_width() - texture.width() - 10.0,
            10.0
        );
        // TODO: do we copy the texture here?
        draw_texture_ex(
            *texture, map_pos.x, map_pos.y, WHITE,
            DrawTextureParams {
                flip_y: true, // this is a temporary workaround
                dest_size: Some(self.mini_map.target_size()),
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

        // render inventory
        if let Some(player) = world.actors.get(&world.player_id()) {
            self.inventory_widget.render(
                &world,
                &player.inventory, &self.item_tileset
            );
        }
    }
}



#[macroquad::main(window_conf)]
async fn main() {
    // TODO: parse command line arguments, e.g. --fullscreen

    let mut state = MainState::new().await.unwrap();

    // the World contains the actual game data
    let mut world = World::new();
    demo_game::populate_world(&mut world);

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

        // Update, if necessary.
        // Only update every DELTA_UPDATE intervals.
        // Do not update, if time since last end of turn is less than DELTA_TURN.
        if !state.egui_has_focus
            && (get_time() - state.last_input > DELTA_UPDATE)
            && (get_time() - state.end_of_turn > DELTA_TURN)
        {
            state.last_input = get_time();

            match state.input_mode {
                MainInputMode::Default
                    => actions.extend(read_input_default(&state, &world)),
                MainInputMode::UseItem
                    => actions.extend(read_input_use_item(&state, &world)),
            }
        }

        state.update(&mut world, &mut actions);
        state.update_fov(&mut world);
        state.render(&world);
        state.messages.truncate(10);  // flush very old messages

        egui_macroquad::draw();
        
        next_frame().await
    }
}


