#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agbrs_playground::{color, rgb15};

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();

    // Create a background
    let mut bg = agb::display::tiled::RegularBackground::new(
        agb::display::Priority::P0,
        agb::display::tiled::RegularBackgroundSize::Background32x32,
        agb::display::tiled::TileFormat::FourBpp,
    );

    // Color wheel palette (13 colors + black + white)
    let palette_data = agb::display::Palette16::new([
        color!(black),    // 0: Black (outside circle)
        color!(red),      // 1: Red (0°)
        rgb15!(0xFF4000), // 2: Red-Orange (30°)
        color!(orange),   // 3: Orange (60°)
        rgb15!(0xFFFF00), // 4: Yellow (90°)
        rgb15!(0x80FF00), // 5: Yellow-Green (120°)
        color!(green),    // 6: Green (150°)
        rgb15!(0x00FF80), // 7: Green-Cyan (180°)
        color!(cyan),     // 8: Cyan (210°)
        rgb15!(0x0080FF), // 9: Blue-Cyan (240°)
        color!(blue),     // 10: Blue (270°)
        rgb15!(0x8000FF), // 11: Blue-Magenta (300°)
        color!(magenta),  // 12: Magenta (330°)
        rgb15!(0xFF0080), // 13: Magenta-Red (360°)
        color!(white),    // 14: White (center)
        color!(gray),     // 15: Unused
    ]);

    agb::display::tiled::VRAM_MANAGER.set_background_palettes(&[palette_data]);

    // Center background on screen (scroll offset for 256x192 bg on 240x160 screen)
    bg.set_scroll_pos((8, 16));

    // Color wheel centered at screen center
    let center_x = 16; // Screen center + scroll offset
    let center_y = 12;
    let max_radius = 9; // 72 pixel radius

    for tile_y in 0..24 {
        for tile_x in 0..32 {
            let mut tile = agb::display::tiled::DynamicTile16::new();

            for py in 0..8 {
                for px in 0..8 {
                    let pixel_x = tile_x * 8 + px;
                    let pixel_y = tile_y * 8 + py;

                    let dx = pixel_x as i32 - center_x * 8;
                    let dy = pixel_y as i32 - center_y * 8;
                    let distance_squared = dx * dx + dy * dy;
                    let max_radius_squared = (max_radius * 8) * (max_radius * 8);

                    let color_index = if distance_squared <= max_radius_squared {
                        if dx == 0 && dy == 0 {
                            14 // White center
                        } else {
                            // Simple atan2 approximation for color wheel
                            let angle_approx = if dx >= 0 && dy >= 0 {
                                (dy * 4) / (dx + dy + 1)
                            } else if dx < 0 && dy >= 0 {
                                4 + ((-dx) * 4) / (dy + (-dx) + 1)
                            } else if dx < 0 && dy < 0 {
                                8 + ((-dy) * 4) / ((-dx) + (-dy) + 1)
                            } else {
                                12 + (dx * 4) / ((-dy) + dx + 1)
                            };
                            (angle_approx % 13) as u8 + 1
                        }
                    } else {
                        0 // Black outside circle
                    };

                    tile.set_pixel(px, py, color_index);
                }
            }

            bg.set_tile_dynamic16(
                (tile_x as i32, tile_y as i32),
                &tile,
                agb::display::tiled::TileEffect::default(),
            );
        }
    }

    loop {
        let mut frame = gfx.frame();
        bg.show(&mut frame);
        frame.commit();
    }
}
