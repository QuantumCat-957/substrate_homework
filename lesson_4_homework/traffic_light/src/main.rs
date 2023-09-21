enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait TimeDuration {
    fn duration(&self) -> u32;
}

impl TimeDuration for TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,   // Red light lasts for 30 seconds
            TrafficLight::Yellow => 5, // Yellow light lasts for 5 seconds
            TrafficLight::Green => 45, // Green light lasts for 45 seconds
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;

    println!("Duration of red light: {} seconds", red_light.duration());
    println!(
        "Duration of yellow light: {} seconds",
        yellow_light.duration()
    );
    println!(
        "Duration of green light: {} seconds",
        green_light.duration()
    );
}
