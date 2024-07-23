# LAB1-GPC
This program is capable of drawing and filling polygons on a framebuffer. With a custom Color class, you can define and use your own colors easily.

- **Draw and Fill Polygons**: Create and manipulate polygons on a framebuffer.
- **Custom Colors**: Define and use custom colors with ease.

## Example Usage

### Define a Custom Color
**Color.rs**
```rust
impl Color {
    pub fn purple() -> Self {
        Self::new(128, 0, 128)
    }
}
```
### Implementation
*main.rs*
``` rust
fn main() {
    // Create a framebuffer
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_current_color(Color::white());

    // Define the vertices of a polygon
    let poly1 = vec![
        (165, 380), (185, 360), (180, 330), 
        (207, 345), (233, 330), (230, 360), 
        (250, 380), (220, 385), (205, 410), 
        (193, 383)
    ];

    // Draw the polygon on the framebuffer
    framebuffer.polygon(&poly1);

    // Set the current drawing color to purple
    framebuffer.set_current_color(Color::purple());

    // Fill the polygon on the framebuffer
    framebuffer.fill_polygon(&poly1);

    // Render the content of the framebuffer to a BMP file named "output.bmp"
    let _ = framebuffer.render_buffer("output.bmp");
}
```
## Branch Information
The main branch is empty, but you can find the code and outputs in the following branches:

- **Polygon-1**
- **Polygon-2**
- **Polygon-3**
- **Polygon-4**

Each branch contains specific implementations and the resulting output.bmp
