fn main() {
    println!("Hello, world!");
}

enum Destination {
    B,
}

fn compute_time_to_deliver(destination: Vec<Destination>) -> usize {
    if destination.is_empty() {
        0
    } else {
        let size = destination.len();
        ((size - 1) / 2) * 10 + 5
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Destination::{self, *};
    use rstest::rstest;

    #[rstest]
    #[case(vec![], 0)]
    #[case(vec![B], 5)]
    #[case(vec![B, B], 5)]
    #[case(vec![B, B, B], 15)]
    #[case(vec![B, B, B, B], 15)]
    #[case(vec![B, B, B, B, B], 25)]
    #[case(vec![B, B, B, B, B, B, B], 35)]
    fn should_compute_time_with_2_trucks(#[case] dest: Vec<Destination>, #[case] expect: usize) {
        let hours_to_deliver = compute_time_to_deliver(dest);
        assert_eq!(expect, hours_to_deliver);
    }
}
