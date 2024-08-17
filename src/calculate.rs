use crate::{G, M};
use std::f64::consts::PI as PI;

pub fn calculate_orbital_speed(radius: f64) -> f64 {
    let v = (G * M) / radius;
    v.sqrt()
}

pub fn calculate_gravity_acceleration(planet_mass: f64, planet_radius: f64) -> f64 {
    let g = (G * planet_mass) / planet_radius.powf(2.0);
    g
}

pub fn falling_time(height: f64, g: f64) -> f64 {
    let t = (2.0 * height) / g;
    t.sqrt()
}

pub fn speed_in_hit(g: f64, t: f64) -> f64 {
    let v = g * t;
    v
}

//I don't know is it correct I will have to figure out
pub fn orbital_period_in_seconds(radius: f64) -> f64 {
    2.0 * PI * (radius.powf(3.0) / (G * M)).sqrt()
}