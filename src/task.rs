#[derive(Copy, Clone)]
pub struct Task {
    pub func: fn(),
    pub name: &'static str,
    pub priority: usize,
    pub stack_size: usize,
    pub state: TaskState,
}

impl Task {
    pub fn new(func: fn(), name: &'static str, priority: usize, stack_size: usize) -> Self {
        Task {
            func,
            name,
            priority,
            stack_size,
            state: TaskState::Ready,
        }
    }

    pub fn run(&mut self) {
        self.state = TaskState::Running;
        // Simulate task execution
        // In a real RTOS, this would involve context switching and running the task code
        (self.func)();
        self.state = TaskState::Blocked; // Simulate task blocking after execution
    }

    pub fn suspend(&mut self) {
        self.state = TaskState::Suspended;
    }

    pub fn resume(&mut self) {
        self.state = TaskState::Ready;
    }
}

#[derive(Copy, Clone)]
pub enum TaskState {
    Ready,
    Running,
    Suspended,
    Blocked,
}

impl TaskState {
    pub fn is_ready(&self) -> bool {
        matches!(self, TaskState::Ready)
    }

    pub fn is_running(&self) -> bool {
        matches!(self, TaskState::Running)
    }

    pub fn is_suspended(&self) -> bool {
        matches!(self, TaskState::Suspended)
    }

    pub fn is_blocked(&self) -> bool {
        matches!(self, TaskState::Blocked)
    }
}
