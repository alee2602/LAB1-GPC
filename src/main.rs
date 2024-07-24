extern crate nalgebra_glm as glm;

mod framebuffer;
mod line;
mod bmp;
mod polygon;
mod color;

use crate::polygon::Polygon;
use crate::framebuffer::Framebuffer;
use crate::bmp::WriteBmp;
use crate::color::Color;


fn main() {
    // Crear un nuevo framebuffer con un tamaño de 800x600 píxeles
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::black());
    framebuffer.clear();


    //Polígono 1


    // Definir los vértices de un polígono
    let poly1 = vec![
        (165, 380), (185, 360), (180, 330), 
        (207, 345), (233, 330), (230, 360), 
        (250, 380), (220, 385), (205, 410), 
        (193, 383)
    ];

    // Configurar el color para rellenar el polígono
    framebuffer.set_current_color(Color::yellow());
    // Rellenar el polígono en el framebuffer
    framebuffer.fill_polygon(&poly1);

    // Configurar el color para el contorno
    framebuffer.set_current_color(Color::white());
    // Dibujar el polígono en el framebuffer
    framebuffer.polygon(&poly1);

    //Renderizar el Framebuffer para crear imágen
    let _ = framebuffer.render_buffer("output.bmp");
    // Imprimir un mensaje indicando que el framebuffer se ha renderizado
    println!("Framebuffer rendered to output.bmp");
}