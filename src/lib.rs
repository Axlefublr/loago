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

    pub fn output(self) -> OutputTasks {
        let now = now();
        let mut output: Vec<KeyToDiff> = self
            .0
            .into_iter()
            .map(|(key, timestamp)| (key, (now - timestamp).num_days()))
            .collect();
        output.sort_by_key(|(_, diff_days)| *diff_days);
        OutputTasks(output)
    }
}

fn now() -> NaiveDateTime { Utc::now().naive_utc() }

type KeyToDiff = (String, i64);

pub struct OutputTasks(Vec<KeyToDiff>);

#[cfg(test)]
mod tasks {
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use chrono::NaiveDateTime;

    use crate::now;
    use crate::Tasks;

    impl Tasks {
        fn dummy() -> Self {
            let mut map = HashMap::new();
            let december = december();
            map.insert(String::from("dust"), december);
            map.insert(String::from("vacuum"), december);
            map.insert(String::from("exercise"), december);
            Self(map)
        }
    }

    fn december() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2023, 12, 20)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    fn november() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2023, 11, 20)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
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
        let diff = now() - test_tasks.0["vacuum"];
        assert_eq!(diff.num_minutes(), 0);
        let diff = now() - test_tasks.0["dust"];
        assert_eq!(diff.num_minutes(), 0);
        let diff = now() - test_tasks.0["exercise"];
        assert!(diff.num_days() > 0);
    }

    #[test]
    fn output() {
        let test_tasks = Tasks::dummy().output();
        let expected_diff = now() - december();
        for (_, actual_diff) in test_tasks.0 {
            assert_eq!(actual_diff, expected_diff.num_days());
        }
    }
}
