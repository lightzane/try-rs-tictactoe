# Try Rust TicTacToe

Developed with 🦀 Rust + WASM Game Engine used within a javascript environment.

Learning **Minimax** algorithm with **Alpha & Beta Pruning** to play games with the computer.

## Development and Testing

```bash
cargo run
```

## Wasm Pack for JS

Whenever changing the `/src`

```bash
wasm-pack build --target web
```

Then copy the generated files from `pkg` to `docs`
