use crate::framebuffer::Framebuffer;

// Definir un trait para dibujar líneas
pub trait Line {
    // Método para dibujar una línea desde (x1, y1) hasta (x2, y2)
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize);
}

// Implementar el trait Line para Framebuffer
impl Line for Framebuffer {
    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        // Calcular la diferencia en las coordenadas
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = -(y2 as i32 - y1 as i32).abs();

        // Determinar la dirección del paso para x e y
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        
        // Inicializar el error
        let mut err = dx + dy;

        // Inicializar las coordenadas actuales
        let mut x = x1 as i32;
        let mut y = y1 as i32;

        // Bucle para dibujar la línea
        loop {
            // Dibujar un punto en las coordenadas actuales
            self.point(x as usize, y as usize);

            // Si se ha llegado al punto final, salir del bucle
            if x == x2 as i32 && y == y2 as i32 {
                break;
            }

            // Duplicar el error para las comparaciones
            let e2 = 2 * err;
            
            // Ajustar las coordenadas y el error según el algoritmo de Bresenham
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}