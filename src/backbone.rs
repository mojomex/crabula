use crate::sensor::Sensor;

struct Backbone {
  sensor: dyn Sensor,
}

impl Backbone {
    fn spin(&self) {
    }
}