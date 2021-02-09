use std::collections::BTreeMap;

use crate::Event;

struct BookingSystem {}

impl BookingSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_events(&self, events: Vec<Event>) -> bool {
        let mut calendar = BTreeMap::new();

        for event in events {
            let mut prev_events_iter = calendar.range(..event.start);
            let mut next_events_iter = calendar.range(event.start..);

            // check overlap
            if let Some((_, end)) = prev_events_iter.next_back() {
                if end > &event.start {
                    return true;
                }
            }

            if let Some((start_next, _)) = next_events_iter.next() {
                if &event.end > start_next {
                    return true;
                }
            }

            calendar.insert(event.start, event.end);
        }

        false
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
