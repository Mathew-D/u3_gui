/*
By: <Your Name Here>
Date: 2026-02-17
Program Details: <Program Description Here>
*/

mod modules;

use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
use crate::modules::still_image::StillImage;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "u3_gui".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut lbl_msg =
        Label::new("Hello\nWorld", 50.0, 50.0, 30);
    lbl_msg
        .with_fixed_size(400.0, 500.0)
        .with_border(RED, 5.0);

    let btn_bhs = TextButton::new(
        50.0, 500.0, 200.0, 50.0, "BHS", BLUE, GREEN, 30,
    );
 let img_out = StillImage::new(
        "assets/bird2.png",
        100.0,  // width
        50.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;

    loop {
        clear_background(WHITE);
        draw_grid(50.0, BROWN);

        if btn_bhs.click() {
            lbl_msg.set_text("Bowmanville asdasdasdasdasdasdasdasdasdasdasdasdasdasdasdHS");
        }
        img_out.draw();
        lbl_msg.draw();
        next_frame().await;
    }
}
