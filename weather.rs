fn average_temperature(temps: &[f64]) -> f64 {
    let sum: f64 = temps.iter().sum();
    sum / temps.len() as f64
}

fn main_weather() {
    let temperatures = [32.0, 34.5, 30.0, 29.0, 35.0, 33.0, 31.5];
    let last_three_days = &temperatures[temperatures.len() - 3..];
    println!("Last 3 days average temperature: {:.2}", average_temperature(last_three_days));
}

fn main() {
    main_weather();
}

