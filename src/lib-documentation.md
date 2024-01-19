This library is an abstraction to handle a HashMap of tasks and when you
last did them.

# Examples

```
use std::collections::HashMap;
use loago::OutputTasks;
use loago::Tasks;

let mut tasks = Tasks::from(HashMap::new());
tasks.update("vacuum");
tasks.update("dust");
let output: OutputTasks = tasks
    .output(|duration: chrono::Duration| duration.num_weeks().to_string());
let displayed = output.to_string();
assert_eq!(displayed, String::from("dust   — 0\nvacuum — 0\n"));
```