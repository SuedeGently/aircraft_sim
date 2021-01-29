//! Holds structures and methods useful for passengers and movement.
//!
//! Mostly responsible for providing `Aircraft` with the information necessary
//! to determine this passenger's optimal move.

use std::fmt;
// use super::tile::{Variant, Tile, SimpleTile};

/// This enum represents a possible move for a pasenger during an update.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Behaviour {
    Move_North,
    Move_South,
    Move_East,
    Move_West,
    Stow, // stow baggage
    Wait,
}

/// A single passenger
///
/// `seat` represents a passenger's assigned seat, and baggage stores whether
/// the passenger still has their carry-on luggage or not.
pub struct Person {
    name: String,
    seat: Option<(u16, u16)>,
    baggage: bool,
}

impl Person {
    /// Constructor
    ///
    /// All passengers start with no target seat or luggage.
    ///
    /// # Examples
    ///
    /// ```
    /// let person = Person::new("DEFAULT");
    ///
    /// assert_eq!(person.get_name(), "DEFAULT");
    /// ```
    pub fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
            seat: None,
            baggage: false,
        }
    }
    
    /// Sets a passenger's target seat.
    ///
    /// # Examples
    ///
    /// ```
    /// let person = Person::new("DEFAULT");
    ///
    /// person.target_seat(2,2);
    ///
    /// assert_eq!(person.get_seat(), (2,2));
    /// ```
    pub fn target_seat(&mut self, x: u16, y: u16) {
        self.seat = Some((x, y));
    }

    /// Sets a passenger's `baggage` value to false.
    pub fn remove_baggage(&mut self) {
        if self.baggage {
            log::info!("Removing baggage");
            self.baggage = false;
        } else {
            log::warn!("Invalid call to `remove_baggage()`");
        }
    }

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

/// Defines how Rust should display this object if it is passed to stdout via a
/// standard library macro like `println`.
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
