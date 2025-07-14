use std::{env, path::Path, process, sync::mpsc};

use notify::{Event, RecursiveMode, Watcher};
use notify_rust::{Notification, Timeout, Urgency};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Failed to parse arguments.\nUsage: {} <path>",
            args.get(0).unwrap()
        );
        process::exit(1);
    }

    let path = Path::new(args.get(1).unwrap());
    if !path.exists() {
        eprintln!("The path you entered doesn't exist.");
        process::exit(1);
    }

    Notification::new()
        .summary("fwatcherd started")
        .body(&format!(
            "watching for events on the path {:?}",
            path.file_name().unwrap()
        ))
        .timeout(Timeout::Milliseconds(4000))
        .show()?;

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(path, RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => handle_event(event, path)?,
            Err(e) => eprintln!("fwatcherd error: {:?}", e),
        }
    }

    Ok(())
}

fn handle_event(ev: Event, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    match ev.kind {
        notify::EventKind::Access(_) => {
            Notification::new()
                .summary("fwatcherd")
                .body(&format!(
                    "{:?} has been accessed!",
                    path.file_name().unwrap()
                ))
                .timeout(Timeout::Never)
                .urgency(Urgency::Normal)
                .show()?;
        }
        notify::EventKind::Create(_) => {
            Notification::new()
                .summary("fwatcherd")
                .body(&format!(
                    "{:?} has been created!",
                    path.file_name().unwrap()
                ))
                .timeout(Timeout::Never)
                .urgency(Urgency::Normal)
                .show()?;
        }
        notify::EventKind::Modify(_) => {
            Notification::new()
                .summary("fwatcherd")
                .body(&format!(
                    "{:?} has been modified!",
                    path.file_name().unwrap()
                ))
                .timeout(Timeout::Never)
                .urgency(Urgency::Normal)
                .show()?;
        }
        notify::EventKind::Remove(_) => {
            Notification::new()
                .summary("fwatcherd")
                .body(&format!(
                    "{:?} has been removed!",
                    path.file_name().unwrap()
                ))
                .timeout(Timeout::Never)
                .urgency(Urgency::Critical)
                .show()?;
        }
        notify::EventKind::Other => {}
        notify::EventKind::Any => {}
    }
    Ok(())
}
