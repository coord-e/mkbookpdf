use crate::Error;
use std::io::ErrorKind::NotFound;
use std::path::Path;
use std::process::Command;

pub fn print<P: AsRef<Path>>(path: P, printer: Option<String>) -> Result<(), Error> {
    let path = path.as_ref();
    let path_str = path
        .to_str()
        .ok_or_else(|| Error::InvaildPath(path.to_path_buf()))?;

    let mut cmd = Command::new("lp");

    cmd.args(&[
        "-o",
        "sides=two-sided-short=edge",
        "-o",
        "number-up=2",
        "-o",
        "number-up-layout=lrtb",
        path_str,
    ]);

    if let Some(p) = printer {
        cmd.args(&["-d", &p]);
    }

    let status = cmd.status().map_err(|e| {
        if e.kind() == NotFound {
            Error::LPNotFound
        } else {
            Error::IO(e)
        }
    })?;

    if !status.success() {
        Err(Error::Print(status))
    } else {
        Ok(())
    }
}
