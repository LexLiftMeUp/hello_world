fn main() {

    let mut value = 0b1111_0101u8;
    println!("the value is {}", value);
    println!("the value is {:08b}", value);

    value = !value;
    println!("the value is {:08b}", value);

    value = value & 0b1111_0111;
    println!("the value is {:08b}", value);
    println!("bit 6 is {}", value & 0b0100_0000);
}