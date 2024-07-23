use crate::framebuffer::Framebuffer;
use crate::line::Line;

// Definir un trait para dibujar polígonos
pub trait Polygon {
    fn polygon(&mut self, points: &[(usize, usize)]);
    fn fill_polygon(&mut self, points: &[(usize, usize)]);
}

// Implementar el trait Polygon para Framebuffer
impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() {
            let (x0, y0) = points[i];
            let (x1, y1) = points[(i + 1) % points.len()]; // Conectar el último punto con el primero

            self.line(x0, y0, x1, y1);
        }
    }

    fn fill_polygon(&mut self, points: &[(usize, usize)]) {
        if points.len() < 3 {
            return;
        }

        let mut ymin = points[0].1;
        let mut ymax = points[0].1;

        // Encontrar los límites verticales del polígono
        for point in points {
            if point.1 < ymin {
                ymin = point.1;
            }
            if point.1 > ymax {
                ymax = point.1;
            }
        }

        // Algoritmo de escaneo de líneas
        for y in ymin..=ymax {
            let mut intersections = vec![];

            for i in 0..points.len() {
                let (x0, y0) = points[i];
                let (x1, y1) = points[(i + 1) % points.len()];

                if y0 == y1 {
                    continue; // Saltar líneas horizontales
                }

                // Asegurarse de que las comparaciones sean correctas y evitar desbordamientos
                if (y0 <= y && y < y1) || (y1 <= y && y < y0) {
                    let x = x0 as f32 + (y as f32 - y0 as f32) * (x1 as f32 - x0 as f32) / (y1 as f32 - y0 as f32);
                    intersections.push(x as usize);
                }
            }

            intersections.sort();

            // Dibujar líneas de relleno entre pares de intersecciones
            for i in (0..intersections.len()).step_by(2) {
                if i + 1 < intersections.len() {
                    self.line(intersections[i], y, intersections[i + 1], y);
                }
            }
        }
    }
}
