use crate::Result;
use lopdf::{dictionary, Document, Object, ObjectId, Stream};

fn calc_resulting_length(len: usize) -> usize {
    if len % 4 == 0 {
        len
    } else {
        len + 4 - len % 4
    }
}

fn add_empty_page(doc: &mut Document, media_box: Object, pages_id: ObjectId) -> ObjectId {
    let content_id = doc.add_object(Stream::new(dictionary! {}, vec![]));
    doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
        "MediaBox" => media_box,
    })
}

fn get_pages_id(doc: &Document) -> Result<ObjectId> {
    let root = doc
        .get_object(doc.trailer.get(b"Root")?.as_reference()?)?
        .as_dict()?;
    root.get(b"Pages")?.as_reference().map_err(Into::into)
}

fn build_new_pages(doc: &mut Document, pages_id: ObjectId) -> Result<Vec<Object>> {
    let pages: Vec<ObjectId> = doc.page_iter().collect();
    let len = calc_resulting_length(pages.len());
    debug_assert!(len % 4 == 0);

    let media_box = doc
        .get_object(pages[0])?
        .as_dict()?
        .get(b"MediaBox")?
        .clone();

    use std::iter::once;
    Ok((0..len / 4)
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
                .unwrap_or_else(|| add_empty_page(doc, media_box.clone(), pages_id))
                .into()
        })
        .collect())
}

pub fn convert(doc: &mut Document) -> Result<()> {
    let pages_id = get_pages_id(doc)?;

    let new_pages = build_new_pages(doc, pages_id)?;

    let pages_mut = doc.get_object_mut(pages_id)?.as_dict_mut()?;

    pages_mut.set(b"Count".to_vec(), Object::Integer(new_pages.len() as i64));
    pages_mut.set(b"Kids".to_vec(), Object::Array(new_pages));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    fn make_test_document() -> Result<Document> {
        Document::load("tests/data/sample.pdf").map_err(Into::into)
    }

    fn restore_pages(pages: &Vec<Object>, orig_len: usize) -> Result<Vec<ObjectId>> {
        let (first, last): (Vec<_>, Vec<_>) =
            pages.iter().enumerate().partition(|(i, _)| match i % 4 {
                1 | 2 => true,
                0 | 3 => false,
                _ => unreachable!(),
            });
        first
            .into_iter()
            .chain(last.into_iter().rev())
            .map(|(_, obj)| obj.as_reference().map_err(Error::from))
            .take(orig_len)
            .collect()
    }

    #[test]
    fn test_calc_resulting_length() {
        assert_eq!(16, calc_resulting_length(15));
        assert_eq!(4, calc_resulting_length(2));
        assert_eq!(32, calc_resulting_length(29));
        assert_eq!(64, calc_resulting_length(64));
    }

    #[test]
    fn test_add_empty_page() -> Result<()> {
        let mut doc = make_test_document()?;
        let pages_id = get_pages_id(&doc)?;

        let media_box = vec![0.into(), 0.into(), 50.into(), 50.into()].into();
        let page_id = add_empty_page(&mut doc, media_box, pages_id);

        let dict = doc.get_object(page_id)?.as_dict()?;
        assert_eq!("Page", dict.get(b"Type")?.as_name_str()?);
        assert_eq!(pages_id, dict.get(b"Parent")?.as_reference()?);
        assert_eq!(false, dict.get(b"Contents")?.is_null());

        Ok(())
    }

    #[test]
    fn test_get_pages_id() -> Result<()> {
        let doc = make_test_document()?;

        let orig_pages: Vec<ObjectId> = doc.page_iter().collect();
        let pages_id = get_pages_id(&doc)?;

        let dict = doc.get_object(pages_id)?.as_dict()?;
        assert_eq!(orig_pages.len() as i64, dict.get(b"Count")?.as_i64()?,);

        Ok(())
    }

    #[test]
    fn test_build_new_pages() -> Result<()> {
        let mut doc = make_test_document()?;

        let orig_pages: Vec<ObjectId> = doc.page_iter().collect();
        let expected_len = calc_resulting_length(orig_pages.len());

        let pages_id = get_pages_id(&doc)?;
        let pages = build_new_pages(&mut doc, pages_id)?;

        assert_eq!(expected_len, pages.len());

        // re-construct the previous order
        let restored_pages = restore_pages(&pages, orig_pages.len())?;

        assert_eq!(orig_pages, restored_pages);

        Ok(())
    }

    #[test]
    fn test_convert() -> Result<()> {
        let mut doc = make_test_document()?;

        let orig_pages: Vec<ObjectId> = doc.page_iter().collect();
        let expected_len = calc_resulting_length(orig_pages.len());

        convert(&mut doc)?;

        let pages_dict = doc.get_object(get_pages_id(&doc)?)?.as_dict()?;
        let len = pages_dict.get(b"Count")?.as_i64()?;
        let pages = pages_dict.get(b"Kids")?.as_array()?;

        let restored_pages = restore_pages(pages, orig_pages.len())?;

        assert_eq!(expected_len, pages.len());
        assert_eq!(expected_len, len as usize);
        assert_eq!(orig_pages, restored_pages);

        Ok(())
    }
}
