use std::fmt;
use super::tile::{Variant, Tile, SimpleTile};

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

    pub fn update(&self, pos: (u16, u16), grid: [SimpleTile; 9]) -> Behaviour {
        let mut current_move = (Behaviour::Wait, 1000.0);
        let (pos_x, pos_y) = (pos.0 as f32, pos.1 as f32);
        let current_tile = grid[4].get_variant();
        if current_tile == Variant::Aisle || current_tile == Variant::Entrance {
            for coords in &[
                (1.0, 1.0, Behaviour::Wait),
                (0.0, 1.0, Behaviour::Move_West),
                (1.0, 0.0, Behaviour::Move_North),
                (1.0, 2.0, Behaviour::Move_South),
                (2.0, 1.0, Behaviour::Move_East)] {
                
                let (dest_x, dest_y) = (pos_x + ((-1.0 + coords.0)), (pos_y + (-1.0 + coords.1)));
                
                if let mut target_seat = self.seat.unwrap() {
                    let target_seat = (target_seat.0 as f32, target_seat.1 as f32);

                    println!("Target seat: {},{}", target_seat.0, target_seat.1);

                    let new_distance = ((target_seat.0 - dest_x).abs() + (target_seat.1 - dest_y).abs());

                    if new_distance < current_move.1 {
                        if grid[(coords.1 * 3.0 + coords.0) as usize].get_variant() != Variant::Seat || dest_y == target_seat.1 {
                            // TODO: Make maths in this if less risky   
                            current_move = ((coords.2), new_distance);
                            println!("NEW MOVE: {:?} x {} -> {:?}", coords.2, new_distance, grid[(coords.1 * 3.0 + coords.0) as usize].get_variant());
                            println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
                        } else {
                            println!("REJECTED: {:?} x {} -> {:?}", coords.2, new_distance, grid[(coords.1 * 3.0 + coords.0) as usize].get_variant());
                            println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
                        }
                    } else {
                        println!("REJECTED: {:?} x {} -> {:?}", coords.2, new_distance, grid[(coords.1 * 3.0 + coords.0) as usize].get_variant());
                        println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
                    }
                } else {
                    // No target seat
                }
                println!("Checked {}, {}", dest_x, dest_y);
            }
        } else if current_tile == Variant::Seat {
            for coords in &[
                (1.0, 1.0, Behaviour::Wait),
                (0.0, 1.0, Behaviour::Move_West),
                (2.0, 1.0, Behaviour::Move_East),
            ] {
                let (dest_x, dest_y) = (pos_x + ((-1.0 + coords.0)), (pos_y + (-1.0 + coords.1)));

                if let mut target_seat = self.seat.unwrap() {
                    let target_seat = (target_seat.0 as f32, target_seat.1 as f32);

                    println!("Target seat: {},{}", target_seat.0, target_seat.1);

                    let new_distance = ((target_seat.0 - dest_x).abs() + (target_seat.1 - dest_y).abs());

                    if new_distance < current_move.1 {
                        if grid[(coords.1 * 3.0 + coords.0) as usize].get_variant() != Variant::Seat || dest_y == target_seat.1 {
                            // TODO: Make maths in this if less risky   
                            current_move = ((coords.2), new_distance);
                            println!("NEW MOVE: {:?} x {} -> {:?}", coords.2, new_distance, grid[(coords.1 * 3.0 + coords.0) as usize].get_variant());
                            println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
                        } else {
                            println!("REJECTED: {:?} x {} -> {:?}", coords.2, new_distance, grid[(coords.1 * 3.0 + coords.0) as usize].get_variant());
                            println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
                        }
                    } else {
                        println!("REJECTED: {:?} x {} -> {:?}", coords.2, new_distance, grid[(coords.1 * 3.0 + coords.0) as usize].get_variant());
                        println!("That was at surroundings[{}]", (coords.1 * 3.0 + coords.0) as usize);
                    }
                } else {
                    // No target seat
                }
                println!("Checked {}, {}", dest_x, dest_y);
                // React
            }
        }
        
        return current_move.0;
    }

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

    #[test]
    fn update() {
        let mut person = Person::new("Dave");
        let grid = [SimpleTile::new(&Tile::aisle());9];
        
        person.target_seat(1, 1);
        assert_eq!(person.update((0, 1), grid), Behaviour::Move_East, "Didn't move as expected");
        assert_eq!(person.update((1, 0), grid), Behaviour::Move_South, "Didn't move as expected");
    }
}
