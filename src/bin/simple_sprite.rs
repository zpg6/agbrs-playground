#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agbrs_playground::color;

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();

    let mut bg = agb::display::tiled::RegularBackground::new(
        agb::display::Priority::P0,
        agb::display::tiled::RegularBackgroundSize::Background32x32,
        agb::display::tiled::TileFormat::FourBpp,
    );

    // Smiley face palette: 0=black, 1=yellow, 2=black, 3=red, 4=white
    let palette_data = agb::display::Palette16::new([
        color!(black),
        color!(yellow),
        color!(black),
        color!(red),
        color!(white),
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

    // Smiley face pattern (8x8 pixels)
    let smiley_pattern = [
        [0, 0, 1, 1, 1, 1, 0, 0], // Top of head
        [0, 1, 1, 1, 1, 1, 1, 0], // Upper head
        [1, 1, 1, 1, 1, 1, 1, 1], // Middle head
        [1, 1, 4, 0, 0, 4, 1, 1], // Eyes
        [1, 1, 1, 1, 1, 1, 1, 1], // Middle head
        [1, 1, 3, 0, 0, 3, 1, 1], // Mouth
        [0, 1, 1, 1, 1, 1, 1, 0], // Lower head
        [0, 0, 1, 1, 1, 1, 0, 0], // Bottom of head
    ];

    let mut tile = agb::display::tiled::DynamicTile16::new();
    for y in 0..8 {
        for x in 0..8 {
            tile.set_pixel(x, y, smiley_pattern[y][x]);
        }
    }

    // Place smiley faces in a cross pattern
    let center_x = 12;
    let center_y = 8;

    for dy in -2..3 {
        for dx in -2..3 {
            let x = center_x + dx;
            let y = center_y + dy;

            if (dx == 0) || (dy == 0) {
                bg.set_tile_dynamic16((x, y), &tile, agb::display::tiled::TileEffect::default());
            }
        }
    }

    loop {
        let mut frame = gfx.frame();
        bg.show(&mut frame);
        frame.commit();
    }
}
