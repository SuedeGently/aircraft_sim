// mod tile;

use std::fmt;
use super::tile::{Variant, Tile};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Behaviour {
    Move_North,
    Move_South,
    Move_East,
    Move_West,
    Wait,
}

pub struct Person {
    name: String,
    seat: Option<(u16, u16)>,
}

impl Person {
    pub fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
            seat: None,
        }
    }

    pub fn target_seat(&mut self, x: u16, y: u16) {
        self.seat = Some((x, y));
    }

    // pub fn update(&self, pos: (u16, u16), grid: &Vec<Vec<SimpleTile>>) -> Behaviour {
    //     let mut current_move = (Behaviour::Wait, 1000.0);
    //     let (pos_x, pos_y) = (pos.0 as usize, pos.1 as usize);
    //     let current_tile = grid[pos_x][pos_y].get_variant();
    //     println!("I'm currently occupying a {:?}", current_tile);
    //     if current_tile == Variant::Aisle || current_tile == Variant::Entrance {
    //         for coords in &[
    //             (1.0, 1.0, Behaviour::Wait),
    //             (0.0, 1.0, Behaviour::Move_West),
    //             (1.0, 0.0, Behaviour::Move_North),
    //             (1.0, 2.0, Behaviour::Move_South),
    //             (2.0, 1.0, Behaviour::Move_East)] {
    //             
    //             let (dest_x, dest_y) = (pos_x as f32 + ((-1.0 + coords.0)), (pos_y as f32 + (-1.0 + coords.1)));
    //             
    //             if dest_x >= 0.0 && dest_x < grid.len() as f32 
    //             && dest_y >= 0.0 && dest_y < grid.len() as f32{
    //                 let target_seat = self.seat.unwrap();
    //                 let target_seat = (target_seat.0 as f32, target_seat.1 as f32);

    //                 println!("Target seat: {},{}", target_seat.0, target_seat.1);
    //                 
    //                 let new_distance = ((target_seat.0 - dest_x).abs() + (target_seat.1 - dest_y).abs());

    //                 if new_distance < current_move.1 {
    //                     if grid[dest_x as usize][dest_y as usize].get_variant() != Variant::Seat || dest_y == target_seat.1 {
    //                         // TODO: Make maths in this if less risky   
    //                         // Correct tile to move to.
    //                         current_move = ((coords.2), new_distance);
    //                         println!("NEW MOVE: {:?} x {}", coords.2, new_distance);
    //                         println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
    //                     } else {
    //                         println!("REJECTED: {:?} x {}", coords.2, new_distance);
    //                         println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
    //                     }
    //                 } else {
    //                     println!("REJECTED: {:?} x {}", coords.2, new_distance);
    //                     println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
    //                 }
    //             } else {
    //                 println!("Seat {},{} is out of bounds", dest_x, dest_y);
    //             }
    //             println!("Checked {}, {}", dest_x, dest_y);
    //         }
    //     } else if current_tile == Variant::Seat {
    //         for coords in &[
    //             (1.0, 1.0, Behaviour::Wait),
    //             (0.0, 1.0, Behaviour::Move_West),
    //             (2.0, 1.0, Behaviour::Move_East),
    //         ] {
    //             let (dest_x, dest_y) = (pos_x as f32 + ((-1.0 + coords.0)), (pos_y as f32 + (-1.0 + coords.1)));

    //             if let mut target_seat = self.seat.unwrap() {
    //                 let target_seat = (target_seat.0 as f32, target_seat.1 as f32);

    //                 println!("Target seat: {},{}", target_seat.0, target_seat.1);

    //                 let new_distance = ((target_seat.0 - dest_x).abs() + (target_seat.1 - dest_y).abs());

    //                 if new_distance < current_move.1 {
    //                     if grid[dest_x as usize][dest_y as usize].get_variant() != Variant::Seat || dest_y == target_seat.1 {
    //                         // TODO: Make maths in this if less risky   
    //                         current_move = ((coords.2), new_distance);
    //                         println!("NEW MOVE: {:?} x {}", coords.2, new_distance);
    //                         println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
    //                     } else {
    //                         println!("REJECTED: {:?} x {}", coords.2, new_distance);
    //                         println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
    //                     }
    //                 } else {
    //                     println!("REJECTED: {:?} x {}", coords.2, new_distance);
    //                     println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
    //                 }
    //             } else {
    //                 // No target seat
    //             }
    //             println!("Checked {}, {}", dest_x, dest_y);
    //             // React
    //         }
    //     }
    //     
    //     return current_move.0;
    // }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_seat(&self) -> Option<(u16, u16)> {
        self.seat
    }

    pub fn set_name(&mut self, n: &str) {
        self.name = n.to_string();
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .field("seat", &self.seat)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let person = Person::new("Dave");
        assert_eq!(person.name, "Dave");
        assert_eq!(person.seat, None);
    }
    
    #[test]
    fn target_seat() {
        let mut person = Person::new("Dave");
        person.target_seat(6, 9);
        
        assert_eq!(person.seat.unwrap(), (6, 9));
    }

    // #[test]
    // fn update() {
    //     let mut person = Person::new("Dave");
    //     let grid = [SimpleTile::new(&Tile::aisle());9];
    //     
    //     person.target_seat(1, 1);
    //     assert_eq!(person.update((0, 1), grid), Behaviour::Move_East, "Didn't move as expected");
    //     assert_eq!(person.update((1, 0), grid), Behaviour::Move_South, "Didn't move as expected");
    // }
}
