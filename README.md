# boids-rust

A flocking simulation built with Rust and [Bevy](https://bevyengine.org/), implementing Craig Reynolds' Boids algorithm.

## About

400 boids flock using three classic steering rules:

- **Alignment** — match the heading of nearby boids
- **Cohesion** — steer toward the center of the local flock
- **Separation** — avoid crowding neighbors

Boids have a 270° field of view (blind spot directly behind) and wrap around the screen edges.

## Controls

### Parameter display

| Key | Action |
|-----|--------|
| Tab | Toggle parameter overlay |

### Tuning (hold Shift for 10× step)

| Keys | Parameter |
|------|-----------|
| Q / A | Perception radius |
| W / S | Separation radius |
| E / D | Alignment weight |
| R / F | Cohesion weight |
| T / G | Avoidance weight |

## Building

Requires [Rust](https://rustup.rs/) (edition 2024).

```
cargo run
```

For faster incremental compilation, dynamic linking is enabled by default in the dev profile.

## Dependencies

- [Bevy](https://bevyengine.org/) 0.18
- [rand](https://docs.rs/rand) 0.10

## License

MIT — see [LICENSE](LICENSE)
