use std::collections::HashMap;
use std::fmt;

use chrono::NaiveDateTime;
use chrono::Utc;

pub const APP_NAME: &str = "loago";

pub struct Tasks(HashMap<String, NaiveDateTime>);

impl From<HashMap<String, NaiveDateTime>> for Tasks {
    fn from(value: HashMap<String, NaiveDateTime>) -> Self {
        Self(value)
    }
}

impl Tasks {
    pub fn update(&mut self, task: impl Into<String>) {
        self.0.insert(task.into(), now());
    }

    pub fn update_multiple(&mut self, tasks: impl IntoIterator<Item = impl Into<String>>) {
        let now = now();
        for task in tasks.into_iter() {
            self.0.insert(task.into(), now);
        }
    }

    pub fn output(self) -> OutputTasks {
        self.output_when(now())
    }

    fn output_when(self, now: NaiveDateTime) -> OutputTasks {
        let mut output: Vec<KeyToDiff> = self
            .0
            .into_iter()
            .map(|(key, timestamp)| (key, (now - timestamp).num_days()))
            .collect();
        output.sort_by_key(|(_, diff_days)| *diff_days);
        OutputTasks(output)
    }
}

fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

type KeyToDiff = (String, i64);

pub struct OutputTasks(Vec<KeyToDiff>);

impl fmt::Display for OutputTasks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut length = 0;
        self.0.iter().for_each(|(task_name, _)| {
            let task_name_len = task_name.len();
            if task_name_len > length {
                length = task_name_len;
            }
        });
        let mut buffer = String::new();
        for (key, days_diff) in self.0.iter() {
            let whitespace = " ".repeat(length - key.len());
            buffer.push_str(key);
            buffer.push_str(&whitespace);
            buffer.push(' ');
            buffer.push('—');
            buffer.push(' ');
            buffer.push_str(&days_diff.to_string());
            buffer.push('\n')
        }
        write!(f, "{}", buffer)
    }
}

#[cfg(test)]
mod tasks {
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use chrono::NaiveDateTime;

    use crate::now;
    use crate::Tasks;

    impl Tasks {
        fn same_days() -> Self {
            let mut map = HashMap::new();
            let december = december();
            map.insert(String::from("dust"), december);
            map.insert(String::from("vacuum"), december);
            map.insert(String::from("exercise"), december);
            Self(map)
        }

        fn different_days() -> Self {
            let mut map = HashMap::new();
            map.insert(String::from("dust"), november(1));
            map.insert(String::from("vacuum"), november(2));
            map.insert(String::from("exercise"), november(3));
            Self(map)
        }
    }

    fn december() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2023, 12, 20)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    fn november(day: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2023, day, 20)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
    }

    #[test]
    fn update() {
        let mut tasks = Tasks::same_days();
        tasks.update("dust");
        let dust_ago = now() - tasks.0["dust"];
        assert_eq!(dust_ago.num_minutes(), 0);
    }

    #[test]
    fn update_multiple() {
        let mut tasks = Tasks::same_days();
        tasks.update_multiple(["vacuum", "dust"]);
        let vacuum_ago = now() - tasks.0["vacuum"];
        let dust_ago = now() - tasks.0["dust"];
        let exercise_ago = now() - tasks.0["exercise"];
        assert_eq!(vacuum_ago.num_minutes(), 0);
        assert_eq!(dust_ago.num_minutes(), 0);
        assert!(exercise_ago.num_days() > 0);
    }

    #[test]
    fn output() {
        let tasks = Tasks::same_days().output();
        let expected_diff = now() - december();
        for (_, actual_diff) in tasks.0 {
            assert_eq!(actual_diff, expected_diff.num_days());
        }
    }

    #[test]
    fn output_display() {
        let tasks = Tasks::different_days().output_when(december());
        let expected = String::from("exercise — 275\nvacuum   — 303\ndust     — 334\n");
        let actual = tasks.to_string();
        assert_eq!(expected, actual);
    }
}
