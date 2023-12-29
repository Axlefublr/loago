use std::collections::HashMap;

use chrono::NaiveDateTime;
use chrono::Utc;

pub const APP_NAME: &str = "loago";

pub struct Tasks(HashMap<String, NaiveDateTime>);

impl From<HashMap<String, NaiveDateTime>> for Tasks {
    fn from(value: HashMap<String, NaiveDateTime>) -> Self { Self(value) }
}

impl Tasks {
    pub fn update(&mut self, task: String) { self.0.insert(task, now()); }

    pub fn update_multiple(&mut self, tasks: impl IntoIterator<Item = String>) {
        let now = now();
        for task in tasks.into_iter() {
            self.0.insert(task, now);
        }
    }

    pub fn remove(&mut self, task: &str) { self.0.remove(task); }

    pub fn remove_multiple(&mut self, tasks: &[&str]) {
        let now = now();
        for task in tasks {
            self.0.remove(*task);
        }
    }

    pub fn keep(&mut self, task: String) {
        let mut new_map = HashMap::new();
        if let Some(timestamp) = self.0.remove(&task) {
            new_map.insert(task, timestamp);
        }
        self.0 = new_map;
    }

    pub fn keep_multiple<T: ToString>(&mut self, tasks: impl IntoIterator<Item = T>) {
        let mut new_map = HashMap::new();
        for task in tasks {
            if let Some(timestamp) = self.0.remove(&task.to_string()) {
                new_map.insert(task.to_string(), timestamp);
            }
        }
        self.0 = new_map;
    }
}

fn now() -> NaiveDateTime { Utc::now().naive_utc() }

#[cfg(test)]
mod tasks {
    use std::collections::HashMap;

    use chrono::NaiveDateTime;

    use crate::now;
    use crate::Tasks;

    impl Tasks {
        fn dummy() -> Self {
            let mut map = HashMap::new();
            let the_seventies = NaiveDateTime::default();
            map.insert(String::from("dust"), the_seventies);
            map.insert(String::from("vacuum"), the_seventies);
            Self(map)
        }
    }

    #[test]
    fn update() {
        let mut test_tasks = Tasks::dummy();
        test_tasks.update(String::from("dust"));
        let diff = now() - test_tasks.0["dust"];
        assert_eq!(diff.num_minutes(), 0);
    }

    #[test]
    fn update_multiple() {
        let mut test_tasks = Tasks::dummy();
        test_tasks.update_multiple(vec![String::from("vacuum"), String::from("dust")]);
        for key in test_tasks.0.keys() {
            let diff = now() - test_tasks.0[key];
            assert_eq!(diff.num_minutes(), 0);
        }
    }
}
