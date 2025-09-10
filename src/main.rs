fn main() {

    let celcius_temp = 23.0;
    let farenheit_temp = celcius_to_farenheit(celcius_temp);

    assert_eq!(farenheit_temp, 73.4);
    println!("test passed");
}

    fn celcius_to_farenheit(x: f64) -> f64{
        x * 1.8 + 32.0
    }