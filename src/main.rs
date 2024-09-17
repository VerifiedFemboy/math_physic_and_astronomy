use std::io::stdin;

const G: f64 = 6.674e-11; //constant GRAVITY
const M: f64 = 1.989e30; //constant MASS of Sun
const K: f64 = 9e9; //constant ELECTROSTATIC


mod calculate;
mod convert;

fn main() {
    println!("Welcome to the Space Calculator!");

    loop {
        println!("\nPlease select an option:");
        println!("1. Height and Gravity");
        println!("2. Distance and Orbit");
        println!("3. Range between stars");
        println!("4. Compare the strength of the electrostatic force to the strength of the gravitational force");
        println!("5. Exit");

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        let input: u32 = input.trim().parse().unwrap();

        match input {
            1 => height_and_gravity(),
            2 => distance_and_orbit(),
            3 => range(),
            4 => compare(),
            5 => break,
            _ => println!("Invalid input. Please try again."),
        }
    }
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


    let orbit_in_seconds = calculate::orbital_period_in_seconds(distance).round();
    println!("Orbital period in seconds: {orbit_in_seconds}");
    let orbit_in_years = calculate::orbital_period_in_years(distance);
    println!("Orbital period in seconds: {orbit_in_years}");

}

fn range() {
    let star1 = (1.0, 2.0, 3.0);
    let star2 = (4.0, 5.0, 6.0);

    let range = calculate::range_between_stars(star1, star2);

    println!("Distance between two stars is {range}");
}

//TODO: finish it because it's not correct
fn compare() {
    let me = 9.1e-31;
    let mp = 1.67e-27;
    let gravity = G * (me * mp) / 1.0 * 1.0;
    let q = 1.6e-19;
    let culomb = K / (q * q) / 1.0 * 1.0;

    let result = calculate::calculate_culomb_strength_with_gravity_strength(gravity, culomb);
                
    println!("The ratio of the strength of the electrostatic force to the strength of the gravitational force is {result}");

}