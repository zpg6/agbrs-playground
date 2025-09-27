# agbrs-playground

Examples of Game Boy Advance game development with Rust using [agbrs](https://agbrs.dev).

## Examples

### Blocking agb Examples

- `hello_world` - Text rendering
  - <img src="./docs/hello_world.gif" width=200 />
- `hello_world_ttf` - Text rendering with TTF font
  - <img src="./docs/hello_world_ttf.gif" width=200 />
- `color_test` - Color Wheel with slices of a few RGB colors
  - <img src="./docs/color_test.gif" width=200 />
- `color_spin` - Spin the Color Wheel continuously
  - <img src="./docs/color_spin.gif" width=200 />
- `simple_sprite` - Basic sprite display using aseprite
  - <img src="./docs/simple_sprite.gif" width=200 />

### Async Examples (embassy-agb)

- `moving_sprite_hold` - Async sprite movement with button holding support
  - <img src="./docs/moving_sprite_hold.gif" width=200 />
- `moving_square` - Async sprite movement with button press detection
  - <img src="./docs/moving_square.gif" width=200 />
- `moving_square_hold` - Async sprite movement with button holding support
  - <img src="./docs/moving_square_hold.gif" width=200 />

> [!NOTE]
> These async examples accompany [agb PR #1089](https://github.com/agbrs/agb/pull/1089) which adds async support to agb. I brought in Embassy as the async executor and implemented async for inputs, display, and time drivers.

## Quick Start

```sh
# Install prerequisites
cargo install agb-gbafix

# Run a blocking example (requires mgba-qt in PATH)
cargo run --bin hello_world

# Run an async example (requires mgba-qt in PATH)
cargo run --bin moving_square
cargo run --bin moving_square_hold

# Build for real hardware
cargo build --release --bin hello_world
agb-gbafix target/thumbv4t-none-eabi/release/hello_world -o hello_world.gba
```

## Resources

- [agb documentation](https://docs.rs/agb/latest/agb/)
- [agbrs book](https://agbrs.dev/book/)
- [mGBA emulator](https://mgba.io)
- ⭐️ [zpg6/agbrs-capture](https://github.com/zpg6/agbrs-capture) - For capturing GIFs of projects and examples

## License

[MIT](./LICENSE)

## Contributing

Contributions are welcome! Whether it's bug fixes, feature additions, or documentation improvements, we appreciate your help in making this project better. For major changes or new features, please open an issue first to discuss what you would like to change.
