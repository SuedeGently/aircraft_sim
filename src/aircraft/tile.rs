use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Variant {
    Aisle,
    Seat,
    Entrance,
}

pub struct Tile {
    variant: Variant,
}

impl Tile {
    pub fn aisle() -> Tile {
        Tile {
            variant: Variant::Aisle,
        }
    }

    pub fn seat() -> Tile {
        Tile {
            variant: Variant::Seat,
        }
    }

    pub fn entrance() -> Tile {
        Tile {
            variant: Variant::Entrance,
        }
    }

    pub fn get_variant(&self) -> Variant {
        return self.variant
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
}
