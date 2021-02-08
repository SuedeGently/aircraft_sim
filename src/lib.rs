//! This file defines the necessary functions and structures for pyo3 to build a
//! Python library that can access compiled Rust. These are essentially wrappers
//! around the different modules written for this project, providing
//! functionality to the basic Python GUI.

mod aircraft;
mod config;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyTypeError};

use std::path::Path;
use std::thread;

use simple_logger::SimpleLogger;

use aircraft::Aircraft;
use aircraft::tile::Variant;
use config::*;

create_exception!(PyAircraft, CustomError, PyException);

/// Python-accesible structure used for interactive mode
///
/// This structure is accesible via Python and provides methods that call
/// through to Rust code; essentially this is a wrapper that allows for
/// predetermined interactions with the Rust backend.
///
/// # Examples
///
/// ```python
/// # Python code
/// import aircraft_sim
///
/// plane = aircraft_sim.PyAircraft()
/// ```
#[pyclass]
struct PyAircraft {
    aircraft: Option<Aircraft>,
    size: (u16, u16),
}

#[pymethods]
impl PyAircraft {
    /// Constructor
    #[new]
    fn new() -> Self {
        PyAircraft{
            size: (0,0),
            aircraft: None,
        }
    }
    
    /// Initialises the Rust logger
    ///
    /// Doesn't need to be called, however if it is not the rust logging will
    /// not be output. This can be modified to change the displayed log level.
    #[staticmethod]
    fn initialise_logger() -> PyResult<()> {
        SimpleLogger::new().init().expect("Failed to initialise logger");
        log::info!("Initialised logger");

        Ok(())
    }
    
    /// Initialises an interactive Aircraft object from filenames.
    ///
    /// Initialises an interactive Aircraft object using the csv file at
    /// `layout_path` and fills it with the passengers contained in the list at
    /// `passengers_path`.
    ///
    /// # Examples
    ///
    /// ```python
    /// # Python Code
    /// import aircraft_sim
    /// 
    /// try:
    ///     plane = aircraft_sim.PyAircraft()
    ///     plane.init_from_file("./layout.csv", "./passengers.csv")
    /// except:
    ///     print("File doesn't exist")
    /// ```
    fn init_from_file(&mut self, layout_path: &str, passengers_path: &str)
        -> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = read_layout(Path::new(layout_path));
            let passengers = read_passengers(Path::new(passengers_path));
            
