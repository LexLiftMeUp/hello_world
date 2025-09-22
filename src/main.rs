struct Rectangle {
    width: f64,
    height: f64,
    }
struct Circle {
    radius: f64, // assume radius > 0
    }

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius } // create a new circle with the given radius
    }

    fn area(&self) -> f64 {
        self.radius * self.radius * 3.14 // return the area computation
    }

    fn circumference(&self) -> f64 {
        3.14*self.radius*2.0 // return circumference computation
    }
    
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        self.height * self.width // this returns the area
    }

    fn perimeter(&self) -> f64 {
        2.0*self.height + 2.0*self.width //this calculates and returns perimenter
    }
    fn is_square(&self) -> bool {
        if self.height == self.width { //logic for determining if its a square
            return true
        }
        else {
            return false
        }
    }
}
fn main() {
    let rect = Rectangle::new(5.0, 5.0);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());
    assert!(Rectangle::new(5.0, 5.0).is_square());
    assert!(!Rectangle::new(5.0, 6.0).is_square());

    let round = Circle::new(5.0);
    println!("Area: {}", round.area());
    println!("Circumferene: {}", round.circumference());
}