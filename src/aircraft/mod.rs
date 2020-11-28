mod tile;
mod person;

use tile::{Tile, Variant, SimpleTile};
use person::{Person, Behaviour};

pub struct Aircraft {
    size: (u16, u16),
    layout: Vec<Vec<Tile>>,
    passengers: Vec<Person>,
}

impl Aircraft {
    pub fn new(x: u16, y: u16) -> Aircraft {
        let mut aircraft = Aircraft {
            size: (x, y),
            layout: Vec::<Vec<Tile>>::new(),
            passengers: Vec::<Person>::new(),
        };
        aircraft.clear();
        return aircraft;
    }

    fn clear(&mut self) {
        self.layout = Vec::<Vec<Tile>>::new();
        for i in 0..self.size.0 {
            self.layout.push(Vec::<Tile>::new());
            for _ in 0..self.size.1 {
                self.layout[i as usize].push(Tile::aisle());
            }
        }
    }

    fn add_passenger(&mut self, p: Person) {
        self.passengers.push(p);
    }

    fn update(&mut self) {
        for i in 0..self.size.0 as usize {
            for j in 0..self.size.1 as usize{
                if self.layout[i][j]
                    .get_variant() == Variant::Entrance {
                        if self.layout[i][j].get_occupier().is_some() {
                            let mut surroundings = [SimpleTile::empty(); 9];
                            let mut pos: usize = 0;
                            for k in (i as i32 - 1)..(i as i32 + 1) {
                                for l in (j as i32 - 1)..(j as i32 + 1) {
                                    if k >= 0 && k < self.size.0 as i32 &&
                                       l >= 0 && l < self.size.1 as i32 {
                                        surroundings[pos] = SimpleTile::new(&self.layout[i][j]);
                                    }
                                    pos += 1;
                                }
                            }
                            let behaviour = self.layout[i][j].get_occupier().unwrap().update((i as u16,j as u16), surroundings);
                            if behaviour != Behaviour::Wait {
                                let person = self.layout[i][j].free();
                                match behaviour {
                                    Behaviour::Wait => println!("Wait"),
                                    Behaviour::Move_North => self.layout[i][j - 1].occupy(person.unwrap()),
                                    Behaviour::Move_South => self.layout[i][j + 1].occupy(person.unwrap()),
                                    Behaviour::Move_East => self.layout[i + 1][j].occupy(person.unwrap()),
                                    Behaviour::Move_West => self.layout[i - 1][j].occupy(person.unwrap()),
                                    _ => panic!("fuck"),
                                }
                            }
                        }
                        if self.passengers.len() > 0 {
                            self.layout[i][j]
                                .occupy(self.passengers.pop().unwrap());
                        }
                }
            }
        }
    }

    pub fn get_size(&self) -> (u16, u16) {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear() {
        let mut aircraft = Aircraft {
            size: (5, 5),
            layout: Vec::<Vec<Tile>>::new(),
            passengers: Vec::<Person>::new(),
        };

        aircraft.clear();
        for i in 0..aircraft.size.0 {
            for j in 0..aircraft.size.1 {
                assert_eq!(aircraft.layout[i as usize][j as usize].get_variant(), Variant::Aisle);
            }
        }
    }

    #[test]
    fn constructor() {
        let aircraft = Aircraft::new(10, 10);
        assert_eq!(aircraft.layout.len(), 10);
        assert_eq!(aircraft.size, (10,10));
        for i in 0..aircraft.size.0 {
            for j in 0..aircraft.size.1 {
                println!("Testing aircraft[{}][{}]", i, j);
                assert_eq!(aircraft.layout[i as usize][j as usize].get_variant(), Variant::Aisle);
            }
        }
    }

    #[test]
    fn add_passenger() {
        println!("Testing add_passenger()");
        let mut aircraft = Aircraft::new(6, 9);
        let passenger = Person::new("Dave");
        aircraft.add_passenger(passenger);
        assert_eq!(aircraft.passengers.get(0).unwrap().get_name(), "Dave");
        assert_eq!(aircraft.passengers.get(0).unwrap().get_seat(), None);
    }

    #[test]
    fn get_size() {
        let aircraft = Aircraft::new(5, 6);
        assert_eq!(aircraft.get_size(), (5, 6));
    }

    #[test]
    fn update() {
        let mut aircraft = Aircraft::new(10, 10);
        let mut passenger = Person::new("Dave");
        passenger.target_seat(1,1);

        aircraft.add_passenger(passenger);
        assert_eq!(aircraft.passengers.len(), 1, "Unwanted passenger at initialisation");

        aircraft.layout[0][0] = Tile::entrance();
        aircraft.update();
        assert_eq!(aircraft.passengers.len(), 0, "Passenger was not removed from passengers array");
        assert!(aircraft.layout[0][0].is_occupied(), "Passenger was not added to entrance tile");

        aircraft.update();
        aircraft.update();
        assert_eq!(aircraft.layout[0][0].is_occupied(), false, "Passenger did not move from entrance");
    }
}
