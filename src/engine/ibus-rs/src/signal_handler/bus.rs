use ibus::Bus;

pub fn connected(_bus: &Bus) {
    println!("Bus connected!");
}

pub fn disconnected(_bus: &Bus) {
    ibus::quit();
    println!("Bus disconnected!");
}
