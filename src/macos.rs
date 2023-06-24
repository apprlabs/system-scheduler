use std::process::{Command};
use crate::helpers::get_command_output;
use crate::Task;

/// The bindings for working with `launchctl` on macOS
///
/// [launchctl](x-man-page://launchctl) & [launchd](x-man-page://launchd) docs
pub fn schedule_task_macos(task: Task) -> Result<String, String> {
    Ok("".to_string())
}

pub fn task_exists_macos(task: Task) -> Result<bool, String> {
    let user = if task.user.is_empty() {
        get_command_output(&mut Command::new("whoami"))?
    } else {
        task.user
    };
    let user_id_output = get_command_output(
        Command::new("id").arg("-u").arg(user)
    )?;
    let tasks_output = get_command_output(
        Command::new("launchctl")
            .arg("print")
            .arg(format!("user/{}", user_id_output.trim()).as_str())
    )?;

    Ok(tasks_output.contains(&task.name))
}

pub fn unschedule_task_macos(task: Task) -> Result<String, String> {
    Ok("".to_string())
}