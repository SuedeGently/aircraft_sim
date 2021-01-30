//! Holds structures, methods, and functions required for dealing with `Tile`s.

use std::fmt;
use super::person::Person;

const PASS_WAIT: u8 = 2; // The amount of additional steps taken when moving
                         // past an occupied seat

/// An enum representing the various possible types of tile that an `Aircraft`
/// may contain.
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Variant {
    Aisle,
    Seat,
    Entrance,
    None,
}

/// A single tile
///
/// Must have a `variant`, may hold one or two passengers. Two passengers are
/// only held when another passenger is making their way past on an aisle.
pub struct Tile {
    pub(crate) variant: Variant,
    occupier: Option<Person>,
    updated: bool,
    pass_counter: u8,
    allowing: Option<Person>,
}

impl Tile {
    /// Constructor with `variant` of `Aisle`.
    pub(crate) fn aisle() -> Tile {
        Tile {
            variant: Variant::Aisle,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    /// Constructor with `variant` of `Seat`.
    pub fn seat() -> Tile {
        Tile {
            variant: Variant::Seat,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    /// Constructor with `variant` of `Entrance`.
    pub fn entrance() -> Tile {
        Tile {
            variant: Variant::Entrance,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    /// Constructor with `variant` of `None`.
    pub fn none() -> Tile {
        Tile {
            variant: Variant::None,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    /// Places a passenger into this tile.
    pub fn occupy(&mut self, p: Person) {
        if self.is_occupied() {
            log::warn!("A passenger is being displaced");
        }
        self.occupier = Some(p);
        self.updated = true;
    }

    /// Allows a second passenger to temporarily occupy this space.
    pub fn pass_in(&mut self, p: Person) {
        self.allowing = Some(p);
    }

    /// Removes the second passenger occupying this space.
    pub fn pass_out(&mut self) -> Person {
        let person = self.allowing.take();
        return person.unwrap();
    }

    pub fn is_occupied(&self) -> bool {
        match self.occupier {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_allowing(&self) -> bool {
        match self.allowing {
            Some(_) => true,
            None => false,
        }
    }

    /// Checks whether second occupant has been delayed enough to pass onwards.
    pub fn pass_count(&mut self) -> bool {
        if self.pass_counter >= PASS_WAIT {
            self.pass_counter = 0;
            return true;
        } else {
            self.pass_counter = self.pass_counter + 1;
            return false;
        }
    }

    pub fn get_variant(&self) -> Variant {
        self.variant
    }

    pub fn get_occupier(&mut self) -> Option<&Person> {
        return self.occupier.as_ref();
    }

    pub fn get_occupier_as_mut(&mut self) -> Option<&mut Person> {
        return self.occupier.as_mut();
    }

    pub fn get_passer(&self) -> Option<&Person> {
        self.allowing.as_ref()
    }

    pub fn get_passer_as_mut(&mut self) -> Option<&mut Person> {
        return self.allowing.as_mut();
    }

    pub fn has_updated(&self) -> bool {
        self.updated
    }

    pub fn set_updated(&mut self, t: bool) {
        self.updated = t;
    }
    
    /// Removes this tile's occupant ready to move to another tile.
    pub fn free(&mut self) -> Option<Person> {
        let person = self.occupier.take();

        if self.allowing.is_some() {
            let passer = self.pass_out();
            self.occupier = Some(passer);
        }

        return person;
    }
}

/// Defines how this Tile should be formatted if it's passed to a standard
/// library macro like `println!()`.
impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
            .field("variant", &self.variant)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let mut tile = Tile::aisle();
        assert_eq!(tile.variant, Variant::Aisle);
        tile = Tile::seat();
        assert_eq!(tile.variant, Variant::Seat);
        tile = Tile::entrance();
        assert_eq!(tile.variant, Variant::Entrance);
    }

    #[test]
    #[should_panic]
    fn occupier() {
        let tile = Tile::aisle();
        tile.occupier.unwrap();
    }

    #[test]
    fn get_occupier() {
        let mut tile = Tile::aisle();
        let person = Person::new("Dave");
        tile.occupy(person);
        assert_eq!(tile.get_occupier().unwrap().get_name(), "Dave");

        // TODO: Fix this
        // tile.get_occupier().unwrap().set_name("Bert");
        // assert_eq!(tile.get_occupier().unwrap().get_name(), "Bert");
    }

    #[test]
    fn allow_to_pass() {
        let mut tile0 = Tile::aisle();
        let mut tile1 = Tile::aisle();
        let mut person = Person::new("DEFAULT");
        person.target_seat(0,0);

        tile0.occupy(person);
        assert!(
            tile0.is_occupied(),
            "Tile 0 was not occupied at initialisation");

        let temp = tile0.free().expect("No passenger was present in tile 0");

        tile1.pass_in(temp);
        assert_eq!(tile0.is_occupied(), false, "Tile 0 was still occupied");
        assert!(tile1.is_allowing(), "Tile 1 was not allowing");

        tile0.occupy(tile1.pass_out());
        assert!(tile0.is_occupied(), "Tile 0 was not occupied post pass");
        assert_eq!(tile1.is_allowing(), false, "Tile 1 was still allowing");
    }
}