            if new_aircraft.is_some() && passengers.is_some() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = self.aircraft.as_ref().unwrap().get_size();
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error2"))
            }
        } else {
            Err(PyTypeError::new_err("Error2"))
        }
    }

    /// Initialises an interactive Aircraft object with a back-first boarding
    /// pattern of passengers.
    ///
    /// Initialises an interactive Aircraft object of the given size and fills
    /// it with passengers in a back-first pattern.
    ///
    /// # Examples
    ///
    /// ```python
    /// # Python Code
    /// import aircraft_sim
    /// 
    /// try:
    ///     plane = aircraft_sim.PyAircraft()
    ///     plane.init_back_front(7,10)
    /// except:
    ///     print("Invalid size")
    /// ```
    fn init_random_back_front(&mut self,size_x:u16,size_y:u16) -> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = standard_layout(size_x, size_y);
            let passengers = random_back_first(size_x, size_y);

            if passengers.is_ok() && new_aircraft.is_ok() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = (size_x,size_y);
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error3"))
            }
        } else {
            Err(PyTypeError::new_err("Error3"))
        }
    }

    /// Initialises an interactive Aircraft object with a front-first boarding
    /// pattern of passengers.
    ///
    /// Initialises an interactive Aircraft object of the given size and fills
    /// it with passengers in a front-first pattern.
    ///
    /// # Examples
    ///
    /// ```python
    /// # Python Code
    /// import aircraft_sim
    /// 
    /// try:
    ///     plane = aircraft_sim.PyAircraft()
    ///     plane.init_front_back(7,10)
    /// except:
    ///     print("Invalid size")
    /// ```
    fn init_random_front_back(&mut self,size_x:u16,size_y:u16) -> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = standard_layout(size_x, size_y);
            let passengers = random_front_first(size_x, size_y);

            if passengers.is_ok() && new_aircraft.is_ok() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = (size_x,size_y);
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error3"))
            }
        } else {
            Err(PyTypeError::new_err("Error3"))
        }
    }

    /// Initialises an interactive Aircraft object with an aisle-first boarding
    /// pattern of passengers.
    ///
    /// Initialises an interactive Aircraft object of the given size and fills
    /// it with passengers in a aisle-first pattern.
    ///
    /// # Examples
    ///
    /// ```python
    /// # Python Code
    /// import aircraft_sim
    /// 
    /// try:
    ///     plane = aircraft_sim.PyAircraft()
    ///     plane.init_aisle_first(7,10)
    /// except:
    ///     print("Invalid size")
    /// ```
    fn init_random_aisle_first(&mut self,size_x:u16,size_y:u16)-> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = standard_layout(size_x, size_y);
            let passengers = random_aisle_first(size_x, size_y);

            if passengers.is_ok() && new_aircraft.is_ok() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = (size_x,size_y);
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error3"))
            }
        } else {
            Err(PyTypeError::new_err("Error3"))
        }
    }

    /// Initialises an interactive Aircraft object with a window-first boarding
    /// pattern of passengers.
    ///
    /// Initialises an interactive Aircraft object of the given size and fills
    /// it with passengers in a window-first pattern.
    ///
    /// # Examples
    ///
    /// ```python
    /// # Python Code
    /// import aircraft_sim
    /// 
    /// try:
    ///     plane = aircraft_sim.PyAircraft()
    ///     plane.init_window_first(7,10)
    /// except:
    ///     print("Invalid size")
    /// ```
    fn init_random_window_first(&mut self, size_x: u16, size_y: u16)
        -> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = standard_layout(size_x, size_y);
            let passengers = random_window_first(size_x, size_y);

            if passengers.is_ok() && new_aircraft.is_ok() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = (size_x,size_y);
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error3"))
            }
        } else {
            Err(PyTypeError::new_err("Error3"))
        }
    }

    /// Initialises an interactive Aircraft object with a pseudo-random boarding
    /// pattern of passengers.
    ///
    /// Initialises an interactive Aircraft object of the given size and fills
    /// it with passengers in a random pattern.
    ///
    /// # Examples
    ///
    /// ```python
    /// # Python Code
    /// import aircraft_sim
    /// 
    /// try:
    ///     plane = aircraft_sim.PyAircraft()
    ///     plane.init_random(7,10)
    /// except:
    ///     print("Invalid size")
    /// ```
    fn init_random(&mut self, size_x: u16, size_y: u16) -> PyResult<()> {
        if self.aircraft.is_none() {
            let new_aircraft = standard_layout(size_x, size_y);
            let passengers = random(size_x, size_y);

            if passengers.is_ok() && new_aircraft.is_ok() {
                let mut new_aircraft = new_aircraft.unwrap();
                for i in passengers.unwrap() {
                    new_aircraft.add_passenger(i);
                }
                self.aircraft = Some(new_aircraft);
                self.size = (size_x,size_y);
                Ok(())
            } else {
                Err(PyTypeError::new_err("Error3"))
            }
        } else {
            Err(PyTypeError::new_err("Error3"))
        }
    }
    
    /// Returns the aircraft's layout
    ///
    /// Returns the layout of the aircraft as a nested vector using integers
    /// instead of enums to provide Python compatibility.
    fn get_values(&mut self) -> PyResult<Vec<Vec<u8>>> {
        if self.aircraft.is_some() {
            let mut values = Vec::<Vec<u8>>::new();
            for y in 0..self.size.1 {
                let mut row = Vec::<u8>::new();
                for x in 0..self.size.0 {
                    row.push(match self.aircraft
                             .as_mut()
                             .unwrap()
                             .get_tile_variant(x,y) {
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
    
    /// Returns the positions of passengers on the given aircraft.
    ///
    /// Returns an integer representing the number of passengers occupying each
    /// space on an aircraft, using the same nested vector arrangement as
    /// `PyAircraft.get_values()`
    fn get_occupancy(&self) -> Vec<Vec<u8>> {
        let mut values = Vec::<Vec<u8>>::new();
        for y in 0..self.size.1 {
            let mut row = Vec::<u8>::new();
            for x in 0..self.size.0 {
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
    
    /// Iterates the aircraft once.
    ///
    /// Calls `Aircraft.update()` and returns a boolean representing whether the
    /// aircraft's passengers have all reached their seats yet.
    ///
    /// # Examples
    ///
    /// ```Python code
    /// import aircraft_sim
    ///
    /// plane = aircraft_sim.PyAircraft()
    /// plane.init_random(7,10)
    ///
    /// iterations = 0
    /// while not plane.update():
    ///     iterations += 1
    ///
    /// print("It took", iterations, "iterations to complete!")
    /// ```
    fn update(&mut self) -> PyResult<bool> {
        // TODO: FIX THIS
        self.aircraft.as_mut().unwrap().update();

        // TODO: FIX THIS
        Ok(self.aircraft.as_ref().unwrap().is_complete())
    }

    fn get_size_x(&self) -> PyResult<u16> {
        Ok(self.size.0)
    }

    fn get_size_y(&self) -> PyResult<u16> {
        Ok(self.size.1)
    }
}

/// Simulates a number of aircraft in parallel and returns how long each took to
/// complete.
///
/// Takes two vectors of filepaths, one for layout files and one for passenger
/// files, and simulates each pair. Every aircraft is simulated in parallel with
/// one another and the resulting times taken are returned as a vector in the
/// same order they were passed in.
#[pyfunction]
fn mass_sim(layouts:Vec<&str>,passenger_lists:Vec<&str>) -> PyResult<Vec<u16>> {
    // If a different number of files are passed in for each argument, the input
    // is invalid.
    if layouts.len() != passenger_lists.len() {
        return Err(PyTypeError::new_err("Invalid input lengths"));
    }

    let mut results = Vec::<u16>::new();

    // This vector holds the handles for each thread, allowing them to be joined
    // after completion.
    let mut jobs:Vec<thread::JoinHandle<Result<u16,&'static str>>> = Vec::new();

    for i in 0..layouts.len() {
        let aircraft = read_layout(Path::new(layouts.get(i).unwrap()));
        let passengers = read_passengers(Path::new(passenger_lists
                                                   .get(i)
                                                   .unwrap()));
        
        if aircraft.is_some() && passengers.is_some() {
            let mut aircraft = aircraft.unwrap();
            for i in passengers.unwrap() {
                aircraft.add_passenger(i);
            }

            // A new thread for each aircraft simulation is created.
            jobs.push(thread::spawn(move || {
                aircraft.run_to_completion()
            }));
        }
    }
    
    // All threads are joined here; errors are logged and a 0 value returned.
    for i in jobs {
        match i.join() {
            Ok(x) => {
                match x {
                    Ok(x) => results.push(x),
                    Err(e) => {
                        log::error!("Simulation failed: {}", e);
                        results.push(0);
                    },
                }
            },
            Err(e) => log::error!("Simulation failed: {:?}", e),
        }
    }
    
    return Ok(results);
}

#[pymodule]
fn aircraft_sim(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("CustomError", py.get_type::<CustomError>())?;
    m.add_class::<PyAircraft>()?;
    m.add_function(wrap_pyfunction!(mass_sim, m)?)?;

    Ok(())
}


fn core_mass_sim(layouts:Vec<&str>,passenger_lists:Vec<&str>) -> Result<Vec<u16>, &'static str> {
    // If a different number of files are passed in for each argument, the input
    // is invalid.
    if layouts.len() != passenger_lists.len() {
        return Err("Invalid input lengths");
    }

    let mut results = Vec::<u16>::new();

    // This vector holds the handles for each thread, allowing them to be joined
    // after completion.
    let mut jobs:Vec<thread::JoinHandle<Result<u16,&'static str>>> = Vec::new();

    for i in 0..layouts.len() {
        let aircraft = read_layout(Path::new(layouts.get(i).unwrap()));
        let passengers = read_passengers(Path::new(passenger_lists
                                                   .get(i)
                                                   .unwrap()));
        
        if aircraft.is_some() && passengers.is_some() {
            let mut aircraft = aircraft.unwrap();
            for i in passengers.unwrap() {
                aircraft.add_passenger(i);
            }

            // A new thread for each aircraft simulation is created.
            jobs.push(thread::spawn(move || {
                aircraft.run_to_completion()
            }));
        }
    }
    
    // All threads are joined here; errors are logged and a 0 value returned.
    for i in jobs {
        match i.join() {
            Ok(x) => {
                match x {
                    Ok(x) => results.push(x),
                    Err(e) => {
                        log::error!("Simulation failed: {}", e);
                        results.push(0);
                    },
                }
            },
            Err(e) => log::error!("Simulation failed: {:?}", e),
        }
    }
    
    return Ok(results);
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

    #[test]
    fn mass_simulation() {
        let mut passenger_lists = Vec::<&str>::new();
        passenger_lists.push("./config/steffen.csv");
        passenger_lists.push("./config/block1.csv");

        let mut layouts = Vec::<&str>::new();
        layouts.push("./config/standard_layout.csv");
        layouts.push("./config/standard_layout.csv");

        assert!(core_mass_sim(layouts, passenger_lists).is_ok());
    }
}
