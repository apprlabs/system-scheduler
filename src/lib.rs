use std::env;
use crate::linux::{schedule_task_linux, task_exists_linux, unschedule_task_linux};
use crate::macos::{schedule_task_macos, task_exists_macos, unschedule_task_macos};
use crate::windows::{schedule_task_windows, task_exists_windows, unschedule_task_windows};

mod helpers;
mod linux;
mod macos;
mod windows;

pub struct Task {
    pub name: String,
    pub command: String,
    pub user: String,
    pub schedule: String,
}

/// Checks whether a given task exists
///
/// # Examples
/// ```
/// use system_scheduler::task_exists;
/// use system_scheduler::Task;
///
/// let task_does_exist = task_exists(Task {
///   user: "troymccabe".to_string(),
///   command: "echo \"hello\"".to_string(),
///   name: "com.apple.assistant_service".to_string(),
///   schedule: "".to_string(),
/// }).unwrap();
///
/// assert_eq!(true, task_does_exist);
/// ```
pub fn task_exists(task: Task) -> Result<bool, String> {
    match env::consts::OS {
        "linux" => task_exists_linux(task),
        "macos" => task_exists_macos(task),
        "windows" => task_exists_windows(task),
        _ => Err(format!("Unsupported operating system: {}", env::consts::OS))
    }
}

pub fn schedule_task(task: Task) -> Result<String, String> {
    match env::consts::OS {
        "linux" => schedule_task_linux(task),
        "macos" => schedule_task_macos(task),
        "windows" => schedule_task_windows(task),
        _ => Err(format!("Unsupported operating system: {}", env::consts::OS))
    }
}

pub fn unschedule_task(task: Task) -> Result<String, String> {
    match env::consts::OS {
        "linux" => unschedule_task_linux(task),
        "macos" => unschedule_task_macos(task),
        "windows" => unschedule_task_windows(task),
        _ => Err(format!("Unsupported operating system: {}", env::consts::OS))
    }
}