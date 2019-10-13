use lopdf::Document;
use mkbooklet::{PrintOpt, Result};
use std::path::PathBuf;
use structopt::StructOpt;
use tempfile::NamedTempFile;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkbooklet")]
#[allow(clippy::option_option)]
struct Opt {
    #[structopt(parse(from_os_str))]
    /// Input PDF file.
    input: PathBuf,

    #[structopt(short, long, required_unless = "destination", parse(from_os_str))]
    /// Specify the output file name. Requried unless --print is used.
    output: Option<PathBuf>,

    #[structopt(short, long, name = "destination")]
    /// Print resulting PDF with `lp` to the named printer.
    print: Option<Option<String>>,

    #[structopt(long, env = "MKBL_LP", default_value = "lp")]
    /// Specify the `lp` executable to use when --print is used.
    lp_bin: String,

    #[structopt(short, long)]
    /// Suppress informational messages.
    quiet: bool,
}

fn print_mode(doc: &mut Document, output: Option<PathBuf>, opts: PrintOpt) -> Result<()> {
    if let Some(ref path) = output {
        doc.save(path)?;
        mkbooklet::print(path, opts)
    } else {
        let mut file = NamedTempFile::new()?;
        doc.save_to(&mut file)?;

        let path = file.into_temp_path();
        mkbooklet::print(path, opts)
    }
}

fn run() -> Result<()> {
    let opt = Opt::from_args();
    let doc = &mut Document::load(opt.input)?;

    mkbooklet::convert(doc)?;

    if let Some(p) = opt.print {
        let popts = PrintOpt {
            printer: p,
            lp_bin: opt.lp_bin,
            quiet: opt.quiet,
        };
        print_mode(doc, opt.output, popts)
    } else {
        let output = opt.output.unwrap();
        doc.save(&output)?;
        if !opt.quiet {
            eprintln!("{}: duplex printing is required, with 2-up left-to-right layout and short-edge binding", output.display());
        }
        Ok(())
    }
}

fn main() {
    std::process::exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("mkbooklet: {}", e);
            1
        }
    });
}
