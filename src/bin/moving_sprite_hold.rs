//! Moving animated sprite example with button holding support
//!
//! Demonstrates async display + async input working together:
//! - VBlank-synchronized rendering for smooth 60Hz display
//! - Timer-based input polling for responsive controls (configurable rate)
//! - Embassy task coordination between input and display
//! - Animated sprite movement with position clamping
//! - Supports holding multiple buttons for diagonal movement
//! - Loading sprites from Aseprite files using `include_aseprite!`
//!
//! Controls: D-pad moves the animated crab, clamped to screen edges
//! - Hold buttons for continuous movement
//! - Hold multiple buttons for diagonal movement
//! Input polling: 60Hz (configurable from 30-120Hz)

#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

use agb::{display::object::Object, include_aseprite};
use embassy_agb::{
    agb::input::Button,
    input::{AsyncInput, InputConfig},
    sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex},
    Spawner,
};

// Import the crab sprites from the Aseprite file
include_aseprite!(mod sprites, "gfx/crab.aseprite");

// Shared button state between input task and main loop
#[derive(Clone, Copy, Default)]
struct ButtonState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl ButtonState {
    /// Calculate net movement from current button state
    fn net_movement(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;

        if self.left {
            x -= 1;
        }
        if self.right {
            x += 1;
        }
        if self.up {
            y -= 1;
        }
        if self.down {
            y += 1;
        }

        (x, y)
    }
}

static BUTTON_STATE: Mutex<CriticalSectionRawMutex, ButtonState> = Mutex::new(ButtonState {
    up: false,
    down: false,
    left: false,
    right: false,
});

// Input task: continuously poll button state and update shared state
#[embassy_executor::task]
async fn input_task(input: AsyncInput) {
    loop {
        // Poll current button state (non-blocking)
        let up_pressed = input.is_pressed(Button::UP);
        let down_pressed = input.is_pressed(Button::DOWN);
        let left_pressed = input.is_pressed(Button::LEFT);
        let right_pressed = input.is_pressed(Button::RIGHT);

        // Update shared state
        {
            let mut state = BUTTON_STATE.lock().await;
            state.up = up_pressed;
            state.down = down_pressed;
            state.left = left_pressed;
            state.right = right_pressed;
        }

        // Small delay to avoid excessive CPU usage
        embassy_agb::time::Timer::after(embassy_agb::time::Duration::from_millis(16)).await;
    }
}

#[embassy_agb::main]
async fn main(spawner: Spawner) -> ! {
    let mut gba = embassy_agb::init(Default::default());

    // Configure input polling at 60Hz
    let input_config = InputConfig { poll_rate: 60 };
    spawner.spawn(embassy_agb::input::input_polling_task(input_config).unwrap());

    let input = gba.input_with_config(input_config);
    let mut display = gba.display();

    // Sprite position and movement
    let mut crab_x = 120; // Center X
    let mut crab_y = 80; // Center Y
    const MOVE_SPEED: i32 = 4;
    const SPRITE_SIZE: i32 = 32; // Crab sprite is 32x32 pixels

    // Screen bounds
    const MIN_X: i32 = 0;
    const MAX_X: i32 = agb::display::WIDTH - SPRITE_SIZE;
    const MIN_Y: i32 = 0;
    const MAX_Y: i32 = agb::display::HEIGHT - SPRITE_SIZE;

    // Animation timing
    let mut frame_count = 0u32;
    const ANIMATION_FRAME_RATE: u32 = 10; // change animation frame every 10 VBlanks

    // Spawn input task
    spawner.spawn(input_task(input).unwrap());

    loop {
        // Wait for VBlank: ensures smooth rendering without tearing
        display.wait_for_vblank().await;

        // Get current button state and calculate net movement
        let (move_x, move_y) = {
            let state = BUTTON_STATE.lock().await;
            state.net_movement()
        };

        // Apply movement if any buttons are pressed
        if move_x != 0 || move_y != 0 {
            // Calculate new position with net movement
            crab_x += move_x * MOVE_SPEED;
            crab_y += move_y * MOVE_SPEED;

            // Clamp to screen bounds
            crab_x = crab_x.clamp(MIN_X, MAX_X);
            crab_y = crab_y.clamp(MIN_Y, MAX_Y);
        }

        // Calculate current animation frame
        let animation_frame = (frame_count / ANIMATION_FRAME_RATE) as usize;

        // Create sprite object with current animation frame and position
        let mut crab = Object::new(sprites::IDLE.animation_sprite(animation_frame));
        crab.set_pos((crab_x, crab_y));

        // Render the frame
        let mut frame = display.frame().await;
        crab.show(&mut frame);
        frame.commit();

        frame_count = frame_count.wrapping_add(1);
    }
}
