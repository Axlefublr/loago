#![doc = include_str!("lib-documentation.md")]

use std::collections::HashMap;
use std::fmt;

use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Utc;

/// A wrapper over a `HashMap<String, NaiveDateTime>` ([`NaiveDateTime`]).
///
/// Meant to be used for updating, removing and filtering tasks.
///
/// Then, once you're done changing your data, use one of the `output` methods
/// on [`Tasks`] to convert it to [`OutputTasks`], which you can use to display
/// the tasks information to the user.
///
/// For any data manipulation not implemented for [`Tasks`], feel free to
/// manipulate the `HashMap` directly beforehand.
pub struct Tasks(HashMap<String, NaiveDateTime>);

/// This is only useful if you can conveniently create a [`HashMap<String,
/// NaiveDateTime>`]. The library is made with the intention to be used with
/// some sort of data file that you can deserialize, and deserializing a
/// datetime `String` straight into a [`NaiveDateTime`] is not supported. So
/// this will mostly be useful if:
/// 1. You're testing something
/// 2. You don't don't get the data from a file and create it programmatically,
///    making you not have to deserialize string data and therefore allowing you
///    to create [`NaiveDateTime`]s straight up.
impl From<HashMap<String, NaiveDateTime>> for Tasks {
    fn from(value: HashMap<String, NaiveDateTime>) -> Self {
        Self(value)
    }
}

/// The reason for this existing is that deserializing
/// into `HashMap<String, String>` is supported by serde.
/// If we were to use [`NaiveDateTime`] immediately though, the only way we
/// could make it work is by creating a wrapper type to be able to implement the
/// specific `serde::Deserializer` traits on it.
/// Except that then *you* wouldn't be able to add `Deserializer`
/// implementations of your own, locking you into a limited set of
/// possibilities.
///
/// # Errors
/// Expects this format: `%Y-%m-%dT%H:%M:%S%.f`, as defined by
/// [`NaiveDateTime`]'s documentation on the `parse` method in the
/// [`std::str::FromStr`].
///
/// So, the only error is that parse failing.
///
/// # Examples
/// More helpfully, that format is automatically used when you format a
/// [`NaiveDateTime`] by using its [`fmt::Debug`] implementation.
/// ```
/// use std::collections::HashMap;
///
/// use loago::Tasks;
/// let now = chrono::Utc::now().naive_utc();
/// let timestamp = format!("{:?}", now);
/// let mut map = HashMap::new();
/// map.insert(String::from("task-name"), timestamp);
/// let tasks: Tasks = Tasks::try_from(map).unwrap();
/// ```
impl TryFrom<HashMap<String, String>> for Tasks {
    type Error = chrono::format::ParseError;

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut map = HashMap::new();
        for (key, timestamp) in value {
            let timestamp = timestamp.parse()?;
            map.insert(key, timestamp);
        }
        Ok(Tasks(map))
    }
}

/// This `From` is useful to convert the data back into a serializable data
/// structure, for you to then write back to the data file.
///
/// The value `String` in the `HashMap` uses the `%Y-%m-%dT%H:%M:%S%.f` format.
///
/// That format is expected by `TryFrom<HashMap<String, String>> for Tasks`, so
/// this `From` comes hand-in-hand with it in terms of making the full binary
/// application: getting data from a file, mutating it, and then writing the new
/// data to the file.
impl From<Tasks> for HashMap<String, String> {
    fn from(value: Tasks) -> Self {
        value
            .0
            .into_iter()
            .map(|(key, timestamp)| (key, format!("{:?}", timestamp)))
            .collect()
    }
}

impl Tasks {
    /// Update a task's [`NaiveDateTime`] timestamp to that of right [`now`].
    /// If the given task didn't exist prior, it will be created.
    pub fn update(&mut self, task: impl Into<String>) {
        self.0.insert(task.into(), now());
    }

    /// Update multiple tasks' [`NaiveDateTime`] timestamps to that of right
    /// [`now`]. If any of the given tasks didn't exist prior, they will be
    /// created.
    pub fn update_multiple(
        &mut self,
        tasks: impl IntoIterator<Item = impl Into<String>>,
    ) {
        let now = now();
        for task in tasks {
            self.0.insert(task.into(), now);
        }
    }

    /// Remove a task from the list.
    pub fn remove(&mut self, task: &str) {
        self.0.remove(task);
    }

    /// Remove multiple tasks from the list.
    pub fn remove_multiple(&mut self, tasks: &[impl AsRef<str>]) {
        for task in tasks {
            self.0.remove(task.as_ref());
        }
    }

