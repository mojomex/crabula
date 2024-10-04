use crate::sensor::Sensor;

struct Backbone<T: Sensor> {
  sensor: T,
}

impl <T: Sensor> Backbone<T> {
    fn spin(&self) {
    }
}