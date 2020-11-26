// mod tile;

use std::fmt;
use super::tile::Variant;

#[derive(Debug, PartialEq)]
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

    fn target_seat(&mut self, x: u16, y: u16) {
        self.seat = Some((x, y));
    }

    pub fn update(&self, pos: (u16, u16), grid: &[Variant]) -> Behaviour {
        // TODO: Add logic
        return Behaviour::Wait;
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
        let person = Person::new("Dave");
        let grid = [Variant::Aisle;9];
        assert_eq!(person.update((0, 0), &grid), Behaviour::Wait);
    }
}
