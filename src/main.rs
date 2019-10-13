use lopdf::Document;
use mkbooklet::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use tempfile::NamedTempFile;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkbooklet")]
#[allow(clippy::option_option)]
struct Opt {
    #[structopt(short, long, required_unless = "print", parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,

    #[structopt(short, long)]
    print: Option<Option<String>>,
}

fn print_mode(doc: &mut Document, output: Option<PathBuf>, printer: Option<String>) -> Result<()> {
    if let Some(ref path) = output {
        doc.save(path)?;
        mkbooklet::print(path, printer)
    } else {
        let mut file = NamedTempFile::new()?;
        doc.save_to(&mut file)?;

        let path = file.into_temp_path();
        mkbooklet::print(path, printer)
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let doc = &mut Document::load(opt.input)?;

    mkbooklet::convert(doc)?;

    if let Some(p) = opt.print {
        print_mode(doc, opt.output, p)
    } else {
        doc.save(opt.output.unwrap())?;
        Ok(())
    }
}
