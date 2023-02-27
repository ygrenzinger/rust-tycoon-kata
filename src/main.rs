use std::vec;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Location {
    FACTORY,
    PORT,
    A,
    B,
}

impl Location {
    fn to_roadmap(self) -> Vec<Segment> {
        match self {
            Location::FACTORY => vec![],
            Location::B => vec![Segment {
                destination: Location::B,
                distance: 5,
            }],
            Location::A => vec![
                Segment {
                    destination: Location::PORT,
                    distance: 1,
                },
                Segment {
                    destination: Location::A,
                    distance: 4,
                },
            ],
            Location::PORT => vec![Segment {
                destination: Location::PORT,
                distance: 1,
            }],
        }
    }
}

#[derive(Clone, Debug)]
struct Segment {
    destination: Location,
    distance: u8,
}

#[derive(Clone, Debug)]
struct Container {
    location: Option<Location>,
    destination: Location,
    roadmap: Vec<Segment>,
    is_delivered: bool,
}

#[derive(Clone, Debug)]
struct TransportShipping {
    container: Container,
    destination: Location,
    distance_to_travel: u8,
    distance_from_base: u8,
}

#[derive(Clone, Debug)]
struct TransportReturningToBase {
    distance_from_base: u8,
}

#[derive(Clone, Debug)]
enum Transport {
    Waiting, // TODO : should contain Base to manage the relation  between transport base and container location
    ReturningToBase(TransportReturningToBase),
    Shipping(TransportShipping),
}

impl Transport {
    fn tick_waiting_transport(
        transport: Transport,
        waitingContainers: Vec<Container>,
        unloadedContainers: Vec<Container>,
    ) -> (Transport, Vec<Container>, Vec<Container>) {
        if let Some((container, rest)) = waitingContainers.clone().split_first() {
            if !container.is_delivered {
                let distance_to_travel = container.roadmap[0].distance;
                let destination = container.roadmap[0].destination;
                let transport = TransportShipping {
                    container: container.clone(),
                    destination,
                    distance_to_travel,
                    distance_from_base: 1,
                };
                if distance_to_travel == 1 {
                    Transport::delivering(transport, rest.to_vec(), unloadedContainers)
                } else {
                    (
                        Transport::Shipping(transport),
                        rest.to_vec(),
                        unloadedContainers,
                    )
                }
            } else {
                (transport, waitingContainers, unloadedContainers)
            }
        } else {
            (transport, waitingContainers, unloadedContainers)
        }
    }

    fn moving_loaded_transport(
        transport: TransportShipping,
        containers: Vec<Container>,
    ) -> (Transport, Vec<Container>) {
        (
            Transport::Shipping(TransportShipping {
                container: transport.container,
                destination: transport.destination,
                distance_from_base: transport.distance_from_base + 1,
                distance_to_travel: transport.distance_to_travel,
            }),
            containers,
        )
    }

    fn delivering(
        transport: TransportShipping,
        waitingContainers: Vec<Container>,
        mut unloadContainers: Vec<Container>,
    ) -> (Transport, Vec<Container>, Vec<Container>) {
        let location = transport.destination.clone();
        let returning_to_base = Transport::ReturningToBase(TransportReturningToBase {
            distance_from_base: transport.distance_from_base,
        });
        let remaining_roadmap = &transport.container.roadmap[1..];
        let container = Container {
            location: Some(location),
            destination: transport.container.destination,
            roadmap: remaining_roadmap.to_vec(),
            is_delivered: location == transport.container.destination,
        };
        unloadContainers.push(container);
        (returning_to_base, waitingContainers, unloadContainers)
    }

    fn returning_to_base(
        transport: TransportReturningToBase,
        containers: Vec<Container>,
    ) -> (Transport, Vec<Container>) {
        let transport = if transport.distance_from_base == 0 {
            Transport::Waiting
        } else {
            Transport::ReturningToBase(TransportReturningToBase {
                distance_from_base: transport.distance_from_base - 1,
            })
        };
        (transport, containers)
    }

    fn tick(
        self,
        waitingContainers: Vec<Container>,
        unloadedContainers: Vec<Container>,
    ) -> (Transport, Vec<Container>, Vec<Container>) {
        match self {
            // Load and move 1 distance if available containers
            Transport::Waiting => {
                Transport::tick_waiting_transport(self, waitingContainers, unloadedContainers)
            }
            // Unload if at destination
            Transport::Shipping(transport)
                if transport.distance_from_base + 1 == transport.distance_to_travel =>
            {
                Transport::delivering(transport, waitingContainers, unloadedContainers)
            }
            // move loaded transport to destination
            Transport::Shipping(transport) => {
                let (transport, waitingContainers) =
                    Transport::moving_loaded_transport(transport, waitingContainers);
                (transport, waitingContainers, unloadedContainers)
            }
            // returning to base
            Transport::ReturningToBase(transport) => {
                let (transport, waitingContainers) =
                    Transport::returning_to_base(transport, waitingContainers);
                (transport, waitingContainers, unloadedContainers)
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
    fn new(destinations: Vec<Location>, transports: Vec<Transport>) -> DeliverySystem {
        DeliverySystem {
            containers: destinations
                .into_iter()
                .map(|destination| Container {
                    location: Some(Location::FACTORY),
                    destination: destination.clone(),
                    roadmap: destination.to_roadmap(),
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
        let mut waitingContainers: Vec<Container> = self.containers;
        let mut unloadedContainers: Vec<Container> = vec![];
        for transport in self.transports.into_iter() {
            let (new_transport, next_waiting_containers, next_unloaded_containers) =
                transport.tick(waitingContainers.clone(), unloadedContainers.clone());
            new_transports.push(new_transport);
            waitingContainers = next_waiting_containers;
            unloadedContainers = next_unloaded_containers;
        }

        DeliverySystem {
            containers: [waitingContainers, unloadedContainers].concat(),
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

        //dbg!(&self.transports);
        //dbg!(&self.containers);

        //println!("any_transport_shipping {any_transport_shipping} all_containers_delivered {all_containers_delivered} ");
        !any_transport_shipping && all_containers_delivered
    }

    fn tick_count(&self) -> u32 {
        self.tick
    }
}

fn run(containers: Vec<Location>) -> u32 {
    let mut delivery_system = DeliverySystem::new(
        containers,
        vec![Transport::Waiting, Transport::Waiting, Transport::Waiting],
    );
    while !delivery_system.all_are_delivered() {
        delivery_system = delivery_system.tick();
    }
    return delivery_system.tick_count();
}

fn main() {
    run(vec![Location::B]);
}

#[test]
fn test_scenario_1() {
    assert_eq!(5, run(vec![Location::B]));
}

#[test]
fn test_scenario_2() {
    assert_eq!(5, run(vec![Location::B, Location::B]));
}

#[test]
fn test_scenario_3() {
    assert_eq!(15, run(vec![Location::B, Location::B, Location::B]));
}

#[test]
fn test_scenario_4() {
    assert_eq!(11, run(vec![Location::B, Location::B, Location::PORT]))
}

#[test]
fn test_scenario_5() {
    assert_eq!(5, run(vec![Location::A]))
}

#[test]
fn test_scenario_6() {
    assert_eq!(7, run(vec![Location::A, Location::B, Location::B]))
}
