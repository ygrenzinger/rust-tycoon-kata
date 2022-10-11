use std::collections::VecDeque;

enum Destination {
    A,
    B
}

struct DeliverySystem {
    tick: u32,
}

impl DeliverySystem {
    fn tick(&self) -> DeliverySystem {
        DeliverySystem { tick: self.tick + 1 }        
    }
    fn all_are_delivered(&self) -> bool {
        self.tick >= 5
    }

    fn tick_count(&self) -> u32 {
        self.tick
    }
}

fn run(items: Vec<Destination>) -> u32 {
    let mut delivery_system = DeliverySystem { tick: 0};

    while !delivery_system.all_are_delivered() {
        delivery_system = delivery_system.tick();
    }
    return delivery_system.tick_count();
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_scenario_1() {
    assert_eq!(5, run(vec![Destination::B]));
}