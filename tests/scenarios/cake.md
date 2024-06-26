# Use Case: Cake

`pilum all` has no tasks to show initially:

```console
$ pilum all
No tasks found.

```

`pilum completed` has no tasks to show initially:

```console
$ pilum completed
No tasks found.

```

`pilum add "Buy milk"` adds a new pending task to the task list:

```console
$ pilum add "Buy flour"
Pending task 1 'Buy flour'.

```

`pilum add "Buy eggs"` after `pilum add "Buy milk` adds a second pending task to the task list:

```console
$ pilum add "Buy milk"
Pending task 2 'Buy milk'.

$ pilum add "Buy eggs"
Pending task 3 'Buy eggs'.

```

`pilum add "Buy sugar" "Bake cake"` adds two pending tasks to the task list:

```console
$ pilum add "Buy sugar" "Bake cake"
Pending task 4 'Buy sugar'.
Pending task 5 'Bake cake'.

```

`pilum all` shows all tasks.

```console
$ pilum all
1 'Buy flour'
2 'Buy milk'
3 'Buy eggs'
4 'Buy sugar'
5 'Bake cake'

```

`pilum done 1` marks the task number 1 as completed:

```console
$ pilum done 1
Completed task 1 'Buy flour'.
Completed 1 task.

```

`pilum done 1` marks the task number 2 and 3 as completed:

```console
$ pilum done 2 3
Completed task 2 'Buy milk'.
Completed task 3 'Buy eggs'.
Completed 2 tasks.

```

`pilum completed` marks the task number 2 and 3 as completed:

```console
$ pilum completed
1 'Buy flour'
2 'Buy milk'
3 'Buy eggs'

```

`pilum start` starts the given tasks:

```console
$ pilum start 4 5
Started task 4 'Buy sugar'.
Started task 5 'Bake cake'.
Started 2 tasks.

```

`pilum active` shows all started tasks:

```console
$ pilum active
4 'Buy sugar'
5 'Bake cake'

```
