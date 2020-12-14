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

    fn ascii_render(&self) {
        print!("   ");
        for i in 0..self.size.0 {
            print!("{}", i);
        }
        print!("\n");
        for j in 0..self.size.1 as usize {
            print!("{:>3}", j);
            for i in 0..self.size.0 as usize {
                if self.layout[i][j].is_occupied() {
                    print!("@");
                } else {
                    print!("{}", match self.layout[i][j].get_variant() {
                        Variant::Aisle => "*",
                        Variant::Seat => "#",
                        Variant::Entrance => "*",
                        Variant::None => "?",
                    });
                }
            }
            print!("\n");
        }
    }

    fn update(&mut self) {
        for i in 0..self.size.0 as usize {
            for j in 0..self.size.1 as usize{
                if 
                    ! self.layout[i][j].has_updated() && (
                    self.layout[i][j].get_variant() == Variant::Entrance ||
                    self.layout[i][j].get_variant() == Variant::Aisle ||
                    self.layout[i][j].get_variant() == Variant::Seat ) {
                        if self.layout[i][j].get_occupier().is_some() {
                            let mut surroundings = [SimpleTile::empty(); 9];
                            let mut pos: usize = 0;
                            for l in (j as i32 - 1)..(j as i32 + 2) {
                                for k in (i as i32 - 1)..(i as i32 + 2) {
                                    if k >= 0 && k < self.size.0 as i32 &&
                                       l >= 0 && l < self.size.1 as i32 {
                                        surroundings[pos] = SimpleTile::new(&self.layout[k as usize][l as usize]);
                                        println!("Tile {},{} = {:?}", k, l, surroundings[pos].get_variant());
                                    }
                                    pos += 1;
                                }
                            }
                            let behaviour = self.layout[i][j].get_occupier().unwrap().update((i as u16,j as u16), surroundings);
                            if behaviour != Behaviour::Wait {
                                println!("Passenger moved: {:?}", behaviour);
                                let person = self.layout[i][j].free();
                                match behaviour {
                                    Behaviour::Wait => println!("Wait"),
                                    Behaviour::Move_North => self.layout[i][j - 1].occupy(person.unwrap()),
                                    Behaviour::Move_South => self.layout[i][j + 1].occupy(person.unwrap()),
                                    Behaviour::Move_East => self.layout[i + 1][j].occupy(person.unwrap()),
                                    Behaviour::Move_West => self.layout[i - 1][j].occupy(person.unwrap()),
                                    _ => panic!("Impossible movement"),
                                }
                            } else {
                                println!("Passenger waited");
                            }
                        }
                        if self.layout[i][j].get_variant() == Variant::Entrance && self.passengers.len() > 0 {
                            self.layout[i][j]
                                .occupy(self.passengers.pop().unwrap());
                        }
                }
            }
        }
        self.reset();
        println!("END_OF_UPDATE_____________________________");
    }

    pub fn reset(&mut self) {
        for i in 0..self.size.0 as usize {
            for j in 0..self.size.1 as usize {
                self.layout[i][j].set_updated(false);
            }
        }
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.size.0, self.size.1)
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

        println!("Adding passenger");
        aircraft.add_passenger(passenger);
        assert_eq!(aircraft.passengers.len(), 1, "Unwanted passenger at initialisation");

        println!("Adding entrance and updating");
        aircraft.layout[2][2] = Tile::entrance();
        // aircraft.layout[2][3] = Tile::aisle();
        aircraft.update();
        assert_eq!(aircraft.passengers.len(), 0, "Passenger was not removed from passengers array");
        assert!(aircraft.layout[2][2].is_occupied(), "Passenger was not added to entrance tile");

        println!("Updating");
        aircraft.update();
        println!("Updating");
        aircraft.update();
        assert_eq!(aircraft.layout[2][2].is_occupied(), false, "Passenger did not move from entrance");
        assert!(aircraft.layout[1][1].is_occupied(), "Passenger did not make it to target seat");

        println!("Updating once too much");
        aircraft.update();
        assert!(aircraft.layout[1][1].is_occupied(), "Passenger shouldn't have moved from their seat");
    }

    #[test]
    fn impassable_terrain() {
        let mut aircraft = Aircraft::new(3,3);
        let mut passenger = Person::new("Dave");
        passenger.target_seat(2,2);
        aircraft.layout[0][0] = Tile::entrance();
        for coords in &[(0, 1), (1, 1), (1, 0)] {
            aircraft.layout[coords.0][coords.1] = Tile::none();
        }
        aircraft.add_passenger(passenger);

        for i in 0..100 {
            aircraft.update();
        }

        assert_eq!(aircraft.layout[2][2].is_occupied(), false, "Passenger made it to seat despite obstacles");
    }

    #[test]
    fn aisle_ignoring() {
        let mut aircraft = Aircraft::new(3,3);
        let mut passenger = Person::new("Dave");
        passenger.target_seat(2, 0);
        aircraft.layout[1][2] = Tile::entrance();
        for i in 0..3 {
            aircraft.layout[0][i] = Tile::seat();
            aircraft.layout[2][i] = Tile::seat();
        }
        aircraft.add_passenger(passenger);

        for i in 0..5 {
            println!("Updating..");
            aircraft.update();
        }
        assert!(aircraft.layout[2][0].is_occupied(), "Passenger did not make it to seat");
    }

    #[test]
    fn advanced_aisle_ignoring() {
        let mut aircraft = Aircraft::new(3,3);
        aircraft.layout[1][2] = Tile::entrance();
        for i in 0..3 {
            for j in &[0, 2] {
                let mut passenger = Person::new("DEFAULT");
                passenger.target_seat(*j, i);
                aircraft.layout[*j as usize][i as usize] = Tile::seat();
                aircraft.add_passenger(passenger);
            }
        }
        // let mut passenger = Person::new("DAVE");
        // passenger.target_seat(2, 0);
        // aircraft.add_passenger(passenger);
        
        for _ in 0..100 {
            aircraft.ascii_render();
            println!("========================");
            aircraft.update();
        }

        for i in 0..3 {
            for j in &[0, 2] {
                assert!(aircraft.layout[*j][i].is_occupied(), "Seat {},{} was not occupied", *j, i);
            }
        }
    }

    #[test]
    fn aisle_shifting() {
        let mut aircraft = Aircraft::new(5,5);

        aircraft.layout[2][4] = Tile::entrance();

        let mut passenger = Person::new("DEFAULT");
        passenger.target_seat(0,0);
        aircraft.add_passenger(passenger);

        let mut passenger = Person::new("DEFAULT");
        passenger.target_seat(1,0);
        aircraft.add_passenger(passenger);

        for _ in 0..5 {
            aircraft.ascii_render();
            println!("========================");
            aircraft.update();
        }

        assert!(aircraft.layout[0][0].is_occupied());
        assert!(aircraft.layout[1][0].is_occupied());
    }
}
