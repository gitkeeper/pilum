# Test Command: Help

`pilum help` displays the application's purpose, its available commands and options:

```console
$ pilum help
Pilum is a sophisticated task manager with a CLI and a GUI written in Rust.

Pilum serves as a convenient and easy-to-use task management tool, operated via the command line and a graphical interface. It keeps track of your to-do tasks, enabling operations like adding, removing and altering tasks as per your requirements. Pilum is equipped with a wide range of commands for sophisticated task manipulations.

Essentially, Pilum functions as a list organizer. You can feed details, along with their respective parameters, and the program neatly structures and displays it. By integrating deadlines and recurring tasks, it becomes a comprehensive to-do manager. Further refinement is achieved by incorporating elements like priorities, tags, project groups and more, making Pilum a fully-fledged task organization program.

Usage: pilum [COMMAND]

Commands:
  active     Shows active tasks
  add        Adds a new pending task to the task list
  all        Shows all tasks
  append     Appends text to an existing task description
  completed  Shows completed tasks
  delete     Deletes the specified task
  done       Marks the specified task as completed
  duplicate  Duplicates the specified tasks
  list       Shows most details of tasks
  modify     Modifies the existing task with provided arguments
  prepend    Prepends text to an existing task description
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

`pilum --help` displays the same information as `pilum help`:

```console
$ pilum --help
Pilum is a sophisticated task manager with a CLI and a GUI written in Rust.

Pilum serves as a convenient and easy-to-use task management tool, operated via the command line and a graphical interface. It keeps track of your to-do tasks, enabling operations like adding, removing and altering tasks as per your requirements. Pilum is equipped with a wide range of commands for sophisticated task manipulations.

Essentially, Pilum functions as a list organizer. You can feed details, along with their respective parameters, and the program neatly structures and displays it. By integrating deadlines and recurring tasks, it becomes a comprehensive to-do manager. Further refinement is achieved by incorporating elements like priorities, tags, project groups and more, making Pilum a fully-fledged task organization program.

Usage: pilum [COMMAND]

Commands:
  active     Shows active tasks
  add        Adds a new pending task to the task list
  all        Shows all tasks
  append     Appends text to an existing task description
  completed  Shows completed tasks
  delete     Deletes the specified task
  done       Marks the specified task as completed
  duplicate  Duplicates the specified tasks
  list       Shows most details of tasks
  modify     Modifies the existing task with provided arguments
  prepend    Prepends text to an existing task description
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

`pilum -h` displays an abbreviated variant of the application's purpose:

```console
$ pilum -h
Pilum is a sophisticated task manager with a CLI and a GUI written in Rust.

Usage: pilum [COMMAND]

Commands:
  active     Shows active tasks
  add        Adds a new pending task to the task list
  all        Shows all tasks
  append     Appends text to an existing task description
  completed  Shows completed tasks
  delete     Deletes the specified task
  done       Marks the specified task as completed
  duplicate  Duplicates the specified tasks
  list       Shows most details of tasks
  modify     Modifies the existing task with provided arguments
  prepend    Prepends text to an existing task description
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

```
