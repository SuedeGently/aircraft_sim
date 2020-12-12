mod aircraft;
mod config;

use std::path::Path;

use aircraft::Aircraft;

fn main() {
    let passenger_list = config::read_passengers(Path::new("./config/test.csv"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let aircraft = Aircraft::new(5, 5);
        assert_eq!(aircraft.get_size(), (5, 5));
    }
}
