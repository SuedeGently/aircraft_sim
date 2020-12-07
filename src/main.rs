mod aircraft;

use aircraft::Aircraft;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston::{ButtonEvent, RenderEvent};
use find_folder;
use std::rc::Rc;
use sdl2_window::Sdl2Window;
use opengl_graphics::*;

use std::io;
use std::io::prelude::*;

type Colour = [f32; 4];

const WINDOW_SIZE: i32 = 320;
const PIXEL_SIZE: f64 = 32.0;
const AIRCRAFT_SIZE: u16 = 10;

const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0, 1.0, 1.0, 1.0];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const GREY: Colour = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;

    let settings = WindowSettings::new("Aircraft Boarding Visualiser", (WINDOW_SIZE as f64, WINDOW_SIZE as f64))
        .exit_on_esc(true)
        .graphics_api(opengl);
    let mut window: Sdl2Window = settings.build()
        .expect("Failed to create window!");

    let settings = EventSettings::new();
    let mut events = Events::new(settings);

    let mut aircraft = Aircraft::new(AIRCRAFT_SIZE, AIRCRAFT_SIZE);
    for i in 0..AIRCRAFT_SIZE {
        aircraft.set_tile(0, i, "seat");
        aircraft.set_tile(1, i, "seat");
        aircraft.set_tile(AIRCRAFT_SIZE - 2, i, "seat");
        aircraft.set_tile(AIRCRAFT_SIZE - 1, i, "seat");

        aircraft.easy_add_passenger("DAVE", Some((0, i)));
        aircraft.easy_add_passenger("DAVE", Some((AIRCRAFT_SIZE - 1, i)));
    }
    aircraft.set_tile(2, AIRCRAFT_SIZE - 1, "entrance");
    
    let mut gl = GlGraphics::new(opengl);
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                // let transform = c.transform.trans(1.0, 1.0);

                clear(WHITE, g);


                for y in 0..AIRCRAFT_SIZE {
                    for x in 0..AIRCRAFT_SIZE {
                        let pos: [f64;4] = [
                            PIXEL_SIZE * x as f64,
                            PIXEL_SIZE * y as f64,
                            PIXEL_SIZE * (x + 1) as f64,
                            PIXEL_SIZE * (y + 1) as f64,
                        ];
                        
                        let colour: Colour;

                        if aircraft.get_tile(x, y).is_occupied() {
                            colour = BLACK;
                        } else {
                            colour = 
                                match aircraft.get_colour(x, y) {
                                    "red" => RED,
                                    "grey" => GREY,
                                    _ => WHITE,
                                };
                        }
                        graphics::Rectangle::new(colour)
                            .draw(
                                pos,
                                &c.draw_state,
                                c.transform,
                                g,
                            );
                    }
                }
            });
        } else if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match k.button {
                    Button::Keyboard(Key::Up) => {
                        aircraft.update();
                    },
                    Button::Keyboard(Key::Down) => {
                        // aircraft.layout[5][9].occupy(Person::new());
                    },
                    _ => { println!("Not yet implemented"); },
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let aircraft = Aircraft::new(5, 5);
        assert_eq!(aircraft.get_size(), (5, 5));
    }
}
