use std::io::stdin;

const G: f64 = 6.674e-11; //constant GRAVITY
const M: f64 = 1.989e30; //constant MASS of Sun

mod calculate;
mod convert;

fn main() {
    // height_and_gravity();
    range();
}


fn height_and_gravity() {
    println!("Please enter the height in meters");
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let height: f64 = input.trim().parse().unwrap();

    let g = calculate::calculate_gravity_acceleration(5.972e24, 6.371e6);
    let t = calculate::falling_time(height, g);
    let v = calculate::speed_in_hit(g, t);

    println!("\nGravity acceleration: {g} m/s^2");
    println!("Falling time: {t} s");
    println!("Speed in hit: {v} m/s");

}

fn distance_and_orbit() {
    println!("Please enter the distance in AU:");
    let mut input = String::new();

    let _ = stdin().read_line(&mut input).expect("Something went wrong while buffering");
    
    input = input.trim().to_string();
    
    let result_input = input.parse().unwrap();

    //Converting to meters
    let distance = convert::convert_from_au_to_meters(result_input);

    println!("\nDistance: {distance} meters");

    let mut speed = calculate::calculate_orbital_speed(distance);

    speed = convert::convert_from_meter_to_km(speed);

    println!("Speed: {speed} km/s");


    let orbit_in_seconds = calculate::orbital_period_in_seconds(distance);
    println!("Orbital period in seconds: {orbit_in_seconds}");
}

fn range() {
    let star1 = (1.0, 2.0, 3.0);
    let star2 = (4.0, 5.0, 6.0);

    let range = calculate::range_between_stars(star1, star2);

    println!("Distance between two stars is {range}");
}