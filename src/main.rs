use crate::Location::{B, FACTORY};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Clone)]
enum Location {
    FACTORY,
    A,
    B,
}

fn compute_time_to_deliver(destination: Vec<Location>) -> usize {
    if destination.is_empty() {
        0
    } else {
        let (a, b): (Vec<_>, Vec<_>) = destination.iter().partition(|&dest| dest == &Location::A);

        let a_count = a.len();
        let b_count = b.len();

        if b_count > 0 {
            ((b_count - 1) / 2) * 10 + 5
        } else {
            (((a_count - 1) / 2) + 1) + ((a_count - 1) * 8 + 4)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Container {
    destination: Location,
    location: Option<Location>,
}

impl Container {
    fn new(destination: Location) -> Self {
        Self {
            destination,
            location: Some(FACTORY),
        }
    }

    fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    fn loaded(&mut self) {
        self.location = None;
    }

    fn is_delivered(&self) -> bool {
        return match &self.location {
            None => false,
            Some(location) => location == &self.destination,
        };
    }
}

struct World {
    containers: Vec<Container>,
}

impl World {
    // 1 destination B
    // 2 truck
    // Factory -> B

    fn new(destinations: Vec<Location>) -> Self {
        Self {
            containers: destinations
                .into_iter()
                .map(|dest| Container::new(dest))
                .collect(),
        }
    }

    fn deliver_containers(&mut self) -> usize {
        let mut transport = Transport::new_empty();
        let mut time_elapsed = 0;

        let mut all_containers = self.containers.iter_mut();
        while !all_containers.all(|c| c.is_delivered()) {
            let next_available_container =
                all_containers.find(|c| !(c.location == None || c.is_delivered()));

            transport.move_forward(next_available_container);

            time_elapsed = time_elapsed + 1;
        }
        time_elapsed
    }
}

#[derive(Debug, Clone)]
struct EmptyTransport {
    distance: u8,
}

struct LoadedTransport<'c> {
    distance: u8,
    load: (&'c Container, u8, Location),
}

enum Transport<'c> {
    Empty(EmptyTransport),
    Loaded(LoadedTransport<'c>),
}

impl<'c> Transport<'c> {
    fn new_empty() -> Self {
        Self::Empty(EmptyTransport::new())
    }
    fn is_available(&self) -> bool {
        match self {
            Self::Empty(transport) => transport.distance == 0,
            Self::Loaded(_) => false,
        }
    }
    fn is_loaded(&self) -> bool {
        match self {
            Self::Empty(_) => false,
            Self::Loaded(_) => true,
        }
    }
    fn move_forward(&mut self, maybe_available_container: Option<&'c mut Container>) {
        match self {
            Self::Empty(t) => {
                // going home, cannot load a container yet
                if t.distance > 0 {
                    t.distance -= 1;
                } else {
                    if let Some(container) = maybe_available_container {
                        // Todo: the container should probably be responsible for its destination / distance
                        *self = Transport::Loaded(t.clone().load(container, 5, Location::B));
                    }
                    // no more work \o/
                }
            }
            Self::Loaded(t) => {
                t.distance += 1;
                let (Container { location, .. }, target_distance, destination) = &mut t.load;

                // arriving at destination
                if t.distance == *target_distance {
                    location = Some(destination.clone());
                    *self = Self::Empty(EmptyTransport {
                        distance: *target_distance,
                    });
                };
            }
        }
    }
}

impl EmptyTransport {
    fn new() -> Self {
        Self { distance: 0 }
    }

    fn load<'c>(
        self,
        container: &'c mut Container,
        distance: u8,
        destination: Location,
    ) -> LoadedTransport<'c> {
        LoadedTransport {
            distance: self.distance,
            load: (container, distance, destination),
        }
    }
}

impl From<LoadedTransport<'_>> for EmptyTransport {
    fn from(loaded_transport: LoadedTransport<'_>) -> Self {
        Self {
            distance: loaded_transport.distance,
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
        let mut EmptyTransport = EmptyTransport::new();
        EmptyTransport.move_forward();
        assert_eq!(false, EmptyTransport.is_loaded());
        assert_eq!(0, EmptyTransport.distance);
    }

    #[test]
    fn should_move_if_transport_loaded() {
        let mut EmptyTransport = EmptyTransport::new();
        EmptyTransport.load(Container::new(B), 2, Location::B);
        EmptyTransport.move_forward();
        assert_eq!(true, EmptyTransport.is_loaded());
        assert_eq!(1, EmptyTransport.distance);
    }

    #[test]
    fn should_make_transport_reach_destination_and_unload() {
        let mut EmptyTransport = EmptyTransport::new();
        EmptyTransport.load(Container::new(B), 2, Location::B);
        (0..2).for_each(|_| EmptyTransport.move_forward());
        assert_eq!(2, EmptyTransport.distance);
        assert_eq!(false, EmptyTransport.is_loaded());
    }

    #[test]
    fn should_make_transport_return_to_destination() {
        let mut EmptyTransport = EmptyTransport::new();
        EmptyTransport.load(Container::new(B), 2, Location::B);
        (0..4).for_each(|_| EmptyTransport.move_forward());
        assert_eq!(0, EmptyTransport.distance);
        assert_eq!(false, EmptyTransport.is_loaded());
    }
}
