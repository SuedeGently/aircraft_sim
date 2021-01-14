use std::fs::File;
use std::path::Path;

use super::aircraft::Aircraft;
use super::aircraft::tile::{Tile,Variant};

struct seat_data {
    seat_x: u16,
    seat_y: u16,
    variant: Variant,
}

impl seat_data {
    fn new(x: u16, y: u16, variant: &str) -> seat_data {
        seat_data {
            seat_x: x,
            seat_y: y,
            variant: str_to_var(variant),
        }
    }

    fn get_x(&self) -> u16 {
        self.seat_x
    }

    fn get_y(&self) -> u16 {
        self.seat_y
    }

    fn get_variant(&self) -> Variant {
        self.variant
    }
}

fn str_to_var(var: &str) -> Variant {
    match var {
        "aisle" => Variant::Aisle,
        "seat" => Variant::Seat,
        "entrance" => Variant::Entrance,
        _ => Variant::None, // Shouldn't happen
    }
}

pub fn read_passengers(path: &Path) -> Option<Vec<[String; 3]>> {
    let mut persons = Vec::<[String; 3]>::new();
    let file = File::open(path).expect("Invalid file path");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.expect("Result is invalid record");
        let data: [String; 3] = [
            record[0].to_string(),
            record[1].to_string(),
            record[2].to_string(),
        ];
        persons.push(data);
    }
    return Some(persons);
}

pub fn read_layout(path: &Path) -> Option<Aircraft> {
    let mut seats = Vec::<seat_data>::new();
    let file = File::open(path).expect("Invalid file path");
    let mut rdr = csv::Reader::from_reader(file);
    
    for result in rdr.records() {
        let record = result.expect("Result is invalid record");
        let data = seat_data::new(
            record[0].parse().expect("Couldn't parse"),
            record[1].parse().expect("Couldn't parse"),
            &record[2],
        );
        seats.push(data);
    }
    
    let (mut size_x, mut size_y) = (0, 0);
    for i in &seats {
        if i.get_x() > size_x { size_x = i.get_x() }
        if i.get_y() > size_y { size_y = i.get_y() }
    }
    size_x = size_x + 1;
    size_y = size_y + 1;

    let mut aircraft = Aircraft::new(size_x, size_y);
    for i in seats {
        aircraft.set_tile(i.get_x(), i.get_y(), i.get_variant());
    }

    return Some(aircraft);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_passengers() {
        let persons = read_passengers(Path::new("./config/test_passengers.csv")).unwrap();

        assert_eq!(persons.len(), 2, "Incorrect number of records");

        let person = persons.get(0).expect("First record not found");

        assert_eq!(person[0], "person0", "First record's name was wrong");
        assert_eq!(person[1], "0", "First record's x coord was wrong");
    }

    #[test]
    fn test_read_layout() {
        let aircraft = read_layout(Path::new("./config/test_layout.csv")).unwrap();

        assert_eq!((5,5), aircraft.get_size());
    }

    #[test]
    fn test_str_to_var() {
        assert_eq!(str_to_var("aisle"), Variant::Aisle);
        assert_eq!(str_to_var("seat"), Variant::Seat);
        assert_eq!(str_to_var("entrance"), Variant::Entrance);
        assert_eq!(str_to_var("none"), Variant::None);
        assert_eq!(str_to_var("invalid"), Variant::None);
    }
}
