use std::fmt;
use super::person::Person;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Variant {
    Aisle,
    Seat,
    Entrance,
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

    pub fn occupy(&mut self, p: Person) {
        self.occupier = Some(p);
    }

    pub fn get_variant(&self) -> Variant {
        self.variant
    }

    pub fn get_occupier(&mut self) -> Option<&mut Person> {
        return self.occupier.as_mut();
    }
}

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

        tile.get_occupier().unwrap().set_name("Bert");
        assert_eq!(tile.get_occupier().unwrap().get_name(), "Bert");
    }
}
