mod shapes;

use std::{fmt::Display, fs, str::FromStr};

use anyhow::{Ok, Result};

use crate::shapes::{
    circle::Circle,
    collisions::{Collidable, Contains, Points},
    rect::Rect,
};

enum Shape {
    Circle(Circle),
    Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));

        match shape {
            "rect" => return Ok(Shape::Rect(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => Err(anyhow::anyhow!("Bad shape!")),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Circle(c) => return c.points(),
            Shape::Rect(r) => return r.points(),
        }
    }
}

impl Contains for Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => return c.contains_point(point),
            Shape::Rect(r) => return r.contains_point(point),
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => return write!(f, "{}", c),
            Shape::Rect(r) => return write!(f, "{}", r),
        }
    }
}

fn main() -> Result<()> {
    let file_content = fs::read_to_string("shapes.txt").expect("Failed to read the file content");

    let shapes = file_content
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<Shape>>();

    shapes
        .iter()
        .skip(1)
        .zip(shapes.iter().take(shapes.len() - 1))
        .filter(|(a, b)| a.collide(b) || b.collide(a))
        .for_each(|(a, b)| println!("{} collides with {}", a, b));

    return Ok(());
}
