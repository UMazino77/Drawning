use rand::random;
use raster::Color;
use raster::Image;

pub trait Drawable {
        fn draw(im : &mut Image) ;

        fn color()->Color ;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

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

    pub fn random(width : i32, height:i32) -> Self {
        let a = random::<i32>() % width;
        let b = random::<i32>() % height;

        Point::new(a, b)
    }
}

impl color for Point {
        fn color()->Color {
        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        Color::rgb(r,g,b)
    }
}

impl Triangle {
    pub fn new(x: Point , y:Point, z : Point) -> Self {
        Triangle(x,y,z)
    }
}

impl color for Triangle {
        fn color()->Color {
        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        Color::rgb(r,g,b)
    }
}

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        Line(x,y)
    }

    pub fn random(width : i32, height: i32) ->Self{
        let a = random::<i32>() % width;
        let b = random::<i32>() % height;

        Line::new(a, b)
    }
}

impl color for Line {
        fn color()->Color {
        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        Color::rgb(r,g,b)
    }
}

impl Rectangle {
    pub fn new(x : Point , y : Point) -> Self {
        Rectangle(x, y)
    }
    
}

impl color for Rectangle {
        fn color()->Color {
        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        Color::rgb(r,g,b)
    }
}

impl Circle {
    pub fn new(c : Point , r : i32) -> Self {
        Circle(c, r)
    }

    pub fn random(width:i32, height : i32) -> Self{
        let a = random::<i32>() % width;
        let b = random::<i32>() % height;
        let c = random::<i32>() % (min(height,width)/2);


        Circle::new(Point::new(a, b),c) 
    }
}

impl color for Circle {
        fn color()->Color {
        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        Color::rgb(r,g,b)
    }
}

pub fn min(a : i32, b: i32)->i32 {
    if a<b {
        return a;
    }
    b
}