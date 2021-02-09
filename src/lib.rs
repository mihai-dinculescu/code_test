#![feature(test)]
extern crate test;

mod array_solution;
mod btreemap_solution;

#[derive(Debug)]
struct Event {
    start: i32,
    end: i32,
}

impl Event {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}
