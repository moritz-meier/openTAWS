use super::*;

#[derive(Debug)]
pub struct Mode2();

impl AlertSystem for Mode2 {
    fn is_armed(&self) -> bool {
        false
    }

    fn is_inhibited(&self) -> bool {
        unimplemented!()
    }

    fn inhibit(&mut self) {
        unimplemented!()
    }

    fn uninhibit(&mut self) {
        unimplemented!()
    }

    fn process(&mut self, _state: &AircraftState) -> Option<(Alert, AlertLevel)> {
        unimplemented!()
    }
}