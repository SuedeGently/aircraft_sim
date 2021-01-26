mod aircraft;
mod config;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::path::Path;

use simple_logger::SimpleLogger;

use aircraft::Aircraft;
use aircraft::person::Person;
use aircraft::tile::Variant;
use config::{read_layout, read_passengers};

#[pyfunction]
fn test() -> PyResult<String> {
    println!("Printing to console in Rust");
    Ok(format!("Here it is?"))
}

#[pyclass]
struct PyAircraft {
    aircraft: Aircraft,
    size: u16,
}

#[pymethods]
impl PyAircraft {
    #[new]
    fn new(filepath: &str) -> Self {
        let paircraft = config::read_layout(Path::new(filepath)).unwrap();

        PyAircraft{
            size: paircraft.get_size().0,
            aircraft: paircraft,
        }
    }
    
    fn get_values(&self) -> Vec<Vec<u8>> {
        let mut values = Vec::<Vec<u8>>::new();
        for y in 0..self.size {
            let mut row = Vec::<u8>::new();
            for x in 0..self.size {
                row.push(match self.aircraft.get_tile_variant(x,y) {
                    Variant::None => 0,
                    Variant::Aisle => 1,
                    Variant::Seat => 2,
                    Variant::Entrance => 3,
                });
            }
            values.push(row);
        }
        return values;
    }

    fn get_occupancy(&self) -> Vec<Vec<bool>> {
        let mut values = Vec::<Vec<bool>>::new();
        for y in 0..self.size {
            let mut row = Vec::<bool>::new();
            for x in 0..self.size {
                row.push(self.aircraft.check_if_occupied(x,y));
            }
            values.push(row);
        }
        return values;
    }


    fn update(&mut self) -> PyResult<bool> {
        self.aircraft.update();

        Ok(self.aircraft.is_complete())
    }

    fn get_size(&self) -> PyResult<u16> {
        Ok(self.size)
    }

    fn test(&mut self, x: u16, y: u16) -> PyResult<()> {
        let mut dave = Person::new("Dave");
        dave.target_seat(x,y);

        self.aircraft.add_passenger(dave);

        Ok(())
    }
}

#[pymodule]
fn aircraft_sim(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyAircraft>()?;

    Ok(())
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
        let mut aircraft = read_layout(Path::new("./config/test_layout.csv"))
            .expect("Failed to read layout");
        let passengers =
            read_passengers(Path::new("./config/test_passengers.csv"))
            .expect("Failed to read passenger list");

        for i in passengers {
            println!("Adding passenger: {}", i[0]);
            let mut passenger = Person::new(&i[0]);
            passenger.target_seat(i[1].parse().expect("Invalid coord"),
                                  i[2].parse().expect("Invalid coord"));
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
