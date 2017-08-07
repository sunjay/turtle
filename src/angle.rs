/// This type represents any angle value
pub type Angle = f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnit {
    Degrees,
    Radians,
}

impl AngleUnit {
    fn to_radians(&self, value: Angle) -> Angle {
        use self::AngleUnit::*;

        match *self {
            Degrees => value.to_radians(),
            Radians => value,
        }
    }

    fn to_degrees(&self, value: Angle) -> Angle {
        use self::AngleUnit::*;

        match *self {
            Degrees => value,
            Radians => value.to_degrees(),
        }
    }
}
