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

    let mut bg = agb::display::tiled::RegularBackground::new(
        agb::display::Priority::P0,
        agb::display::tiled::RegularBackgroundSize::Background32x32,
        agb::display::tiled::TileFormat::FourBpp,
    );

    // Star palette: 0=transparent/background, 1=dark blue, 2=bright yellow, 3=white
    let palette_data = agb::display::Palette16::new([
        rgb15!(0x001122), // Dark blue background
        rgb15!(0x000033), // Darker blue
        rgb15!(0xFFDD00), // Bright yellow for star
        rgb15!(0xFFFF88), // Light yellow highlight
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
        color!(black),
    ]);

    agb::display::tiled::VRAM_MANAGER.set_background_palettes(&[palette_data]);

    // Classic 5-pointed star pattern (8x8 pixels)
    let star_pattern = [
        [0, 0, 0, 0, 0, 0, 0, 0], // Empty top
        [0, 0, 0, 3, 0, 0, 0, 0], // Top point
        [0, 0, 0, 3, 2, 0, 0, 0], // Upper body
        [2, 2, 2, 3, 2, 2, 2, 2], // Wide horizontal (left & right points)
        [0, 2, 2, 2, 2, 2, 2, 0], // Inner body
        [0, 0, 2, 2, 2, 2, 0, 0], // Lower body
        [0, 0, 2, 0, 0, 2, 0, 0], // Bottom points start
        [0, 2, 0, 0, 0, 0, 2, 0], // Bottom points
    ];

    let mut tile = agb::display::tiled::DynamicTile16::new();
    for y in 0..8 {
        for x in 0..8 {
            tile.set_pixel(x, y, star_pattern[y][x]);
        }
    }

    // Place single star in center of screen
    let center_x = 15;
    let center_y = 10;

    bg.set_tile_dynamic16((center_x, center_y), &tile, agb::display::tiled::TileEffect::default());

    loop {
        let mut frame = gfx.frame();
        bg.show(&mut frame);
        frame.commit();
    }
}
