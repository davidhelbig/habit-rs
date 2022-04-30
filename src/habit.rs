use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Habits {
    habits: Vec<Habit>,
}

impl Habits {
    pub fn new() -> Self {
        Habits { habits: vec![] }
    }

    pub fn add(&mut self, habit: Habit) {
        self.habits.push(habit)
    }

    pub fn return_mut_by_name(&mut self, name: &str) -> Option<&mut Habit> {
        self.habits.iter_mut().find(|h| h.name == name)
    }

    pub fn from_json(serialized: &str) -> Self {
        serde_json::from_str(serialized).expect("Could not perform deserialization.")
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("Could not serialize to json.")
    }
}

/// Struct that represents a single habit to be tracked.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Habit {
    name: String,
    start_date: NaiveDate,
    completed_on: BTreeSet<NaiveDate>,
}

impl Habit {
    pub fn new(name: String, start_date: NaiveDate) -> Self {
        Habit {
            name,
            start_date,
            completed_on: BTreeSet::new(),
        }
    }

    /// A convenience function that creates a new habit and sets the `start_date`
    /// to today's date.
    pub fn start_today(name: String) -> Self {
        Habit {
            name,
            start_date: Local::now().naive_local().date(),
            completed_on: BTreeSet::new(),
        }
    }

    pub fn add_completed_day(&mut self, date: NaiveDate) {
        if date < self.start_date {
            panic!("Completed date cannot be before start date!");
        }
        self.completed_on.insert(date);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Could not serialize to json.")
    }

    pub fn from_json(serialized: &str) -> Self {
        serde_json::from_str(serialized).expect("Deserialization failed.")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn builder_can_create_new_habit() {
        let start_date = NaiveDate::from_ymd(2022, 10, 31);

        let habit = Habit {
            name: String::from("Reading"),
            start_date,
            completed_on: BTreeSet::new(),
        };

        assert_eq!(Habit::new(String::from("Reading"), start_date), habit);
    }

    #[test]
    fn can_add_completed_day() {
        let mut habit = Habit::new(String::from("Reading"), NaiveDate::from_ymd(2022, 10, 31));

        habit.add_completed_day(NaiveDate::from_ymd(2022, 10, 31));
    }

    #[test]
    #[should_panic]
    fn panics_if_completed_day_is_before_started_day() {
        let mut habit = Habit::new(String::from("Reading"), NaiveDate::from_ymd(2022, 10, 31));

        habit.add_completed_day(NaiveDate::from_ymd(2022, 10, 30));
    }

    #[test]
    fn can_serialize_and_deserialize_json() {
        let mut habit = Habit::new(String::from("Reading"), NaiveDate::from_ymd(2022, 10, 31));
        habit.add_completed_day(NaiveDate::from_ymd(2022, 10, 31));

        let serialized = habit.to_json();

        let deserialized = Habit::from_json(&serialized);

        assert_eq!(habit, deserialized);
    }

    fn can_return_by_name() {
        let reading = Habit::new(String::from("Reading"), NaiveDate::from_ymd(2022, 10, 31));
        let meditation = Habit::new(String::from("Meditation"), NaiveDate::from_ymd(2022, 9, 30));
        let mut habits = Habits::new();
        habits.add(reading);
        habits.add(meditation);

        let returned = habits
            .return_mut_by_name(&String::from("Meditation"))
            .unwrap();

        // assert_eq!(meditation, *returned);
    }
}
