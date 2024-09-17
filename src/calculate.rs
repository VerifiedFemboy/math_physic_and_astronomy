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

pub fn orbital_period_in_years(radius: f64) -> f64 {
    (2.0 * PI * (radius.powf(3.0) / (G * M)).sqrt() / 31_536_000.0).round()
}

pub fn range_between_stars(star1: (f64, f64, f64), star2: (f64, f64, f64)) -> f64 {
    let x1 = star1.0;
    let y1 = star1.1;
    let z1 = star1.2;

    let x2 = star2.0;
    let y2 = star2.1;
    let z2 = star2.2;

    let d = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0) + (z1 - z2).powf(2.0)).sqrt();
    d
}

pub fn calculate_culomb_strength_with_gravity_strength(gravity_strength: f64, culomb_strength: f64) -> f64 {
    let strength: f64 = gravity_strength / culomb_strength;
    strength
}

// pub fn min_speed(radius: f64, planet_mass: f64) -> f64 {
//     let vc = ((2.0*G*M) / radius).sqrt();
//     vc
// }

//vc = sqrt((2*G*M)/R)
//G - constant gravity (6.667e-11)
//M - planet's mass
//R - planet's radius