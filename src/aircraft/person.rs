// mod tile;

use std::fmt;
use super::tile::{Variant, Tile, SimpleTile};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Behaviour {
    Move_North,
    Move_South,
    Move_East,
    Move_West,
    Stow,
    Wait,
}

pub struct Person {
    name: String,
    seat: Option<(u16, u16)>,
    baggage: bool,
}

impl Person {
    pub fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
            seat: None,
            baggage: false,
        }
    }

    pub fn target_seat(&mut self, x: u16, y: u16) {
        self.seat = Some((x, y));
    }

    pub fn remove_baggage(&mut self) {
        if self.baggage {
            log::info!("Removing baggage");
            self.baggage = false;
        } else {
            log::warn!("Invalid call to `remove_baggage()`");
        }
    }

    // pub fn check_for_delay(&self) -> Option<Behaviour> {

    pub fn has_baggage(&self) -> bool {
        self.baggage
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

    pub fn set_baggage(&mut self, t: bool) {
        self.baggage = t;
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
