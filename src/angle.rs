/// This type represents any angle value
pub type Angle = f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AngleUnit {
    Degrees,
    Radians,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction {
    angle: Angle,
    unit: AngleUnit,
}

impl Direction {
    pub fn zero_degrees() -> Direction {
        Direction {
            angle: Default::default(),
            unit: AngleUnit::Degrees,
        }
    }

    pub fn into_radians(self) -> Direction {
        Direction {
            angle: self.to_radians(),
            unit: AngleUnit::Radians,
        }
    }

    pub fn into_degrees(self) -> Direction {
        Direction {
            angle: self.to_degrees(),
            unit: AngleUnit::Degrees,
        }
    }

    pub fn to_radians(&self) -> Angle {
        match self.unit {
            AngleUnit::Degrees => self.angle.to_radians(),
            AngleUnit::Radians => self.angle,
        }
    }

    pub fn to_degrees(&self) -> Angle {
        match self.unit {
            AngleUnit::Degrees => self.angle,
            AngleUnit::Radians => self.angle.to_degrees(),
        }
    }

    /// The raw angle represented by this direction with no conversion whatsoever
    pub fn raw_angle(&self) -> Angle {
        self.angle
    }

    pub fn rotate_clockwise(&mut self, angle: Angle) {
        self.angle += angle;
    }

    pub fn rotate_counterclockwise(&mut self, angle: Angle) {
        self.angle -= angle;
    }
}
