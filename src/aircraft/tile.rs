use std::fmt;
use super::person::Person;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Variant {
    Aisle,
    Seat,
    Entrance,
    None,
}

pub struct Tile {
    variant: Variant,
    occupier: Option<Person>,
    updated: bool,
    pass_counter: u8,
    allowing: Option<Person>,
}

impl Tile {
    pub(crate) fn aisle() -> Tile {
        Tile {
            variant: Variant::Aisle,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    pub fn seat() -> Tile {
        Tile {
            variant: Variant::Seat,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    pub fn entrance() -> Tile {
        Tile {
            variant: Variant::Entrance,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    pub fn none() -> Tile {
        Tile {
            variant: Variant::None,
            occupier: None,
            updated: false,
            pass_counter: 0,
            allowing: None,
        }
    }

    pub fn occupy(&mut self, p: Person) {
        self.occupier = Some(p);
        self.updated = true;
    }

    pub fn pass_in(&mut self, p: Person) {
        self.allowing = Some(p);
    }

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

    pub fn get_variant(&self) -> Variant {
        self.variant
    }

    pub fn get_occupier(&mut self) -> Option<&Person> {
        return self.occupier.as_ref();
    }

    pub fn get_passer(&self) -> Option<&Person> {
        self.allowing.as_ref()
    }

    pub fn has_updated(&self) -> bool {
        self.updated
    }

    pub fn set_updated(&mut self, t: bool) {
        self.updated = t;
    }

    pub fn free(&mut self) -> Option<Person> {
        let mut occupier = Person::new(&self.occupier.as_ref().unwrap().get_name());
        let target_seat = self.occupier.as_ref().unwrap().get_seat().unwrap();
        occupier.target_seat(target_seat.0, target_seat.1);
        self.occupier = None;
        return Some(occupier);
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tile")
            .field("variant", &self.variant)
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct SimpleTile {
    variant: Variant,
    occupied: bool,
}

impl SimpleTile {
    pub fn new(t: &Tile) -> SimpleTile {
        SimpleTile {
            variant: t.get_variant(),
            occupied: t.is_occupied(),
        }
    }

    pub fn empty() -> SimpleTile {
        SimpleTile {
            variant: Variant::None,
            occupied: false,
        }
    }

    pub fn get_variant(&self) -> Variant {
        return self.variant;
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
