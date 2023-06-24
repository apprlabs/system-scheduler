use crate::Task;

/// The bindings for working with `crontab` on linux
///
/// [crontab](x-man-page://crontab) docs
pub fn schedule_task_linux(task: Task) -> Result<String, String> {
    Ok("".to_string())
}

pub fn task_exists_linux(task: Task) -> Result<bool, String> {
    Ok(false)
}

pub fn unschedule_task_linux(task: Task) -> Result<String, String> {
    Ok("".to_string())
}