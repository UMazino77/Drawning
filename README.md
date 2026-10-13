# 🎨 Drawing — Geometrical Shapes in Rust

A small Rust project that draws geometrical shapes (points, lines, triangles, rectangles and circles) onto a blank image and saves the result as a PNG. It was built to practice **traits**, **modules**, and **testing** in Rust.

## ✨ Features

- `Point`, `Line`, `Triangle`, `Rectangle`, `Circle`
- Random generation for `Point`, `Line` and `Circle` (always inside the image bounds)
- Each shape is drawn in a different color
- Two custom traits:
  - `Drawable` — `draw` and `color`
  - `Displayable` — `display` (implemented for `raster::Image` in `main.rs`)

## 🗂️ Project Structure

```
.
├── Cargo.toml
├── src
│   ├── main.rs                 # entry point, creates the image and draws shapes
│   └── geometrical_shapes.rs   # shapes, traits and unit tests
└── image.png                   # generated output
```

## 🧩 Design

### Traits

```rust
pub trait Drawable {
    fn draw(&self, image: &mut impl Displayable);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
```

- `Drawable` describes anything that can render itself and knows its color.
- `Displayable` abstracts the canvas: shapes only need a way to put a pixel somewhere, so they don't depend directly on the `raster` crate.

### Shapes

| Shape       | Constructor                                        | `random` |
|-------------|----------------------------------------------------|:--------:|
| `Point`     | `Point::new(x: i32, y: i32)`                       | ✅ |
| `Line`      | `Line::new(&Point, &Point)`                        | ✅ |
| `Triangle`  | `Triangle::new(&Point, &Point, &Point)`            | ❌ |
| `Rectangle` | `Rectangle::new(&Point, &Point)`                   | ❌ |
| `Circle`    | `Circle::new(&Point, radius: i32)`                 | ✅ |

- `Line` is drawn with Bresenham's algorithm.
- `Triangle` is made of three lines.
- `Rectangle` normalizes its two corners (min/max), so the order of the points does not matter.
- `Circle` uses the midpoint circle algorithm.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)

### Dependencies

```toml
[dependencies]
raster = "0.2.0"
rand = "0.8"
```

### Run

```bash
git clone <https://github.com/UMazino77/Drawning>
cd Drawning
cargo run
```

This generates `image.png` in the project root.


## 📚 What I Learned

- Defining and implementing **traits**, including implementing a trait for an external type (`Image`)
- Organizing code with **modules**
- Basic rasterization algorithms (Bresenham, midpoint circle)

## 🔗 Resources

- [raster crate docs](https://docs.rs/raster/0.2.0/raster/)
- [The Rust Book — Traits](https://doc.rust-lang.org/stable/book/ch10-02-traits.html)

## 👤 Author

**Mohamed ATMANI** — [GitHub](https://github.com/UMazino77) · Zone01 Oujda
