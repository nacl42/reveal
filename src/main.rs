use macroquad::prelude::*;
use macroquad::{input, window};

mod effect;
use effect::{TextEffect, ScaleText};


fn window_conf() -> Conf {
    Conf {
        window_title: "Reveal".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("You are in a cave and there is no light.");

    println!("Press <q> to quit, <up> to move text up, <s> to scale text!");
    
    // load assets
    let font = load_ttf_font("assets/DejaVuSerif.ttf").await;
    let mut params = TextParams {
        font,
        font_size: 24,
        color: RED,
        ..Default::default()
    };

    let mut effects: Vec<Box<dyn TextEffect>> = vec!();
    
    // main loop
    let mut last_update = get_time();
    const DELTA: f64 = 0.05;
    let (x, mut y) = (10.0, 42.0);
    
    loop {
        // update, if necessary
        if get_time() - last_update > DELTA {
            last_update = get_time();
            y += 1.0;

            if is_key_down(KeyCode::Q) {
                println!("GOODBYE");
                break;
            }

            if is_key_down(KeyCode::Up) {
                if y > 5.0 {
                    y -= 5.0;   
                }
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
        draw_text_ex("Reveal - Mystic Land of Magic and Adventure", x, y, params);
        
        next_frame().await
    }
}
