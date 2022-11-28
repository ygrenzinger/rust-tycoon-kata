#[derive(Clone)]
enum Destination {
    A,
    B,
}

#[derive(Clone)]
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

impl Transport {
    fn tick(self, containers: Vec<Container>) -> (Transport, Vec<Container>) {
        match self {
            Transport::Waiting => {
                if let Some((container, rest)) = containers.clone().split_first() {
                    (
                        Transport::Shipping {
                            container: container.clone(),
                            destination: container.destination.clone(),
                            distance_to_travel: 5,
                            distance_from_base: 0,
                        },
                        rest.to_vec(),
                    )
                } else {
                    (self, containers)
                }
            }
            Transport::Shipping { .. } => (self, containers), // TODO : implements the thing that we discussed last week, if you remembered....
            Transport::ReturningToBase { .. } => (self, containers),
        }
    }
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
        let mut new_containers: Vec<Container> = self.containers;

        for transport in self.transports.into_iter() {
            let (new_transport, next_containers) = transport.tick(new_containers.clone());
            new_transports.push(new_transport);
            new_containers = next_containers;
        }

        DeliverySystem {
            containers: new_containers,
            transports: new_transports,
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
