pub fn convert_from_au_to_meters(input: f64) -> f64 {
    let meters = 1.496e11; //1 AU = 1.496*10111.496*10^11 meters
    let result = input * meters;
    result
}

pub fn convert_from_meter_to_km(input: f64) -> f64 {
    input / 1000.0
}