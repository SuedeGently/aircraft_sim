pub struct Person {
    name: String,
    seat: Option<(u16, u16)>,
}

impl Person {
    pub fn new(n: &str) -> Person {
        Person {
            name: n.to_string(),
            seat: None,
        }
    }

    fn target_seat(&mut self, x: u16, y: u16) {
        self.seat = Some((x, y));
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_seat(&self) -> Option<(u16, u16)> {
        self.seat
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let person = Person::new("Dave");
        assert_eq!(person.name, "Dave");
        assert_eq!(person.seat, None);
    }

    fn target_seat() {
        let mut person = Person::new("Dave");
        person.target_seat(6, 9);
        
        assert_eq!(person.seat.unwrap(), (6, 9));
    }
}
