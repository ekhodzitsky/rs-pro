struct DistanceInKilometers(f64);

struct DistanceInMiles(f64);

impl DistanceInKilometers {
    fn to_miles(&self) -> DistanceInMiles {
        DistanceInMiles(self.0 * 0.6)
    }
}

impl DistanceInMiles {
    fn to_kilometers(&self) -> DistanceInKilometers {
        DistanceInKilometers((self.0 * 1.6667).round())
    }
}

pub fn demo() {
    println!("Newtype");

    let distance_km = DistanceInKilometers(100.0);
    let distance_miles = distance_km.to_miles();
    let converted_back = distance_miles.to_kilometers();

    println!("Distance in kilometers: {} km", distance_km.0);
    println!("Distance in miles: {} miles", distance_miles.0);
    println!("Converted back to kilometers: {} km", converted_back.0);

    println!();
}
