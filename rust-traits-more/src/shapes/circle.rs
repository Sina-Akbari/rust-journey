use super::{
    area::Area,
    collisions::{Contains, Points},
};
use std::{f64::consts::PI, fmt::Display, str::FromStr};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        (dx * dx + dy * dy) <= self.radius * self.radius
    }
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![(self.x, self.y)].into();
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            x: 0.0,
            y: 0.0,
            radius: 10.0,
        }
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle ({} ,{}), Radius {}", self.x, self.y, self.radius)
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();

        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Bad circle from str"));
        }

        Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        })
    }
}
