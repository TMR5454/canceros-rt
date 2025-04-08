#![no_std]
#![no_main]

use canceros_rt::core::System;
use canceros_rt::task::Task;

use panic_halt as _;
use cortex_m_rt::entry;

fn subtask() {
    // Subtask code here
    loop {
        // Do something
    }
}

#[entry]
fn main() -> ! {
    // Initialize the system
    let mut system = System::new();

    let task1 = Task::new(subtask, "Task 1", 1, 1024);

    // Add the task to the system
    system.add_task(task1);

    // Start the scheduler
    system.start();

    loop {}
}
