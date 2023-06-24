# system-scheduler
A Rust crate that enables interaction with system-based scheduling systems (`cron`, `launchctl`, `schtasks`)

## Usage

There are 3 primary functions you'll use while using this crate:

`task_exists(task: Task) -> Result<bool, String>`: Whether the task has been added to the system's scheduler
`schedule_task(task: Task) -> Result<String, String>` Schedules the task in the system's scheduler
`unschedule_task(task: Task) -> Result<String, String>` Unschedules the task from the system's scheduler