    /// Only keep the specified task in the list, removing all the other ones.
    pub fn keep(&mut self, task: impl Into<String>) {
        let task = task.into();
        let mut map = HashMap::new();
        if self.0.contains_key(&task) {
            let timestamp = self.0[&task];
            map.insert(task, timestamp);
        };
        self.0 = map;
    }

    /// Only keep the specified tasks in the list, removing all the other ones.
    pub fn keep_multiple(
        &mut self,
        tasks: impl IntoIterator<Item = impl Into<String>>,
    ) {
        let mut map = HashMap::new();
        for task in tasks {
            let task = task.into();
            if self.0.contains_key(&task) {
                let timestamp = self.0[&task];
                map.insert(task, timestamp);
            }
        }
        self.0 = map;
    }

    /// Convert this [`Tasks`] into a [`OutputTasks`], meant to be used for
    /// displaying the final data to the user.
    ///
    /// Assumes you're checking how long ago the tasks were done compared to
    /// [`now`].
    ///
    /// Displays the time difference of each task in days.
    pub fn output_days(self) -> OutputTasks {
        self.output(|duration| duration.num_days().to_string())
    }

    /// Convert this [`Tasks`] into a [`OutputTasks`], meant to be used for
    /// displaying the final data to the user.
    ///
    /// Assumes you're checking how long ago the tasks were done compared to
    /// [`now`].
    ///
    /// Convert the [`Duration`] into a `String`
    /// representation of your choosing, by mapping it with a closure.
    pub fn output<F>(self, to_string: F) -> OutputTasks
    where
        F: Fn(Duration) -> String,
    {
        self.output_when(now(), to_string)
    }

    /// Convert this [`Tasks`] into a [`OutputTasks`], meant to be used for
    /// displaying the final data to the user.
    ///
    /// Allows to pass the [`NaiveDateTime`] that will be considered
    /// "now".
    /// This "now" is what we compare each task's timestamp against to see how
    /// long ago it got done. Useful for testing and other applications I'm
    /// probably missing, which is why this is public.
    ///
    /// Convert the [`Duration`] into a `String`
    /// representation of your choosing, by mapping it with a closure.
    pub fn output_when<F>(self, now: NaiveDateTime, to_string: F) -> OutputTasks
    where
        F: Fn(Duration) -> String,
    {
        type KeyToDuration = (String, Duration);
        let mut output: Vec<KeyToDuration> = self
            .0
            .into_iter()
            .map(|(key, timestamp)| (key, now - timestamp))
            .collect();
        output.sort_by_key(|(_, diff_days)| *diff_days);
        let output: Vec<KeyToDisplay> = output
            .into_iter()
            .map(|(key, duration)| (key, to_string(duration)))
            .collect();
        OutputTasks(output)
    }
}

/// When the library says "now" in the documentation, this is what it means.
///
/// The implementation is literally just `chrono::Utc::now().naive_utc()`.
pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

type KeyToDisplay = (String, String);

/// Used exclusively for its [`fmt::Display`] implementation, which is what
/// you're supposed to use to display the final data to the user in a friendly
/// way.
pub struct OutputTasks(Vec<KeyToDisplay>);

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
    fn remove() {
        let mut tasks = Tasks::same_days();
        tasks.remove("vacuum");
        assert!(!tasks.0.contains_key("vacuum"))
    }

    #[test]
    fn remove_multiple() {
        let mut tasks = Tasks::same_days();
        tasks.remove_multiple(&["vacuum", "dust"]);
        assert!(!tasks.0.contains_key("vacuum"));
        assert!(!tasks.0.contains_key("dust"));
    }

    #[test]
    fn keep() {
        let mut tasks = Tasks::same_days();
        tasks.keep("vacuum");
        assert!(!tasks.0.contains_key("dust"));
        assert!(tasks.0.contains_key("vacuum"));
    }

    #[test]
    fn keep_multiple() {
        let mut tasks = Tasks::same_days();
        tasks.keep_multiple(["dust", "vacuum"]);
        assert!(!tasks.0.contains_key("exercise"));
        assert!(tasks.0.contains_key("dust"));
        assert!(tasks.0.contains_key("vacuum"));
    }

    #[test]
    fn output_days() {
        let tasks = Tasks::same_days().output_days();
        let expected = (now() - december()).num_days().to_string();
        for (_, actual) in tasks.0 {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn output_display() {
        let tasks = Tasks::different_days()
            .output_when(december(), |duration| {
                duration.num_days().to_string()
            });
        let expected =
            String::from("exercise — 275\nvacuum   — 303\ndust     — 334\n");
        let actual = tasks.to_string();
        assert_eq!(expected, actual);
    }
}
