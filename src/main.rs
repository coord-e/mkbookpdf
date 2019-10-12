use lopdf::Document;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkbooklet")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,

    #[structopt(short, long)]
    print: Option<Option<String>>,
}

fn main() -> Result<(), mkbooklet::Error> {
    let opt = Opt::from_args();
    let mut doc = Document::load(opt.input)?;

    mkbooklet::convert(&mut doc)?;

    doc.save(&opt.output)?;

    if let Some(printer) = opt.print {
        mkbooklet::print(opt.output, printer)
    } else {
        Ok(())
    }
}
