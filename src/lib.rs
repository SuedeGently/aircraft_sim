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
    fn new(size: u16) -> Self {
        PyAircraft{
            aircraft: Aircraft::new(size, size),
            size: size,
        }
    }
    
    fn get_values(&self) -> Vec<Vec<u8>> {
        let mut values = Vec::<Vec<u8>>::new();
        for i in 0..self.size {
            let mut row = Vec::<u8>::new();
            for j in 0..self.size {
                row.push(match self.aircraft.get_tile_variant(i,j) {
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

    fn test(&mut self) -> PyResult<()> {
        for i in 0..self.size {
            self.aircraft.set_tile(i,i, Variant::Seat);
        }

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
