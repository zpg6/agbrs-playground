//! Simple animated sprite example
//!
//! Demonstrates basic sprite rendering with animation and physics:
//! - Loading sprites from Aseprite files using `include_aseprite!`
//! - Animated sprite rendering with frame timing
//! - Basic physics with bouncing movement
//! - VBlank-synchronized rendering for smooth display
//!
//! Features a bouncing animated crab that moves around the screen,
//! reversing direction when hitting screen edges.

#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agb::{display::object::Object, include_aseprite};

// Import the crab sprites from the Aseprite file
include_aseprite!(mod sprites, "gfx/crab.aseprite");

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut gfx = gba.graphics.get();

    // Sprite physics state
    let mut crab_x = 50;
    let mut crab_y = 50;
    let mut x_velocity = 1; // pixels per move
    let mut y_velocity = 1;

    // Animation timing
    let mut frame_count = 0u32;
    const MOVE_EVERY_N_FRAMES: u32 = 60;
    const ANIMATION_FRAME_RATE: u32 = 120;

    loop {
        // Update position based on frame count
        if frame_count % MOVE_EVERY_N_FRAMES == 0 {
            // Calculate new position
            let new_x = crab_x + x_velocity;
            let new_y = crab_y + y_velocity;

            // Crab sprite is 32x32 pixels
            const SPRITE_SIZE: i32 = 32;

            // Boundary collision and bouncing
            if new_x <= 0 {
                x_velocity = -x_velocity;
                crab_x = 0;
            } else if new_x >= agb::display::WIDTH - SPRITE_SIZE {
                x_velocity = -x_velocity;
                crab_x = agb::display::WIDTH - SPRITE_SIZE;
            } else {
                crab_x = new_x;
            }

            if new_y <= 0 {
                y_velocity = -y_velocity;
                crab_y = 0;
            } else if new_y >= agb::display::HEIGHT - SPRITE_SIZE {
                y_velocity = -y_velocity;
                crab_y = agb::display::HEIGHT - SPRITE_SIZE;
            } else {
                crab_y = new_y;
            }
        }

        // Calculate current animation frame based on frame count
        let animation_frame = (frame_count / ANIMATION_FRAME_RATE) as usize;

        // Create sprite object with current animation frame and position
        let mut crab = Object::new(sprites::IDLE.animation_sprite(animation_frame));
        crab.set_pos((crab_x, crab_y));

        // Wait for VBlank and render the frame
        let mut frame = gfx.frame();
        crab.show(&mut frame);
        frame.commit();

        frame_count = frame_count.wrapping_add(1);
    }
}
