# Loago

Fork of @Axlefublr/loago.

> Check how LOng AGO you did any task!

I don't always have the energy to do tasks around the house, so I needed a program to shame me into doing it.

You can use `loago` precisely for that!

Let's say you vacuumed today. Let's track that:

```
loago do vacuum
```

You will see:
```
Added work item vacuum at time 06/10/2025 01:55:40.
```

Now when you execute `loago view`

You will see:

```
vacuum last done 0 days, 0 hours, 2 minutes, 1 seconds ago.
```

As time passes, the time will increase. So if you wait 14 days and execute `loago view`, you will get:

```
vacuum last done 14 days, 0 hours, 0 minutes, 0 seconds ago.
```

Probably time to vacuum again! Execute `loago do vacuum` to update the task and reset its amount of days to 0 again.

```
Updated work item vacuum at time 06/24/2025 01:58:17.
```


`loago view` automatically displays all of the tasks, sorting them by their (ascending) amount of days. But you can instead specify only the tasks that you want to see:

`loago view floor bed keyboard`
```
keyboard last done 0 days, 0 hours, 0 minutes, 9 seconds ago.
floor last done 0 days, 0 hours, 0 minutes, 11 seconds ago.
bed last done 0 days, 0 hours, 0 minutes, 12 seconds ago.
```

Want to remove some task(s)? Use `loago remove`

In both `view` and `remove`, you can specify one or many task names at once.

You cannot do multiple things at the same time!

Don't like the names of the subcommands? Or you forgot the name? There are a few aliases for them you can look up in `loago --help` (or the [the next section](##Usage)), which is automatically shown if you make a mistake!

## Usage

```
Loago - How long ago program
Licensed under the MIT license.

  do, add, new, update, reset    Update last time of doing task.
  view, list, look, see          Check last time tasks have been done. Optionally provide list of tasks
  remove, delete                 Remove task or list of tasks from database
  help, --help, anything else    Show this help screen.
```

## Install

See the compile command for native AOT compilation.

## Uninstall

The program is too perfect to uninstall. If you really want to though:

Delete the executable.

```
rm -fr ~/.config/loago
```

