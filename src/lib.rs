mod aircraft;
mod config;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyTypeError};

use std::path::Path;

use simple_logger::SimpleLogger;

use aircraft::Aircraft;
use aircraft::person::Person;
use aircraft::tile::Variant;
use config::{read_layout, read_passengers, random_back_first, standard_layout};

create_exception!(PyAircraft, CustomError, PyException);

#[pyfunction]
fn test() -> PyResult<String> {
    println!("Printing to console in Rust");
    Ok(format!("Here it is?"))
}

#[pyclass]
struct PyAircraft {
    aircraft: Option<Aircraft>,
    size: u16,
}

#[pymethods]
impl PyAircraft {
    #[new]
    fn new() -> Self {
        PyAircraft{
            size: 0,
            aircraft: None,
        }
    }

    #[staticmethod]
    fn initialise_logger() -> PyResult<()> {
        SimpleLogger::new().init().expect("Failed to initialise logger");
        log::info!("Initialised logging");

        Ok(())
    }

    fn init_from_file(&mut self, layout_path: &str, passengers_path: &str) -> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = read_layout(Path::new(layout_path));
            let passengers = read_passengers(Path::new(passengers_path));
            
            if new_aircraft.is_some() && passengers.is_some() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = self.aircraft.as_ref().unwrap().get_size().0;
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error2"))
            }
        } else {
            Err(PyTypeError::new_err("Error2"))
        }
    }

    fn init_random_back_front(&mut self, size_x: u16, size_y: u16) -> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = standard_layout(size_x, size_y);
            let passengers = random_back_first(size_x, size_y);

            if passengers.is_ok() && new_aircraft.is_ok() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = size_x;
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error3"))
            }
        } else {
            Err(PyTypeError::new_err("Error3"))
        }
    }
    
    fn get_values(&mut self) -> PyResult<Vec<Vec<u8>>> {
        if self.aircraft.is_some() {
            let mut values = Vec::<Vec<u8>>::new();
            for y in 0..self.size {
                let mut row = Vec::<u8>::new();
                for x in 0..self.size {
                    row.push(match self.aircraft.as_mut().unwrap().get_tile_variant(x,y) {
                        Variant::None => 0,
                        Variant::Aisle => 1,
                        Variant::Seat => 2,
                        Variant::Entrance => 3,
                    });
                }
                values.push(row);
            }
            return Ok(values);
        } else {
            return Err(PyTypeError::new_err("Error"));
        }
    }

    fn get_occupancy(&self) -> Vec<Vec<u8>> {
        let mut values = Vec::<Vec<u8>>::new();
        for y in 0..self.size {
            let mut row = Vec::<u8>::new();
            for x in 0..self.size {
                let mut occupancy: u8 = 0;
                // TODO: FIX
                if self.aircraft.as_ref().unwrap().check_if_occupied(x,y) {
                    occupancy += 1;
                }
                // TODO: FIX
                if self.aircraft.as_ref().unwrap().check_if_allowing(x,y) {
                    occupancy += 1;
                }
                row.push(occupancy);
            }
            values.push(row);
        }
        return values;
    }


    fn update(&mut self) -> PyResult<bool> {
        // TODO: FIX THIS
        self.aircraft.as_mut().unwrap().update();

        // TODO: FIX THIS
        Ok(self.aircraft.as_ref().unwrap().is_complete())
    }

    fn get_size(&self) -> PyResult<u16> {
        Ok(self.size)
    }

    fn test(&mut self, x: u16, y: u16) -> PyResult<()> {
        let mut dave = Person::new("Dave");
        dave.target_seat(x,y);

        // TODO: FIX THIS
        self.aircraft.as_mut().unwrap().add_passenger(dave);

        Ok(())
    }
}

#[pymodule]
fn aircraft_sim(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("CustomError", py.get_type::<CustomError>())?;
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
