#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agb::{
    display::{
        font::{AlignmentKind, Font, Layout, RegularBackgroundTextRenderer},
        tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER},
        Priority, Rgb15,
    },
    include_font,
};

// Include the TTF font at compile time
static FONT: Font = include_font!("fnt/ark-pixel-10px-proportional-latin.ttf", 10);

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();

    // Set up background palette - index 1 will be white for text
    VRAM_MANAGER.set_background_palette_colour(0, 1, Rgb15::WHITE);

    // Create a regular background for text rendering
    let mut bg = RegularBackground::new(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    // Create text renderer starting at tile position (4, 8) - near the top left corner
    let mut text_renderer = RegularBackgroundTextRenderer::new((4, 8));

    // Create text layout with "Hello World!" message
    let mut text_layout = Layout::new(
        "Hello World!\nTTF Font Example",
        &FONT,
        AlignmentKind::Left,
        32,  // minimum group size
        200, // max width
    );

    // Show the background initially
    let mut frame = gfx.frame();
    bg.show(&mut frame);
    frame.commit();

    // Render text letter by letter over multiple frames
    // This prevents frame skipping and distributes the rendering workload
    loop {
        if let Some(letter_group) = text_layout.next() {
            text_renderer.show(&mut bg, &letter_group);
        }

        let mut frame = gfx.frame();
        bg.show(&mut frame);
        frame.commit();
    }
}
