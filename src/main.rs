use lopdf::{Document, Error, Object, ObjectId};
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

fn calc_resulting_length(len: usize) -> usize {
    if len % 4 == 0 {
        len
    } else {
        len + 4 - len % 4
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let mut doc = Document::load(opt.input)?;

    let pages: Vec<ObjectId> = doc.page_iter().collect();
    // let len = calc_resulting_length(pages.len());

    let root = doc
        .get_object(doc.trailer.get(b"Root")?.as_reference()?)?
        .as_dict()?;
    let pages_id = root.get(b"Pages")?.as_reference()?;

    let pages_mut = doc.get_object_mut(pages_id)?.as_dict_mut()?;

    pages_mut.set(
        b"Kids".to_vec(),
        Object::Array(pages.into_iter().rev().map(Into::into).collect()),
    );
    // let kids_mut = pages.get_mut(b"Kids")?.as_array_mut().unwrap();
    // let count = pages.get(b"Count")?.as_i64()?;

    doc.save(opt.output)?;
    Ok(())
}
