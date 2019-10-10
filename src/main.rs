use lopdf::{Document, Error};

fn main() -> Result<(), Error> {
    let mut doc = Document::load("example.pdf")?;
    doc.version = "1.4".to_string();
    doc.replace_text(1, "Hello World!", "Modified text!")?;
    doc.save("modified.pdf")?;
    Ok(())
}
