use std::path::PathBuf;

use clap::Parser;
use observe::trace;
use record::info;
use record::warn;

#[derive(Parser)]
#[command(
    name = "publish",
    about = "Publish generated documents to the workspace"
)]
struct Arguments {
    #[arg(long)]
    runfiles: PathBuf,

    #[arg(long)]
    workspace: PathBuf,

    #[arg(long)]
    manifest: PathBuf,

    #[arg(long)]
    verify: bool,
}

#[trace(channels = [document])]
fn run(arguments: &Arguments) -> miette::Result<()> {
    let workspace = &arguments.workspace;

    let entries =
        std::fs::read_to_string(&arguments.manifest).map_err(|_| error::Error::Output {
            path: arguments.manifest.display().to_string(),
        })?;

    let mut drift = Vec::new();

    for line in entries.lines().filter(|l| !l.is_empty()) {
        let mut parts = line.split('\t');
        let (Some(source), Some(destination)) = (parts.next(), parts.next()) else {
            continue;
        };

        let resolved = resolve(&arguments.runfiles, source)?;
        let target = workspace.join(destination);

        if arguments.verify {
            if let Ok(existing) = std::fs::read(&target) {
                let generated = std::fs::read(&resolved).map_err(|_| error::Error::Output {
                    path: resolved.display().to_string(),
                })?;
                if existing != generated {
                    drift.push(destination.to_owned());
                    warn!("drift: {destination}");
                }
            } else {
                drift.push(destination.to_owned());
                warn!("missing: {destination}");
            }
            continue;
        }

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|_| error::Error::Output {
                path: parent.display().to_string(),
            })?;
        }

        if target.exists() {
            std::fs::remove_file(&target).map_err(|_| error::Error::Output {
                path: target.display().to_string(),
            })?;
        }

        std::fs::copy(&resolved, &target).map_err(|_| error::Error::Output {
            path: target.display().to_string(),
        })?;

        info!("generated: {destination}");
    }

    if !drift.is_empty() {
        return Err(error::Error::Drift {
            files: drift.join(", "),
        }
        .into());
    }

    Ok(())
}

fn main() -> miette::Result<()> {
    trace::initialize(None, |channels| {
        channels.iter().any(|c| c.name == "document")
    })?;
    let result = run(&Arguments::parse());
    trace::flush();
    result
}

#[trace(channels = [document])]
fn resolve(runfiles: &std::path::Path, relative: &str) -> miette::Result<PathBuf> {
    let path = runfiles.join(relative);
    if path.exists() {
        return Ok(path);
    }

    Err(error::Error::Output {
        path: relative.into(),
    }
    .into())
}
