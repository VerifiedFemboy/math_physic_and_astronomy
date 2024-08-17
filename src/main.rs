use std::io::stdin;

const G: f64 = 6.674e-11; //constant GRAVITY
const M: f64 = 1.989e30; //constant MASS

fn main() {
    println!("Please enter the distance in AU:");
    let mut input = String::new();

    let _ = stdin().read_line(&mut input).expect("Something went wrong while buffering");
    
    input = input.trim().to_string();

    let result = convert_from_au_to_meters(input.parse().expect("Something went wrong while parsing"));

    println!("{result} meters");

    let mut speed = calculate_orbital_speed(result.clone());

    speed = convert_from_meter_to_km(speed);

    println!("Speed: {speed} km/s");
}

fn convert_from_au_to_meters(input: f64) -> f64 {
    let meters = 1.496e11; //1 AU = 1.496*10111.496*10^11 meters
    let result = input * meters;
    result
}

fn calculate_orbital_speed(radius: f64) -> f64 {
    let v = (G * M) / radius;
    v.sqrt()
}

fn convert_from_meter_to_km(input: f64) -> f64 {
    input / 1000.0
}