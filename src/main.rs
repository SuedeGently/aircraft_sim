mod aircraft;
mod config;

use std::path::Path;

use simple_logger::SimpleLogger;

use aircraft::Aircraft;
use aircraft::person::Person;
use config::{read_layout, read_passengers};

fn test() {
    println!("Logging(?)");
    log::warn!("Hmm");
}

fn check_if_complete(aircraft: Aircraft, passenger_list: Vec<Person>) -> bool {
    // TODO: Implement
    panic!("Not implemented");
}

fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Initialised logger");

    let mut aircraft = read_layout(Path::new("./config/test_layout.csv")).expect("Failed to read layout");
    let passengers = read_passengers(Path::new("./config/test_passengers.csv")).expect("Failed to read passenger list");

    for i in passengers {
        println!("Adding passenger: {}", i[0]);
        let mut passenger = Person::new(&i[0]);
        passenger.target_seat(i[1].parse().expect("Invalid coord"), i[2].parse().expect("Invalid coord"));
        aircraft.add_passenger(passenger);
    }

    for _ in 0..15 {
        aircraft.update();
        aircraft.ascii_render();
    }

    assert!(aircraft.check_if_occupied(0,0));
    assert!(aircraft.check_if_occupied(4,4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let aircraft = Aircraft::new(5, 5);
        assert_eq!(aircraft.get_size(), (5, 5));
    }

    #[test]
    fn test_from_file() {
        let mut aircraft = read_layout(Path::new("./config/test_layout.csv")).expect("Failed to read layout");
        let passengers = read_passengers(Path::new("./config/test_passengers.csv")).expect("Failed to read passenger list");

        for i in passengers {
            println!("Adding passenger: {}", i[0]);
            let mut passenger = Person::new(&i[0]);
            passenger.target_seat(i[1].parse().expect("Invalid coord"), i[2].parse().expect("Invalid coord"));
            aircraft.add_passenger(passenger);
        }

        for _ in 0..15 {
            aircraft.update();
            aircraft.ascii_render();
        }

        assert!(aircraft.check_if_occupied(0,0));
        assert!(aircraft.check_if_occupied(4,4));
    }
}
