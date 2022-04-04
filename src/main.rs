fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]
enum Destination {
    A,
    B,
}

fn compute_time_to_deliver(destination: Vec<Destination>) -> usize {
    if destination.is_empty() {
        0
    } else {
        let b_count = destination
            .iter()
            .filter(|dest| dest == &&Destination::B)
            .collect::<Vec<_>>()
            .len();
        let a_count = destination
            .iter()
            .filter(|dest| dest == &&Destination::A)
            .collect::<Vec<_>>()
            .len();

        if b_count > 0 {
            ((b_count - 1) / 2) * 10 + 5
        } else {
            (((a_count - 1) / 2) + 1) + (((a_count - 1)) * 8 + 4)
        }
    }
}

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
    #[case(vec![A, A], 13, "two truck 1h + one boat 4h, 1 + 4 + 4 + 4 = 13")]
    fn should_compute_time_with_2_trucks(
        #[case] dest: Vec<Destination>,
        #[case] expect: usize,
        #[case] msg: &str,
    ) {
        let hours_to_deliver = compute_time_to_deliver(dest);
        assert_eq!(expect, hours_to_deliver, "{msg}");
    }
}
