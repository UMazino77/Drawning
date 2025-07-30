use rand::random;
use raster::Color;

pub trait Drawable {
    fn draw(&self, im: &mut dyn Displayable);
    
    fn color() -> Color {
        let r = random::<u8>();
        let g = random::<u8>();
        let b = random::<u8>();

        Color::rgb(r, g, b)
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug, Clone, Copy)]
pub struct Point(i32, i32);
pub struct Line(pub Point, pub Point);
pub struct Triangle(pub Point, pub Point, pub Point);
pub struct Rectangle(pub Point, pub Point);
pub struct Circle(pub Point, i32);
// pub struct Cube(pubPoint, i32);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let a = random::<i32>() % width;
        let b = random::<i32>() % height;

        Point::new(a, b)
    }
}

impl Drawable for Point {
    fn draw(&self, im: &mut dyn Displayable) {
        im.display(self.0, self.1, Point::color());
    }
}

impl Triangle {
    pub fn new(x: &Point, y: &Point, z: &Point) -> Self {
        Triangle(*x, *y, *z)
    }
}

impl Drawable for Triangle {
    fn draw(&self, im: &mut dyn Displayable) {
        let l1 = Line::new(self.0, self.1);
        let l2 = Line::new(self.1, self.2);
        let l3 = Line::new(self.0, self.2);

        let col = Triangle::color();

        helper(&l1, im, col.clone());
        helper(&l2, im, col.clone());
        helper(&l3, im, col.clone());
    }
}

impl Line {
    pub fn new(x: Point, y: Point) -> Self {
        Line(x, y)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let a = random::<i32>().rem_euclid(width);
        let b = random::<i32>().rem_euclid(height);

        let p1 = Point::new(a, b);

        let c = random::<i32>().rem_euclid(width);
        let d = random::<i32>().rem_euclid(height);

        let p2 = Point::new(c, d);

        Line::new(p1, p2)
    }
}

impl Drawable for Line {
    fn draw(&self, im: &mut dyn Displayable) {
        let col = Line::color();
        helper(&self, im, col);
    }
}

impl Rectangle {
    pub fn new(x: &Point, y: &Point) -> Self {
        Rectangle(*x, *y)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, im: &mut dyn Displayable) {
        let col = Rectangle::color();
        let p3 : Point = Point::new(self.0.0 , self.1.1);
        let p4 : Point = Point::new(self.1.0 , self.0.1);
        
        let l1 = Line::new(self.0, p4);
        let l2 = Line::new(p4, self.1);
        let l3 = Line::new(self.1, p3);
        let l4 = Line::new(p3, self.0);
        // let l5 = Line::new(self.0, self.1);
        // let l6 = Line::new(p3, p4);
        
        
        helper(&l1, im, col.clone());
        helper(&l2, im, col.clone());
        helper(&l3, im, col.clone());
        helper(&l4, im, col.clone());
        // helper(&l5, im, col.clone());
        // helper(&l6, im, col.clone());
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

impl Drawable for Circle {
    fn draw(&self, im :&mut dyn Displayable) {
        let col = Circle::color();
        for i in 0..=self.1 {
            for j in 0..=self.1 {
                let a = (i*i+j*j) as f64;
                if a.sqrt() as i32 == self.1 {
                    im.display(self.0.0+self.1-i,self.0.1+self.1-j,col.clone() );
                    im.display(self.0.0+self.1-i,self.0.1+self.1+j,col.clone() );
                    im.display(self.0.0+self.1+i,self.0.1+self.1-j,col.clone() );
                    im.display(self.0.0+self.1+i,self.0.1+self.1+j,col.clone() );

                }
            }
        }
    }

}

pub fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }
    b
}

pub fn helper(ddd : &Line, im : &mut dyn Displayable, col : Color) {
    let a = (ddd.1.0 - ddd.0.0) as f64;
        let b = (ddd.1.1 - ddd.0.1) as f64;
        let dis: f64 = (a * a + b * b).sqrt();


        let cc: f64 = a / dis.ceil();
        let dd: f64 = b / dis.ceil();
        let k = dis as i32;
        for i in 0..=k {
            // let m: i32 = (sel4096f.1.1-ddd.0.1) / (ddd.1.0-ddd.0.0) ;
            // println!("{} ===> {}",ddd.0.0+i*m,ddd.0.1+i*m);
            // let ccc =
            // cc += cc;
            // dd += dd;
            let x = ddd.0.0 + (((i as f64) * cc).round() as i32);
            let y = ddd.0.1 + (((i as f64) * dd).round() as i32);
            im.display(x, y, col.clone());
        }
}