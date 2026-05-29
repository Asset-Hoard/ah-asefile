# ah-asefile

[![Build](https://github.com/Asset-Hoard/ah-asefile/actions/workflows/rust.yml/badge.svg)](https://github.com/Asset-Hoard/ah-asefile/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/ah-asefile.svg)](https://crates.io/crates/ah-asefile)
[![Documentation](https://docs.rs/ah-asefile/badge.svg)](https://docs.rs/ah-asefile)

Load [Aseprite](https://www.aseprite.org/) files directly from their binary format — no JSON export step required. Fast enough to load assets at game boot or wire into an asset pipeline.

Used primarily in [AssetHoard](https://assethoard.com) to load Aseprite assets, including animations. 

Fork of [`alpine-alpaca/asefile`](https://github.com/alpine-alpaca/asefile), maintained here because upstream is inactive. Adds gamma/ICC color profile support and updated dependencies.

## Install

```toml
[dependencies]
ah-asefile = "0.4"
```

For acceptable dev-mode performance, override the opt-level:

```toml
[profile.dev.package.ah-asefile]
opt-level = 2
```

## Examples

**Load a file:**

```rust
use ah_asefile::AsepriteFile;

let ase = AsepriteFile::read_file("sprite.aseprite")?;
println!("{}x{}, {} frames", ase.width(), ase.height(), ase.num_frames());
```

**Export each frame as PNG:**

```rust
use ah_asefile::AsepriteFile;
use image::ImageFormat;

let ase = AsepriteFile::read_file("sprite.aseprite")?;
for frame in 0..ase.num_frames() {
    ase.frame(frame)
        .image()
        .save_with_format(format!("frame_{frame}.png"), ImageFormat::Png)?;
}
```

**Read a single layer:**

```rust
let layer = ase.layer_by_name("background").unwrap();
let img = ase.layer_image(0, layer.id());
```

**Read tags (animations):**

```rust
for tag in ase.tags() {
    println!("{}: frames {}..={}", tag.name(), tag.from_frame(), tag.to_frame());
}
```

See the [API docs](https://docs.rs/ah-asefile) for slices, palettes, cels, and tilesets.

## Quirks

- **Indexed-color blend modes** match the in-editor preview, not Aseprite's PNG export (which ignores them).
- **Luminance/color blend modes** reproduce an Aseprite bug — output matches the editor exactly. If upstream Aseprite fixes it, this crate will gate the fix on file version.

## License

MIT — see [LICENSE](LICENSE). Original copyright © alpine-alpaca; fork modifications © Mark Gandolfo.
