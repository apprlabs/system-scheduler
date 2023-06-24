use crate::Task;

/// The bindings for working with `schtasks` on Windows
///
/// [schtasks](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/schtasks) docs
pub fn schedule_task_windows(task: Task) -> Result<String, String> {
    Ok("".to_string())
}

pub fn task_exists_windows(task: Task) -> Result<bool, String> {
    Ok(false)
}

pub fn unschedule_task_windows(task: Task) -> Result<String, String> {
    Ok("".to_string())
}