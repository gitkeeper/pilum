/// Task
///
/// The Task struct represents a single task with an id, title, and completed status.
///
struct Task {
    id: u64,
    title: String,
    completed: bool,
}

/// TaskList
///
/// The TaskList struct is a collection of Task instances. It has methods to add a new task, list
/// all tasks, and mark a task as completed.
///
pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    /// Create a new TaskList instance.
    ///
    /// # Examples
    /// ```
    /// let task_list = TaskList::new();
    /// ```
    ///
    pub fn new() -> TaskList {
        TaskList { tasks: Vec::new() }
    }

    /// Add a new task to the task list.
    ///
    /// # Examples
    /// ```
    /// let mut task_list = TaskList::new();
    ///
    /// task_list.add_task("Buy groceries.".to_string());
    /// ```
    ///
    pub fn add_task(&mut self, title: String) {
        let id = self.tasks.len() as u64 + 1;
        let task = Task {
            id,
            title: title.clone(),
            completed: false,
        };
        self.tasks.push(task);
        println!("Added task: {}", title)
    }

    /// List all tasks in the task list.
    ///
    /// If there are no tasks, a message is displayed.
    ///
    /// # Examples
    /// ```
    /// let mut task_list = TaskList::new();
    ///
    /// task_list.add_task("Buy groceries.".to_string());
    ///
    /// task_list.list_tasks();
    /// ```
    ///
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for task in &self.tasks {
                let status = if task.completed { "[X]" } else { "[ ]" };
                println!("{} {} – {}", status, task.id, task.title);
            }
        }
    }

    /// Mark a task as completed.
    ///
    /// If the task is not found, a message is displayed.
    ///
    /// # Examples
    /// ```
    /// let mut task_list = TaskList::new();
    ///
    /// task_list.add_task("Buy groceries.".to_string());
    ///
    /// task_list.complete_task(1);
    /// ```
    ///
    pub fn complete_task(&mut self, id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|i| i.id == id) {
            task.completed = true;
            println!("Completed: {}", task.title)
        } else {
            println!("Task with ID {} not found.", id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_task_list() -> TaskList {
        let mut task_list = TaskList::new();
        task_list.add_task("Buy groceries.".to_string());
        task_list.add_task("Walk the dog.".to_string());
        task_list.add_task("Do the laundry.".to_string());
        task_list
    }

    #[test]
    fn test_add_task() {
        let mut task_list = setup_task_list();

        task_list.add_task("Wash the dishes.".to_string());

        assert_eq!(task_list.tasks.len(), 4);
    }

    #[test]
    fn test_list_tasks() {
        let task_list = setup_task_list();

        task_list.list_tasks();
    }

    #[test]
    fn test_complete_task() {
        let mut task_list = setup_task_list();

        task_list.complete_task(2);

        assert!(task_list.tasks[1].completed);
    }
}
