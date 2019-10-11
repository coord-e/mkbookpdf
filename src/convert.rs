use lopdf::{dictionary, Document, Error, Object, ObjectId, Stream};

fn calc_resulting_length(len: usize) -> usize {
    if len % 4 == 0 {
        len
    } else {
        len + 4 - len % 4
    }
}

fn add_empty_page(doc: &mut Document, pages_id: ObjectId) -> ObjectId {
    let content_id = doc.add_object(Stream::new(dictionary! {}, vec![]));
    doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    })
}

pub fn convert(doc: &mut Document) -> Result<(), Error> {
    let pages: Vec<ObjectId> = doc.page_iter().collect();
    let len = calc_resulting_length(pages.len());

    let root = doc
        .get_object(doc.trailer.get(b"Root")?.as_reference()?)?
        .as_dict()?;
    let pages_id = root.get(b"Pages")?.as_reference()?;

    let new_pages = (0..len / 4)
        .flat_map(|idx| vec![len - 2 * idx, 1 + 2 * idx, 2 + 2 * idx, len - 1 - 2 * idx])
        .map(|idx| {
            pages
                .get(idx - 1)
                .cloned()
                .unwrap_or_else(|| add_empty_page(doc, pages_id))
        })
        .map(Into::into)
        .collect();

    let pages_mut = doc.get_object_mut(pages_id)?.as_dict_mut()?;

    pages_mut.set(b"Kids".to_vec(), Object::Array(new_pages));
    pages_mut.set(b"Count".to_vec(), Object::Integer(len as i64));

    Ok(())
}
