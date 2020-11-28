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
}

impl Tile {
    pub fn aisle() -> Tile {
        Tile {
            variant: Variant::Aisle,
            occupier: None,
        }
    }

    pub fn seat() -> Tile {
        Tile {
            variant: Variant::Seat,
            occupier: None,
        }
    }

    pub fn entrance() -> Tile {
        Tile {
            variant: Variant::Entrance,
            occupier: None,
        }
    }

    pub fn none() -> Tile {
        Tile {
            variant: Variant::None,
            occupier: None,
        }
    }

    pub fn occupy(&mut self, p: Person) {
        self.occupier = Some(p);
    }

    pub fn is_occupied(&self) -> bool {
        match self.occupier {
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
}
