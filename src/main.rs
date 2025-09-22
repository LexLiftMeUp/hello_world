struct Rectangle {
width: f64,
height: f64,
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
}