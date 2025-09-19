#![no_std]

//! Shared utilities for GBA development
/// Macro to convert hex color codes (#RRGGBB) to GBA RGB15 format
#[macro_export]
macro_rules! rgb15 {
    ($hex:expr) => {{
        const HEX: u32 = $hex;
        const R: u32 = (HEX >> 16) & 0xFF;
        const G: u32 = (HEX >> 8) & 0xFF;
        const B: u32 = HEX & 0xFF;

        // Convert 8-bit RGB to 5-bit RGB15
        // GBA RGB15 format: 0b0BBBBBGGGGGRRRRR
        const R5: u32 = (R >> 3) & 0x1F;
        const G5: u32 = (G >> 3) & 0x1F;
        const B5: u32 = (B >> 3) & 0x1F;

        agb::display::Rgb15::new(((B5 << 10) | (G5 << 5) | R5) as u16)
    }};
}

/// Convenience macro for common colors using hex format
#[macro_export]
macro_rules! color {
    (red) => {
        $crate::rgb15!(0xFF0000)
    };
    (green) => {
        $crate::rgb15!(0x00FF00)
    };
    (blue) => {
        $crate::rgb15!(0x0000FF)
    };
    (yellow) => {
        $crate::rgb15!(0xFFFF00)
    };
    (magenta) => {
        $crate::rgb15!(0xFF00FF)
    };
    (cyan) => {
        $crate::rgb15!(0x00FFFF)
    };
    (white) => {
        $crate::rgb15!(0xFFFFFF)
    };
    (black) => {
        $crate::rgb15!(0x000000)
    };
    (gray) => {
        $crate::rgb15!(0x808080)
    };
    (dark_gray) => {
        $crate::rgb15!(0x404040)
    };
    (light_gray) => {
        $crate::rgb15!(0xC0C0C0)
    };
    (orange) => {
        $crate::rgb15!(0xFF8000)
    };
    (purple) => {
        $crate::rgb15!(0x8000FF)
    };
    (pink) => {
        $crate::rgb15!(0xFF80FF)
    };
    (brown) => {
        $crate::rgb15!(0x804000)
    };
    (lime) => {
        $crate::rgb15!(0x80FF00)
    };
    (teal) => {
        $crate::rgb15!(0x008080)
    };
}
