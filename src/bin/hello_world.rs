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

    // Simple palette: 0=black, 1=white
    let palette_data = agb::display::Palette16::new([
        color!(black),
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
        color!(black),
        color!(black),
        color!(black),
    ]);

    agb::display::tiled::VRAM_MANAGER.set_background_palettes(&[palette_data]);

    // Center background like in color wheel
    bg.set_scroll_pos((8, 16));

    // HELLO WORLD letter patterns (8x8 pixels each)
    let h_pattern = [
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
    ];

    let e_pattern = [
        [1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let l_pattern = [
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ];

    let o_pattern = [
        [0, 1, 1, 1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [0, 1, 1, 1, 1, 1, 1, 0],
    ];

    let space_pattern = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let w_pattern = [
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 0, 0, 1],
        [1, 0, 1, 0, 0, 1, 0, 1],
        [1, 1, 0, 0, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
    ];

    let r_pattern = [
        [1, 1, 1, 1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 0],
        [1, 0, 1, 0, 0, 0, 0, 0],
        [1, 0, 0, 1, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0],
        [1, 0, 0, 0, 0, 1, 1, 1],
    ];

    let d_pattern = [
        [1, 1, 1, 1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 0],
    ];

    let create_tile = |pattern: &[[u8; 8]; 8]| -> agb::display::tiled::DynamicTile16 {
        let mut tile = agb::display::tiled::DynamicTile16::new();
        for y in 0..8 {
            for x in 0..8 {
                tile.set_pixel(x, y, pattern[y][x]);
            }
        }
        tile
    };

    // Create tiles for each letter
    let h_tile = create_tile(&h_pattern);
    let e_tile = create_tile(&e_pattern);
    let l1_tile = create_tile(&l_pattern);
    let l2_tile = create_tile(&l_pattern);
    let o_tile = create_tile(&o_pattern);
    let space_tile = create_tile(&space_pattern);
    let w_tile = create_tile(&w_pattern);
    let o2_tile = create_tile(&o_pattern);
    let r_tile = create_tile(&r_pattern);
    let l3_tile = create_tile(&l_pattern);
    let d_tile = create_tile(&d_pattern);

    // Position letters to spell "HELLO WORLD" (centered like color wheel)
    // Screen center is at tile (16, 12) with scroll offset
    // Text spans 21 tiles (11 letters + 10 spaces), so start at x = 16 - 10 = 6
    let center_x = 16;
    let center_y = 12;
    let text_start_x = center_x - 10; // 21 tiles / 2 = 10.5, rounded down
    
    bg.set_tile_dynamic16((text_start_x, center_y), &h_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 2, center_y), &e_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 4, center_y), &l1_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 6, center_y), &l2_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 8, center_y), &o_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 10, center_y), &space_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 12, center_y), &w_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 14, center_y), &o2_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 16, center_y), &r_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 18, center_y), &l3_tile, agb::display::tiled::TileEffect::default());
    bg.set_tile_dynamic16((text_start_x + 20, center_y), &d_tile, agb::display::tiled::TileEffect::default());

    loop {
        let mut frame = gfx.frame();
        bg.show(&mut frame);
        frame.commit();
    }
}
