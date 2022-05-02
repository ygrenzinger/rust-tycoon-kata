use crate::Destination::B;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Destination {
    A,
    B,
}

fn compute_time_to_deliver(destination: Vec<Destination>) -> usize {
    if destination.is_empty() {
        0
    } else {
        let (a, b): (Vec<_>, Vec<_>) = destination
            .iter()
            .partition(|&dest| dest == &Destination::A);

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
    destination: Destination
}

impl Container {
    fn new(destination: Destination) -> Self {
        Self {
            destination
        }
    }

    fn is_delivered(&self) -> bool {
        true
    }
}

struct World {
    containers: Vec<Container>
}

impl World {
    // 1 destination B
    // 2 truck
    // Factory -> B

    fn new(destinations: Vec<Destination>) -> Self {
        Self {
            containers : destinations.into_iter().map(|dest| Container { destination : dest}).collect()
        }
    }

    fn deliver_containers(self) -> usize {
        while !self.is_everything_delivered() {
            // self.containers.iter_mut().for_each(|container| container.arrive_at(&container.destination))

        }
        0
    }

    fn is_everything_delivered(&self) -> bool {
        self.containers.iter().all(|c| c.is_delivered())
    }
}

#[derive(Debug)]
struct Transport {
    distance_traveled: u8,
    load: Option<(Container, u8)>
}

impl Transport {

    fn new() -> Self {
        Self {
            distance_traveled: 0,
            load: None
        }
    }

    fn load(&mut self, container: Container, destination: u8) {
        self.load = Some((container, destination));
    }

    fn is_available(&self) -> bool {
        return self.distance_traveled == 0
    }

    fn is_loaded(&self) -> bool {
        return self.load.is_some();
    }

    fn move_forward(&mut self) {
        match self.load {
            None => if self.distance_traveled > 0 {
                self.distance_traveled = self.distance_traveled - 1;
            }
            Some((_, d)) => {
                self.distance_traveled = self.distance_traveled + 1;
                if self.distance_traveled == d {
                    self.load = None;
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
    use super::Destination::{self, *};
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
        #[case] dest: Vec<Destination>,
        #[case] expect: usize,
        #[case] msg: &str,
    ) {
        let hours_to_deliver = compute_time_to_deliver(dest);
        assert_eq!(expect, hours_to_deliver, "{msg}");
    }

    #[test]
    #[ignore]
    fn should_compute_time_to_deliver_containers() {
        assert_eq!(5, World::new(vec![B]).deliver_containers());
        assert_eq!(15, World::new(vec![B, B, B]).deliver_containers());
    }

    #[test]
    fn should_not_move_if_transport_not_loaded() {
        let mut transport = Transport::new();
        transport.move_forward();
        assert_eq!(false, transport.is_loaded());
        assert_eq!(0, transport.distance_traveled);
    }

    #[test]
    fn should_move_if_transport_loaded() {
        let mut transport = Transport::new();
        transport.load(Container::new(B), 2);
        transport.move_forward();
        assert_eq!(true, transport.is_loaded());
        assert_eq!(1, transport.distance_traveled);
    }

    #[test]
    fn should_make_transport_reach_destination_and_unload() {
        let mut transport = Transport::new();
        transport.load(Container::new(B), 2);
        transport.move_forward();
        transport.move_forward();
        assert_eq!(2, transport.distance_traveled);
        assert_eq!(false, transport.is_loaded());
    }

}
