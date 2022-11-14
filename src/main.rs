
enum Destination {
    A,
    B,
}

struct Container {
    destination: Destination,
    is_delivered: bool,
}

#[derive(Clone)]
enum Transport {
    Waiting,
    ReturningToBase {
        distance_from_base: u8,
    },
    Shipping {
        container: Container,
        destination: Destination,
        distance_to_travel: u8,
        distance_from_base: u8,
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
        // map through Destination checking if transport is at destination
        let mut new_transports: Vec<Transport> = vec![];
        let new_containers: Vec<Container> = self.containers;

        for transport in self.transports.iter() {
            let (new_transport, new_containers) = self.tick_transport(transport, new_containers);
            new_transports.push(new_transport);
        }

        DeliverySystem {
            containers: new_containers,
            transports: new_transports,
            tick: self.tick + 1,
        }
    }

    fn tick_transport(self, transport: &Transport, containers: Vec<Container>) -> (Transport, Vec<Container>) {
        let new_transport = match transport {
            &Transport::Waiting => {
                let container = containers.pop();
                if let Some(container) = container {
                    Transport::Shipping { container: container, destination: container.destination, distance_to_travel: 5, distance_from_base: 0 } 
                } else {
                    transport.clone()
                }
            },
            &Transport::Shipping {..} => transport.clone(), // TODO : implements the thing that we discussed last week, if you remembered....
            &Transport::ReturningToBase {..} => transport.clone(),
        };
        (new_transport, containers) 
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
        DeliverySystem::new(containers, vec![Transport::Waiting, Transport::Waiting]);

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
