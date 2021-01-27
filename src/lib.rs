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
    fn new(layout: &str, passengers: &str) -> Self {
        let paircraft = config::read_layout(Path::new(layout)).unwrap();
        let ppassengers = config::read_passengers(Path::new(passengers)).unwrap();

        let mut aircraft = PyAircraft{
            size: paircraft.get_size().0,
            aircraft: paircraft,
        };

        for i in ppassengers {
            aircraft.aircraft.add_passenger(i);
        }

        return aircraft;
    }

    #[staticmethod]
    fn initialise_logger() -> PyResult<()> {
        SimpleLogger::new().init().expect("Failed to initialise logger");
        log::info!("Initialised logging");

        Ok(())
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

    fn get_occupancy(&self) -> Vec<Vec<u8>> {
        let mut values = Vec::<Vec<u8>>::new();
        for y in 0..self.size {
            let mut row = Vec::<u8>::new();
            for x in 0..self.size {
                let mut occupancy: u8 = 0;
                if self.aircraft.check_if_occupied(x,y) {
                    occupancy += 1;
                }
                if self.aircraft.check_if_allowing(x,y) {
                    occupancy += 1;
                }
                row.push(occupancy);
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
            println!("Adding passenger: {}", i.get_name());
            aircraft.add_passenger(i);
        }

        for _ in 0..30 {
            aircraft.update();
            aircraft.ascii_render();
        }

        assert!(aircraft.check_if_occupied(0,0));
        assert!(aircraft.check_if_occupied(4,4));
    }
}
