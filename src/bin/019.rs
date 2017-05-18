// How many sundays fell on first of the month from 1 Jan 1901 to 31 Dec 2000?
// This is a perhaps overkill way to structure it. It's less math and more
// programming, but should still be relatively efficient using iterators.

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MONTH_OFFSET_LEAP: Vec<usize> =
        vec![0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30] //, 31]
            .into_iter()
            .scan(1, |state, x| {
                *state = *state + x;
                Some(*state)
            }).collect();

    static ref MONTH_OFFSET: Vec<usize> =
        vec![0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30]//, 31]
            .into_iter()
            .scan(1, |state, x| {
                *state = *state + x;
                Some(*state)
            }).collect();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Clocker {
    year: usize,
    leap_year: bool,
    day_count: usize,
    day: Day,
}

impl Clocker {
    pub fn new(year: usize, day_count: usize, day: Day) -> Self {
        Clocker {
            year: year,
            leap_year: Self::is_leap_year(year),
            day_count: day_count,
            day: day,
        }
    }

    fn is_leap_year(year: usize) -> bool {
        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0 {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn next_day(&mut self) {
        use Day::*;
        match self.day {
            Mon => self.day = Tue,
            Tue => self.day = Wed,
            Wed => self.day = Thu,
            Thu => self.day = Fri,
            Fri => self.day = Sat,
            Sat => self.day = Sun,
            Sun => self.day = Mon,
        }
    }
}

impl Iterator for Clocker {
    type Item = Option<Output>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.day_count < 365 || (self.day_count == 365 && self.leap_year) {
            self.day_count += 1;
            self.next_day();
        } else if self.day_count == 365 && !self.leap_year || (self.day_count == 366 && self.leap_year) {
            self.day_count = 1;
            self.year += 1;
            self.leap_year = Self::is_leap_year(self.year);
            self.next_day();
        }

        if self.leap_year && MONTH_OFFSET_LEAP.contains(&self.day_count) {
            Some(Some(Output {
                day: self.day.clone(),
                year: self.year.clone(),
            }))
        } else if !self.leap_year && MONTH_OFFSET.contains(&self.day_count) {
            Some(Some(Output {
                day: self.day.clone(),
                year: self.year.clone(),
            }))
        } else {
            Some(None)
        }

    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Output {
    day: Day,
    year: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Day {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

fn main() {
    // Monday is day 1, 1 Jan 1900.
    let counter = Clocker::new(
        1899,
        365,
        Day::Sun,
    );

    // First, offset to first day of epoch.
    let sundays_first_of_month = counter
        .skip(365)
        .take_while(|ref output| {
            if let &&Some(ref output) = output {
                output.year < 2001
            } else { true }
        })
        .filter_map(|output| output.map(|o| o.day))
        .filter(|ref day| **day == Day::Sun)
        .count();

    println!("Number of sundays on first day of month in epoch: {}",
             sundays_first_of_month);
}
