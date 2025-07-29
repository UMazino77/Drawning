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

#[derive(Debug, Clone)]
pub struct Point(i32, i32);
pub struct Line(pub Point, pub Point);
pub struct Triangle(pub Point, pub Point, pub Point);
// pub struct Rectangle(pub Point,pub Point);
// pub struct Circle(pub Point, i32);

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

// impl Triangle {
//     pub fn new(x: Point , y:Point, z : Point) -> Self {
//         Triangle(x,y,z)
//     }
// }

// impl Drawable for Triangle {
//     fn draw(&self, im : &mut dyn Displayable) {
//         im.display(x, y, color);
//     }
// }

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
   fn draw(&self, im : &mut dyn Displayable) {
    let a = abs(self.0.0-self.1.0);
    let b = abs(self.0.1-self.1.1);
    let mut dis : f64 = max(a as f64, b as f64) ;

    let col = Line::color();

    let cc  = a as f64 / dis;
    let dd = b as f64 / dis;
    dis = dis.sqrt();
    let k = dis as i32;
        for i in 0..=k {
            // let m: i32 = (sel4096f.1.1-self.0.1) / (self.1.0-self.0.0) ;
            // println!("{} ===> {}",self.0.0+i*m,self.0.1+i*m);
            // let ccc = 
            // cc += cc;
            // dd += dd;
            im.display(self.0.0+(i as f64 *cc) as i32,self.0.1+(i as f64 *dd) as i32 , col.clone());
        }

   }
}

// impl Drawable for Line {
//     fn draw(&self, im: &mut dyn Displayable) {
//         // let a = self.0.0-self.1.0;
//         // let b = self.0.1-self.1.1;
//         // let mut dis : f64 = (a*a+b*b) as f64;
//         // dis = dis.sqrt();
//         // let k = dis as i32;
//         let n: f64 = (self.1.1 - self.0.1) as f64;
//         let p :f64 = (self.1.0 - self.0.0) as f64;
//         let m = n/p;
//         let mm = m as i32;

//         println!("le point 1 : x = {} , y = {}\n\nle point 2 : x = {} , y = {}", self.0.0, self.0.1,self.1.0,self.1.1);

//         let col = Line::color();

//         let mut c_pos = 1;
//         let mut y_pos = 1;
//         if self.1.0-self.0.0 < 0{
//             c_pos = -1
//         }

//         if self.1.1-self.0.1 < 0{
//             y_pos = -1;
//         }


//         for y in 0..=abs(self.1.1 - self.0.1) {
//             for x in 0..=abs(self.1.0 - self.0.0) {
                
               
//                 // println!("{} ===> {}", self.0.0 + i * m, self.0.1 + i * m);
//                 let yy = y as f64;
//                 let xx = x as f64;
//                 if yy >= m*xx-2.0 && yy <= m*xx+2.0 {
//                     im.display((self.0.0 + c_pos*x), self.0.1 + y_pos*y, col.clone());
//                 }
//             }
//         }
//     }
// }

// impl Rectangle {
//     pub fn new(x : Point , y : Point) -> Self {
//         Rectangle(x, y)
//     }

// }

// impl color for Rectangle {
//         fn color()->Color {
//         let r = random::<u8>();
//         let g = random::<u8>();
//         let b = random::<u8>();

//         Color::rgb(r,g,b)
//     }
// }

// impl Circle {
//     pub fn new(c : Point , r : i32) -> Self {
//         Circle(c, r)
//     }

//     pub fn random(width:i32, height : i32) -> Self{
//         let a = random::<i32>() % width;
//         let b = random::<i32>() % height;
//         let c = random::<i32>() % (min(height,width)/2);

//         Circle::new(Point::new(a, b),c)
//     }
// }

// impl color for Circle {
//         fn color()->Color {
//         let r = random::<u8>();
//         let g = random::<u8>();
//         let b = random::<u8>();

//         Color::rgb(r,g,b)
//     }
// }

pub fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }
    b
}

pub fn abs(a: i32) -> i32 {
    if a < 0 {
        return -1 * a;
    }
    a
}


pub fn aabs(a: f64) -> f64 {
    if a < 0.0 {
        return -1.0 * a;
    }
    a
}

pub fn max(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}