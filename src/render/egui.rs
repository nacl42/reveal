use macroquad::prelude::*;

use crate::MainState;
use crate::world::World;

pub fn render_and_update_egui(state: &mut MainState, world: &World) {

    state.egui_has_focus = false;
    
    egui_macroquad::ui(|egui_ctx| {

        // status window
        if state.show_status {
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
                                         state.viewport.x1,
                                         state.viewport.y1,
                                         state.viewport.x2,
                                         state.viewport.y2
                        ));
                        ui.label(format!("game time: {}",
                                         world.time
                        ));
                        ui.label(format!("health: {}",
                                         player.health));
                        ui.label(format!("gold: {}",
                                         player.gold));
                        ui.label(format!("skills: {}",
                                         player.skills.iter().map(|s| s.description()).collect::<Vec<String>>().join(",")));
                        
                    }
                });
        };
            
        // help window
        if state.show_help {
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
            
        if state.show_inventory {
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
