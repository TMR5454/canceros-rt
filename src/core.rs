use crate::define;
use crate::task::Task;

pub struct System {
    pub tasks: [Option<Task>; define::MAX_TASKS],
    pub current_task: usize,
}

impl System {
    pub fn new() -> Self {
        System {
            tasks: [None; define::MAX_TASKS],
            current_task: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        // Add a task to the system
        for i in 0..define::MAX_TASKS {
            if self.tasks[i].is_none() {
                self.tasks[i] = Some(task);
                return;
            }
        }
        panic!("No more tasks can be added");
    }

    pub fn start(&mut self) {
        // Start the scheduler
        loop {
            if let Some(ref mut task) = self.tasks[self.current_task] {
                if task.state.is_ready() {
                    task.run();
                }
            }
            self.current_task = (self.current_task + 1) % define::MAX_TASKS;
        }
    }
}
