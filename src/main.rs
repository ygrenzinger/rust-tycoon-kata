use crate::Location::{B, FACTORY};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Location {
    FACTORY,
    A,
    B,
}

fn compute_time_to_deliver(destination: Vec<Location>) -> usize {
    if destination.is_empty() {
        0
    } else {
        let (a, b): (Vec<_>, Vec<_>) = destination
            .iter()
            .partition(|&dest| dest == &Location::A);

        let a_count = a.len();
        let b_count = b.len();

        if b_count > 0 {
            ((b_count - 1) / 2) * 10 + 5
        } else {
            (((a_count - 1) / 2) + 1) + (((a_count - 1)) * 8 + 4)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Container {
    destination: Location,
    location: Option<Location>
}

impl Container {
    fn new(destination: Location) -> Self {
        Self {
            destination,
            location: Some(FACTORY)
        }
    }

    fn loaded(self) -> Self {
        return Container {
            destination: self.destination,
            location: None
        }
    }

    fn is_delivered(&self) -> bool {
        return match &self.location {
            None => false,
            Some(location) => location == &self.destination
        }
    }
}

struct World {
    containers: Vec<Container>
}

impl World {
    // 1 destination B
    // 2 truck
    // Factory -> B

    fn new(destinations: Vec<Location>) -> Self {
        Self {
            containers : destinations.into_iter().map(|dest| Container::new(dest)).collect()
        }
    }

    fn deliver_containers(self) -> usize {
        let mut time_elapsed = 0;
        while !self.is_everything_delivered() {
            // self.containers.iter_mut().for_each(|container| container.arrive_at(&container.destination))
            time_elapsed = time_elapsed + 1;

        }
        time_elapsed
    }

    fn is_everything_delivered(&self) -> bool {
        true
        //self.containers.iter().all(|c| c.is_delivered())
    }
}

#[derive(Debug)]
struct Transport {
    distance: u8,
    load: Option<(Container, u8)>
}

impl Transport {

    fn new() -> Self {
        Self {
            distance: 0,
            load: None
        }
    }

    fn load(&mut self, container: Container, destination: u8) {
        self.load = Some((container.loaded(), destination));
    }

    fn is_available(&self) -> bool {
        return self.distance == 0
    }

    fn is_loaded(&self) -> bool {
        return self.load.is_some();
    }

    fn move_forward(self) -> Self {
        return match self.load {
            None => if self.distance > 0 {
                Transport {
                    distance: self.distance - 1,
                    load: self.load
                }
            } else {
                self
            },
            Some((_, d)) => {
                let distance = self.distance + 1;
                let load = if distance == d {
                    None
                } else {
                    self.load
                };
                Transport {
                    distance,
                    load
                }
            }
        }
    }
}

//    P -------- A
//    | 1 weight + 4 weight
// X -
//    | 5 weight
//    B

#[cfg(test)]
mod test {
    use super::Location::{self, *};
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![], 0, "hello world")]
    #[case(vec![B], 5, "one truck 5h")]
    #[case(vec![B, B], 5, "2 truck 5h")]
    #[case(vec![B, B, B], 15, "2 truck 15h")]
    #[case(vec![B, B, B, B], 15, "2 truck 15h")]
    #[case(vec![B, B, B, B, B], 25, "2 truck 25h")]
    #[case(vec![B, B, B, B, B, B, B], 35, "2 truck 35")]
    #[case(vec![A], 5, "one truck 1h + one boat 4h")]
    #[case(vec![A, A], 13, "two trucks 1h + one boat 4h, 1 + 4 + 4 + 4 = 13")]
    // #[case(vec![A, A, B, A, B, B, A, B], 29, "")]
    #[ignore]
    fn should_compute_time_with_2_trucks(
        #[case] dest: Vec<Location>,
        #[case] expect: usize,
        #[case] msg: &str,
    ) {
        let hours_to_deliver = compute_time_to_deliver(dest);
        assert_eq!(expect, hours_to_deliver, "{msg}");
    }

    #[test]
    fn should_compute_time_to_deliver_containers() {
        assert_eq!(5, World::new(vec![B]).deliver_containers());
        assert_eq!(5, World::new(vec![B, B]).deliver_containers());
        assert_eq!(15, World::new(vec![B, B, B]).deliver_containers());
    }

    #[test]
    fn should_not_leave_base_if_transport_not_loaded() {
        let mut transport = Transport::new();
        let transport = transport.move_forward();
        assert_eq!(false, transport.is_loaded());
        assert_eq!(0, transport.distance);
    }

    #[test]
    fn should_move_if_transport_loaded() {
        let mut transport = Transport::new();
        transport.load(Container::new(B), 2);
        let transport = transport.move_forward();
        assert_eq!(true, transport.is_loaded());
        assert_eq!(1, transport.distance);
    }

    #[test]
    fn should_make_transport_reach_destination_and_unload() {
        let mut transport = Transport::new();
        transport.load(Container::new(B), 2);
        let transport = (0..2).fold(transport, |transport,_| transport.move_forward());
        assert_eq!(2, transport.distance);
        assert_eq!(false, transport.is_loaded());
    }

    #[test]
    fn should_make_transport_return_to_destination() {
        let mut transport = Transport::new();
        transport.load(Container::new(B), 2);
        let transport = (0..4).fold(transport, |transport,_| transport.move_forward());
        assert_eq!(0, transport.distance);
        assert_eq!(false, transport.is_loaded());
    }

}
