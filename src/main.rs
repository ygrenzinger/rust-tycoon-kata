use std::collections::VecDeque;

enum Destination {
    A,
    B,
}

struct Container {
    destination: Destination,
    is_delivered: bool,
}

enum Transport {
    Empty,
    ReturningToBase {
        distanceFromBase: u8,
    },
    Shipping {
        container: Container,
        destination: Destination,
        distanceToTravel: u8,
        distanceFromBase: u8,
    },
}

struct DeliverySystem {
    containers: Vec<Container>,
    tick: u32,
    transports: Vec<Transport>,
}

impl DeliverySystem {
    fn new(destinations: Vec<Destination>, transports: Vec<Transport>) -> DeliverySystem {
        DeliverySystem {
            containers: destinations
                .into_iter()
                .map(|destination| Container {
                    destination,
                    is_delivered: true,
                })
                .collect(),
            tick: 0,
            transports,
        }
    }

    fn tick(self) -> DeliverySystem {
        // map through Destination checking if transport
        DeliverySystem {
            containers: self.containers,
            transports: self.transports,
            tick: self.tick + 1,
        }
    }

    fn all_are_delivered(&self) -> bool {
        self.containers
            .iter()
            .all(|container| container.is_delivered)
    }

    fn tick_count(&self) -> u32 {
        self.tick
    }
}

fn run(containers: Vec<Destination>) -> u32 {
    let mut delivery_system =
        DeliverySystem::new(containers, vec![Transport::Empty, Transport::Empty]);

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

#[test]
fn test_scenario_2() {
    assert_eq!(5, run(vec![Destination::B, Destination::B]));
}

#[test]
fn test_scenario_3() {
    assert_eq!(
        15,
        run(vec![Destination::B, Destination::B, Destination::B])
    );
}
