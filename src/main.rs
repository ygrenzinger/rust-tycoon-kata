fn main() {
    println!("Hello, world!");
}

enum Destination {
    B
}

fn compute_time_to_deliver(destination: Vec<Destination>) -> i32 {
    5
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_simplest_case() {

        let hours_to_deliver: i32 = compute_time_to_deliver(vec![Destination::B]);
        assert_eq!(5, hours_to_deliver);
    }

    #[test]
    fn should_compute_time_with_2_destinations() {

        let hours_to_deliver: i32 = compute_time_to_deliver(vec![Destination::B, Destination::B]);
        assert_eq!(5, hours_to_deliver);
    }
}