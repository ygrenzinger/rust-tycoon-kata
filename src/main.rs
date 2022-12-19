#[derive(Clone, Debug)]
enum Destination {
    A,
    B,
}

#[derive(Clone, Debug)]
struct Container {
    destination: Destination,
    is_delivered: bool,
}

#[derive(Clone, Debug)]
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

trait TransportTrait {
    fn tick(self, containers: Vec<Container>) -> (Box<dyn TransportTrait>, Vec<Container>);
}

struct TransportWaiting;

impl TransportTrait for TransportWaiting {
    fn tick(self, containers: Vec<Container>) -> (Box<dyn TransportTrait>, Vec<Container>) {
        return (Box::new(self), containers);
    }
}

struct TransportReturningToBase {
    distance_from_base: u8,
}
struct TransportShipping {
    container: Container,
    destination: Destination,
    distance_to_travel: u8,
    distance_from_base: u8,
}

impl Transport {
    fn tick(self, mut containers: Vec<Container>) -> (Transport, Vec<Container>) {
        match self {
            Transport::Waiting => {
                if let Some((container, rest)) = containers.clone().split_first() {
                    if !container.is_delivered {
                        (
                            Transport::Shipping {
                                container: container.clone(),
                                destination: container.destination.clone(),
                                distance_to_travel: 5,
                                distance_from_base: 1,
                            },
                            rest.to_vec(),
                        )
                    } else {
                        (self, containers)
                    }
                } else {
                    (self, containers)
                }
            }
            Transport::Shipping {
                container,
                destination,
                distance_from_base,
                distance_to_travel,
            } => {
                if distance_from_base + 1 == distance_to_travel {
                    let transport = Transport::ReturningToBase {
                        distance_from_base: distance_from_base,
                    };
                    let container = Container {
                        destination,
                        is_delivered: true,
                    };
                    containers.push(container);
                    (transport, containers)
                } else {
                    (
                        Transport::Shipping {
                            container,
                            destination,
                            distance_from_base: distance_from_base + 1,
                            distance_to_travel,
                        },
                        containers,
                    )
                }
            }
            Transport::ReturningToBase { distance_from_base } => {
                let transport = if distance_from_base == 0 {
                    Transport::Waiting
                } else {
                    Transport::ReturningToBase {
                        distance_from_base: distance_from_base - 1,
                    }
                };
                (transport, containers)
            }
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
                    is_delivered: false,
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
        let any_transport_shipping = self
            .transports
            .iter()
            .any(|transport| matches!(transport, Transport::Shipping { .. }));
        let all_containers_delivered = self
            .containers
            .iter()
            .all(|container| container.is_delivered);

        dbg!(&self.transports);
        dbg!(&self.containers);

        println!("any_transport_shipping {any_transport_shipping} all_containers_delivered {all_containers_delivered} ");
        !any_transport_shipping && all_containers_delivered
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
    run(vec![Destination::B]);
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
