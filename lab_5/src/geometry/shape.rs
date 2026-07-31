pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub struct Polygon {
    pub vertices: Vec<Point>,
}

impl Polygon {
    pub fn perimeter(&self) -> f64 {
        if self.vertices.len() < 2 {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 0..self.vertices.len() {
            let current = &self.vertices[i];
            let next = &self.vertices[(i + 1) % self.vertices.len()];
            total += current.distance(next);
        }
        total
    }

    pub fn is_closed(&self) -> bool {
        self.vertices.len() >= 3
    }
}

pub fn run() {
    let square = Polygon {
        vertices: vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ],
    };

    let perimeter = square.perimeter();
    let is_closed = square.is_closed();
    let origin = Point::new(0.0, 0.0);
    let corner = Point::new(1.0, 1.0);
    let distance = origin.distance(&corner);

    println!("square perimeter = {:.2}", perimeter);
    println!("square closed = {}", is_closed);
    println!("distance from origin to corner = {:.2}", distance);
}
