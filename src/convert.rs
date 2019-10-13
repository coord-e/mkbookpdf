use crate::Result;
use lopdf::{dictionary, Document, Object, ObjectId, Stream};

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

fn get_pages_id(doc: &Document) -> Result<ObjectId> {
    let root = doc
        .get_object(doc.trailer.get(b"Root")?.as_reference()?)?
        .as_dict()?;
    root.get(b"Pages")?.as_reference().map_err(Into::into)
}

pub fn convert(doc: &mut Document) -> Result<()> {
    let pages: Vec<ObjectId> = doc.page_iter().collect();
    let len = calc_resulting_length(pages.len());

    let pages_id = get_pages_id(doc)?;

    use std::iter::once;
    let new_pages = (0..len / 4)
        .flat_map(|idx| {
            once(len - 2 * idx)
                .chain(once(1 + 2 * idx))
                .chain(once(2 + 2 * idx))
                .chain(once(len - 1 - 2 * idx))
        })
        .map(|idx| {
            pages
                .get(idx - 1)
                .cloned()
                .unwrap_or_else(|| add_empty_page(doc, pages_id))
                .into()
        })
        .collect();

    let pages_mut = doc.get_object_mut(pages_id)?.as_dict_mut()?;

    pages_mut.set(b"Kids".to_vec(), Object::Array(new_pages));
    pages_mut.set(b"Count".to_vec(), Object::Integer(len as i64));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_empty_page() -> Result<()> {
        let mut doc = Document::new();
        let pages_id = get_pages_id(&doc)?;

        let page_id = add_empty_page(&mut doc, pages_id);

        let dict = doc.get_object(page_id)?.as_dict()?;
        assert_eq!(dict.get(b"Type")?.as_name_str()?, "Page");
        assert_eq!(dict.get(b"Parent")?.as_reference()?, pages_id);
        assert_eq!(dict.get(b"Contents")?.is_null(), false);

        Ok(())
    }
}
