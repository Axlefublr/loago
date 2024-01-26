# Loago

> Check how LOng AGO you did any task!

I don't always have the energy to do tasks around the house, so I needed a program to shame me into doing it.

You can use `loago` precisely for that!

Let's say you vacuumed today. Let's track that:

```
loago do vacuum
```

Now when you execute `loago view`

You will see:

```
vacuum — 0
```

In other words, you did the task "vacuum" 0 days ago. As days come by, that number will increase. So if you wait 14 days and execute `loago view`, you will get:

```
vacuum — 14
```

Probably time to vacuum again! Execute `loago do vacuum` to update the task and reset its amount of days to 0 again.

`loago view` automatically displays all of the tasks, sorting them by their (ascending) amount of days. But you can instead specify only the tasks that you want to see:

`loago view floor bed keyboard`
```
bed      — 4
floor    — 6
keyboard — 8
```

Want to remove some task(s)? Use `loago remove`

In both `do` and `remove`, you can specify one or many task names at once!

Don't like the names of the subcommands? There are a few aliases for them you can look up in `loago --help` (or the [the next section](##Usage))!

## Usage

```
Track how long ago you last did a task

Usage: loago <COMMAND>

Commands:
  do      [aliases: add, new, update, reset]
          Update tasks' dates to now. Creates tasks that didn't
          exist before

  view    [aliases: list, look, see]
          View all (default) or specified tasks, with how many days
          (and optionally, hours and minutes) ago you last did them

  remove  [aliases: delete]
          Remove specified tasks from the list

  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Install

```
cargo install loago
```

`cargo-quickinstall` and `cargo-binstall` are also supported.

## Uninstall

```
cargo uninstall loago
```