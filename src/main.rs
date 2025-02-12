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



    //Polígono 2
    let poly2 = vec![
        
        (321, 335),
        (288, 286),
        (339, 251),
        (374, 302)
    ];

    framebuffer.set_current_color(Color::blue());
    framebuffer.fill_polygon(&poly2);

    framebuffer.set_current_color(Color::white());
    framebuffer.polygon(&poly2);


    //Polígono 3

    let poly3 = vec![
        
        (377, 249),
        (411, 197),
        (436, 249)
    ];

    framebuffer.set_current_color(Color::red());
    framebuffer.fill_polygon(&poly3);
    
    framebuffer.set_current_color(Color::white());
    framebuffer.polygon(&poly3);


    //Polígono 4 (Tetera)

    let poly4 = vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180)
    ];

    framebuffer.set_current_color(Color::green());
    framebuffer.fill_polygon(&poly4);

    framebuffer.set_current_color(Color::white());
    framebuffer.polygon(&poly4);



    //Polígono 5 (Agujero del polígono 4)

    let poly5 = vec![
        (682, 175 ),(708, 120 ),(735, 148 ), (739, 170)
    ];
    
    framebuffer.set_current_color(Color::black());
    framebuffer.fill_polygon(&poly5);

    framebuffer.set_current_color(Color::white());
    framebuffer.polygon(&poly5);


    //Renderizar el Framebuffer para crear imágen
    let _ = framebuffer.render_buffer("output.bmp");
    // Imprimir un mensaje indicando que el framebuffer se ha renderizado
    println!("Framebuffer rendered to output.bmp");
}