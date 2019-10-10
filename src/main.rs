use lopdf::{Document, Error, Object};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkbooklet")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

fn collect_pages(doc: &Document) -> Result<Vec<Vec<u8>>, Error> {
    doc.page_iter().map(|id| doc.get_page_content(id)).collect()
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let mut doc = Document::load(opt.input)?;

    let pages = collect_pages(&doc)?;

    doc.save(opt.output)?;
    Ok(())
}
