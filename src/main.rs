use std::path::PathBuf;
use lopdf::{Document, Error, Object};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkbooklet")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let mut doc = Document::load(opt.input)?;
    doc.replace_text(1, "a", "horaaaaaaay!")?;
    doc.save(opt.output)?;
    Ok(())
}
