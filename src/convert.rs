use crate::{Error, Result};
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
    if pages.is_empty() {
        return Err(Error::EmptyPDF);
    }

    let len = calc_resulting_length(pages.len());
    debug_assert!(len % 4 == 0);

    let media_box = doc
        .get_object(pages[0])?
        .as_dict()?
        .get(b"MediaBox")?
        .clone();

    for page in &pages {
        let page_mut = doc.get_object_mut(page.clone())?.as_dict_mut()?;
        page_mut.set("Parent", Object::Reference(pages_id));
    }

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

    pages_mut.set("Count", Object::Integer(new_pages.len() as i64));
    pages_mut.set("Kids", Object::Array(new_pages));

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

    fn check_parents(doc: &Document, kids: &Vec<Object>, parent: &ObjectId) -> Result<()> {
        for kid in kids {
            let dict = doc.get_object(kid.as_reference()?)?.as_dict()?;
            assert_eq!(parent, &dict.get(b"Parent")?.as_reference()?);
        }

        Ok(())
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

        let media_box_raw = vec![0, 0, 50, 50];

        let media_box: Vec<_> = media_box_raw.clone().into_iter().map(Into::into).collect();
        let page_id = add_empty_page(&mut doc, media_box.into(), pages_id);

        let dict = doc.get_object(page_id)?.as_dict()?;
        assert_eq!("Page", dict.get(b"Type")?.as_name_str()?);
        assert_eq!(pages_id, dict.get(b"Parent")?.as_reference()?);
        assert_eq!(false, dict.get(b"Contents")?.is_null());

        let restored_media_box: Vec<_> = dict
            .get(b"MediaBox")?
            .as_array()?
            .into_iter()
            .map(|o| o.as_i64().map_err(Into::into))
            .collect::<Result<_>>()?;
        assert_eq!(media_box_raw, restored_media_box);

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

        // re-construct the previous order
        let restored_pages = restore_pages(&pages, orig_pages.len())?;

        check_parents(&doc, &pages, &pages_id)?;
        assert_eq!(expected_len, pages.len());
        assert_eq!(orig_pages, restored_pages);

        Ok(())
    }

    #[test]
    fn test_convert() -> Result<()> {
        let mut doc = make_test_document()?;

        let orig_pages: Vec<ObjectId> = doc.page_iter().collect();
        let expected_len = calc_resulting_length(orig_pages.len());

        convert(&mut doc)?;

        let pages_id = get_pages_id(&doc)?;
        let pages_dict = doc.get_object(pages_id)?.as_dict()?;
        let len = pages_dict.get(b"Count")?.as_i64()?;
        let pages = pages_dict.get(b"Kids")?.as_array()?;

        let restored_pages = restore_pages(pages, orig_pages.len())?;

        check_parents(&doc, pages, &pages_id)?;
        assert_eq!(expected_len, pages.len());
        assert_eq!(expected_len, len as usize);
        assert_eq!(orig_pages, restored_pages);

        Ok(())
    }
}
