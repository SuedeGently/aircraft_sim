//! This file provides a command-line frontend when the crate is run as a binary

mod config;
mod aircraft;

use clap::{App, Arg};
use std::path::Path;

use config::{read_layout, read_passengers};

fn main() {
    let matches = App::new("aircraft_sim")
                    .arg(Arg::with_name("layout")
                           .index(1)
                           .required(true)
                           .help("Layout file as csv with headers"))
                    .arg(Arg::with_name("passengers")
                           .index(2)
                           .required(true)
                           .help("Passenger list as csv with headers"))
                    .get_matches();

    let layout_file = matches.value_of("layout").unwrap();
    let passenger_list = matches.value_of("passengers").unwrap();
    
    let aircraft = read_layout(Path::new(layout_file));
    let passengers = read_passengers(Path::new(passenger_list));
    
    if aircraft.is_some() && passengers.is_some() {
        let mut aircraft = aircraft.unwrap();
        for i in passengers.unwrap() {
            aircraft.add_passenger(i);
        }
        match aircraft.run_to_completion() {
            Ok(x) => println!("Completed in {} steps!", x),
            _ => println!("Incorrect input files"),
        }
    } else {
        println!("Invalid files given as input; exiting");
    }
}
