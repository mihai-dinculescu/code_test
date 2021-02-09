use crate::Event;
struct BookingSystem {}

impl BookingSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_events(&self, mut events: Vec<Event>) -> bool {
        // with the events sorted by the start time
        // we can iterate through each and ensure that
        // start time is greater or equal to the previous end time

        // the big question if the array is pre-sorted by the start time or not
        // if the array doesn't need sorting, this solution will definitely be quicker than the btree one

        // assume the worst: the events are not sorted
        events.sort_by(|a, b| a.start.cmp(&b.start));

        // e.g. 10, 20 | 15, 30 | 20, 25
        let check = events.iter().try_fold(None, |prev_end, event| {
            if let Some(prev_end_value) = prev_end {
                if prev_end_value > event.start {
                    return Err(Some(prev_end_value));
                }
            }

            Ok(Some(event.end))
        });

        match check {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_success() {
        let system = BookingSystem::new();

        let events = vec![Event::new(10, 15), Event::new(20, 25)];

        assert_eq!(system.check_events(events), false);
    }

    #[bench]
    fn test_success_bench(b: &mut Bencher) {
        b.iter(|| {
            let system = BookingSystem::new();

            let events = vec![Event::new(10, 15), Event::new(20, 25)];

            assert_eq!(system.check_events(events), false);
        });
    }

    #[test]
    fn test_fail() {
        let system = BookingSystem::new();

        let events = vec![Event::new(10, 15), Event::new(20, 25), Event::new(15, 30)];

        assert_eq!(system.check_events(events), true);
    }

    #[bench]
    fn test_fail_bench(b: &mut Bencher) {
        b.iter(|| {
            let system = BookingSystem::new();

            let events = vec![Event::new(10, 15), Event::new(20, 25), Event::new(15, 30)];

            assert_eq!(system.check_events(events), true);
        });
    }

    #[test]
    fn test_edge() {
        let system = BookingSystem::new();

        let events = vec![Event::new(10, 15), Event::new(20, 25), Event::new(15, 20)];

        assert_eq!(system.check_events(events), false);
    }

    #[bench]
    fn test_edge_iter(b: &mut Bencher) {
        b.iter(|| {
            let system = BookingSystem::new();

            let events = vec![Event::new(10, 15), Event::new(20, 25), Event::new(15, 20)];

            assert_eq!(system.check_events(events), false);
        });
    }

    #[test]
    fn test_same_start() {
        let system = BookingSystem::new();

        let events = vec![Event::new(10, 15), Event::new(10, 15)];

        assert_eq!(system.check_events(events), true);
    }

    #[bench]
    fn test_same_start_bench(b: &mut Bencher) {
        b.iter(|| {
            let system = BookingSystem::new();

            let events = vec![Event::new(10, 15), Event::new(10, 15)];

            assert_eq!(system.check_events(events), true);
        });
    }
}
