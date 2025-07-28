use rand::random;

//     pub trait Drawable {
// }

// pub trait Displayable {}

#[derive(Debug, Clone)]
pub struct Point(i32, i32);
pub struct Line(pub Point,pub Point);
pub struct Triangle(pub Point,pub Point,pub Point);
pub struct Rectangle(pub Point,pub Point);
pub struct Circle(pub Point, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }

    pub fn random() -> (i32,i32) {
        let a = random::<i32>() % 4096;
        let b = random::<i32>() % 4096;

        (a, b)
    }
}

impl Triangle {
    pub fn new(x: Point , y:Point, z : Point) -> Self {
        Triangle(x,y,z)
    }
}

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        Line(x,y)
    }
}

impl Rectangle {
    pub fn new(x : Point , y : Point) -> Self {
        Rectangle(x, y)
    }
    
}

impl Circle {
    pub fn new(c : Point , r : i32) -> Self {
        Circle(c, r)
    }
}