mod aircraft;

use simple_logger::SimpleLogger;

use aircraft::Aircraft;

fn test() {
    println!("Logging(?)");
    log::warn!("Hmm");
}

fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Initialised logger");

    let test = Aircraft::new(5,5);
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
