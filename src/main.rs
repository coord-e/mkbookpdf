use lopdf::Document;
use mkbookpdf::{Error, PrintOpt, Result};
use std::path::PathBuf;
use structopt::StructOpt;
use tempfile::NamedTempFile;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkbookpdf")]
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

    #[structopt(short = "i", long)]
    /// Prompt before printing.
    confirm: bool,

    #[structopt(short = "I", long, conflicts_with = "confirm")]
    /// Prompt before printing large output.
    confirm_large: bool,

    #[structopt(long, env = "MKBL_CONFIRM_WHEN", default_value = "10")]
    /// Specify the number of papars to be prompted when -I is used.
    confirm_when: usize,

    #[structopt(short, long)]
    /// Suppress informational messages.
    quiet: bool,
}

fn print_mode(doc: &mut Document, output: Option<PathBuf>, opts: PrintOpt) -> Result<()> {
    if let Some(ref path) = output {
        doc.save(path)?;
        mkbookpdf::print(path, opts)
    } else {
        let mut file = NamedTempFile::new()?;
        doc.save_to(&mut file)?;

        let path = file.into_temp_path();
        mkbookpdf::print(path, opts)
    }
}

fn confirm(message: &str) -> Result<()> {
    use std::io::{stdin, stdout, Write};

    print!("{} [y/N] ", message);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match input.trim() {
        "y" | "Y" | "yes" => Ok(()),
        "n" | "N" | "no" | "" => Err(Error::Cancelled),
        _ => confirm(message),
    }
}

fn should_confirm(opt: &Opt, len: usize) -> bool {
    match opt {
        Opt { confirm: true, .. } => true,
        Opt {
            confirm_large: true,
            confirm_when: when,
            ..
        } if *when < len => true,
        _ => false,
    }
}

fn run() -> Result<()> {
    let opt = Opt::from_args();
    let doc = &mut Document::load(&opt.input)?;

    let len = mkbookpdf::convert(doc)?;

    if let Some(p) = &opt.print {
        if should_confirm(&opt, len) {
            confirm(&format!("{} papers are to be printed. continue?", len))?;
        }

        let popts = PrintOpt {
            printer: p.clone(),
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
            eprintln!("mkbookpdf: {}", e);
            1
        }
    });
}
