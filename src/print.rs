use crate::{Error, Result};
use std::io::ErrorKind;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct PrintOpt {
    pub printer: Option<String>,
    pub lp_bin: String,
    pub quiet: bool,
}

pub fn print<P: AsRef<Path>>(path: P, opts: PrintOpt) -> Result<()> {
    let path = path.as_ref();
    let path_str = path
        .to_str()
        .ok_or_else(|| Error::InvaildPath(path.to_path_buf()))?;

    let mut cmd = Command::new(&opts.lp_bin);

    cmd.args(&[
        "-o",
        "sides=two-sided-short-edge",
        "-o",
        "number-up=2",
        "-o",
        "number-up-layout=lrtb",
        path_str,
    ]);

    if let Some(p) = &opts.printer {
        cmd.args(&["-d", p]);
    }

    if opts.quiet {
        cmd.stdout(Stdio::null());
    }

    let status = cmd.status().map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            Error::LPNotFound(opts.lp_bin.clone())
        } else {
            Error::IO(e)
        }
    })?;

    if !status.success() {
        Err(Error::Print(opts.lp_bin, status))
    } else {
        Ok(())
    }
}
