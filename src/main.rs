use std::io::stdin;
use std::f64::consts::PI as PI;

const G: f64 = 6.674e-11; //constant GRAVITY
const M: f64 = 1.989e30; //constant MASS //constant PI

fn main() {
    println!("Please enter the distance in AU:");
    let mut input = String::new();

    let _ = stdin().read_line(&mut input).expect("Something went wrong while buffering");
    
    input = input.trim().to_string();
    
    let result_input = input.parse().unwrap();

    //Converting to meters
    let distance = convert_from_au_to_meters(result_input);

    println!("\nDistance: {distance} meters");

    let mut speed = calculate_orbital_speed(distance);

    speed = convert_from_meter_to_km(speed);

    println!("Speed: {speed} km/s");


    let orbit_in_seconds = orbital_period_in_seconds(distance);
    println!("Orbital period in seconds: {orbit_in_seconds}");
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


//I don't know is it correct I will have to find out
fn orbital_period_in_seconds(radius: f64) -> f64 {
    2.0 * PI * (radius.powf(3.0) / (G * M)).sqrt()
}