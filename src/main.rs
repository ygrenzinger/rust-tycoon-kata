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
                origin: Location::FACTORY,
                destination: Location::B,
                distance: 5,
            }],
            Location::A => vec![
                Segment {
                    origin: Location::FACTORY,
                    destination: Location::PORT,
                    distance: 1,
                },
                Segment {
                    origin: Location::PORT,
                    destination: Location::A,
                    distance: 4,
                },
            ],
            Location::PORT => vec![Segment {
                origin: Location::FACTORY,
                destination: Location::PORT,
                distance: 1,
            }],
        }
    }
}

#[derive(Clone, Debug)]
struct Segment {
    origin: Location,
    destination: Location,
    distance: u8,
}

#[derive(Clone, Debug)]
struct Container {
    destination: Location,
    roadmap: Vec<Segment>,
    is_delivered: bool,
}

#[derive(Clone, Debug)]
struct TransportWaiting {
    base: Location,
}

#[derive(Clone, Debug)]
struct TransportShipping {
    base: Location,
    container: Container,
    destination: Location,
    distance_to_travel: u8,
    distance_from_base: u8,
}

#[derive(Clone, Debug)]
struct TransportReturningToBase {
    base: Location,
    distance_from_base: u8,
}

#[derive(Clone, Debug)]
enum Transport {
    Waiting(TransportWaiting),
    ReturningToBase(TransportReturningToBase),
    Shipping(TransportShipping),
}

impl Transport {
    fn base(&self) -> Location {
        match self {
            Transport::Waiting(TransportWaiting { base }) => base.to_owned(),
            Transport::ReturningToBase(TransportReturningToBase { base, .. }) => base.to_owned(),
            Transport::Shipping(TransportShipping { base, .. }) => base.to_owned(),
        }
    }

    fn tick_waiting_transport(
        transport: Transport,
        containers: Vec<Container>,
        unloaded_containers: Vec<Container>,
    ) -> (Transport, Vec<Container>, Vec<Container>) {
        let mut containers_to_be_loaded = containers.clone();
        if let Some(index) = containers_to_be_loaded.iter().position(|container| {
            !container.is_delivered && container.roadmap[0].origin == transport.base()
        }) {
            let container_to_be_loaded = containers_to_be_loaded.remove(index);
            let distance_to_travel = container_to_be_loaded.roadmap[0].distance;
            let destination = container_to_be_loaded.roadmap[0].destination;
            let transport = TransportShipping {
                base: transport.base().to_owned(),
                container: container_to_be_loaded.clone(),
                destination,
                distance_to_travel,
                distance_from_base: 1,
            };
            if distance_to_travel == 1 {
                Transport::delivering(transport, containers_to_be_loaded, unloaded_containers)
            } else {
                (
                    Transport::Shipping(transport),
                    containers_to_be_loaded,
                    unloaded_containers,
                )
            }
        } else {
            (transport, containers, unloaded_containers)
        }
    }

    fn moving_loaded_transport(
        transport: TransportShipping,
        containers: Vec<Container>,
    ) -> (Transport, Vec<Container>) {
        (
            Transport::Shipping(TransportShipping {
                base: transport.base,
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
        containers: Vec<Container>,
        mut unload_containers: Vec<Container>,
    ) -> (Transport, Vec<Container>, Vec<Container>) {
        let location = transport.destination.clone();
        let distance_from_base = if transport.distance_from_base == 1 {
            0
        } else {
            transport.distance_from_base
        };
        let returning_to_base = Transport::ReturningToBase(TransportReturningToBase {
            base: transport.base,
            distance_from_base,
        });
        let remaining_roadmap = &transport.container.roadmap[1..];
        let container = Container {
            destination: transport.container.destination,
            roadmap: remaining_roadmap.to_vec(),
            is_delivered: location == transport.container.destination,
        };
        unload_containers.push(container);
        (returning_to_base, containers, unload_containers)
    }

    fn returning_to_base(
        transport: TransportReturningToBase,
        containers: Vec<Container>,
    ) -> (Transport, Vec<Container>) {
        let transport = if transport.distance_from_base == 0 {
            Transport::Waiting(TransportWaiting {
                base: transport.base,
            })
        } else {
            Transport::ReturningToBase(TransportReturningToBase {
                base: transport.base,
                distance_from_base: transport.distance_from_base - 1,
            })
        };
        (transport, containers)
    }

    fn tick(
        self,
        containers: Vec<Container>,
        unloaded_containers: Vec<Container>,
    ) -> (Transport, Vec<Container>, Vec<Container>) {
        match self {
            // Load and move 1 distance if available containers
            Transport::Waiting(TransportWaiting { base }) => {
                Transport::tick_waiting_transport(self, containers, unloaded_containers)
            }
            // Unload if at destination
            Transport::Shipping(transport)
                if transport.distance_from_base + 1 == transport.distance_to_travel =>
            {
                Transport::delivering(transport, containers, unloaded_containers)
            }
            // move loaded transport to destination
            Transport::Shipping(transport) => {
                let (transport, waiting_containers) =
                    Transport::moving_loaded_transport(transport, containers);
                (transport, waiting_containers, unloaded_containers)
            }
            // returning to base
            Transport::ReturningToBase(transport) => {
                let (transport, waiting_containers) =
                    Transport::returning_to_base(transport, containers);
                (transport, waiting_containers, unloaded_containers)
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
        let mut containers: Vec<Container> = self.containers;
        let mut unloaded_containers: Vec<Container> = vec![];
        for transport in self.transports.into_iter() {
            let (new_transport, next_waiting_containers, next_unloaded_containers) =
                transport.tick(containers.clone(), unloaded_containers.clone());
            new_transports.push(new_transport);
            containers = next_waiting_containers;
            unloaded_containers = next_unloaded_containers;
        }

        DeliverySystem {
            containers: [containers, unloaded_containers].concat(),
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
        vec![
            Transport::Waiting(TransportWaiting {
                base: Location::FACTORY,
            }),
            Transport::Waiting(TransportWaiting {
                base: Location::FACTORY,
            }),
            Transport::Waiting(TransportWaiting {
                base: Location::PORT,
            }),
        ],
    );
    while !delivery_system.all_are_delivered() {
        delivery_system = delivery_system.tick();
    }
    return delivery_system.tick_count();
}

fn main() {
    run(vec![Location::B]);
}

/*

#[test]
fn test_scenario_0() {
    assert_eq!(1, run(vec![Location::PORT]));
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
    assert_eq!(5, run(vec![Location::A]))
}

#[test]
fn test_scenario_5() {
    assert_eq!(13, run(vec![Location::A, Location::A]))
}

#[test]
fn test_scenario_6() {
    assert_eq!(5, run(vec![Location::A, Location::B]))
}

#[test]
fn test_scenario_7() {
    assert_eq!(15, run(vec![Location::B, Location::B, Location::A]))
}

#[test]
fn test_scenario_8() {
    assert_eq!(7, run(vec![Location::A, Location::B, Location::B]))
}

*/

#[test]
fn test_complete() {
    assert_eq!(
        29,
        run(vec![
            Location::A,
            Location::A,
            Location::B,
            Location::A,
            Location::B,
            Location::B,
            Location::A,
            Location::B
        ])
    )
}
