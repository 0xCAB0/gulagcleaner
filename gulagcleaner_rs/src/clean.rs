use crate::models::{self, method::Method};
use models::page_type;

use lopdf::{Dictionary, Document, Object, ObjectId};
use std::{collections::HashSet, error::Error};

/// Trait implemented by the different PDF methods
pub trait Cleaner {
    fn clean(&mut self) -> (Vec<u32>, u8);
}

/// Cleans a PDF document by modifying its pages and removing unnecessary content.
///
/// # Arguments
///
/// * `data` - The PDF document data as a vector of bytes.
/// * `force_naive` - A boolean indicating whether to use the naive cleaning method.
///
/// # Returns
///
/// A tuple containing the cleaned PDF document data as a vector of bytes and a method code.
/// The method code indicates the cleaning method used: 0 for "Wuolah", 1 for "StuDocu", and 2 for "Naive".
pub fn clean_pdf(data: Vec<u8>, force_naive: bool) -> (Vec<u8>, u8) {
    //Load the PDF into a Document
    let mut doc = Document::load_mem(&data).unwrap();

    //We first need to determine what method we're using, either "Wuolah", "StuDocu" or "Wuolah naive".
    // We keep it like this to allow for future methods if needed.

    //Each method should mark pages for deletion in to_delete and modify the contents of the pages.

    let (to_delete, method_code) = match_method(&doc, force_naive).clean();

    //Delete the pages that we've marked for deletion.
    for (offset, page) in to_delete.into_iter().enumerate() {
        doc.delete_pages(&[page - offset as u32]);
    }
    //Save the document.
    let mut return_stream = Vec::new();
    doc.save_to(&mut return_stream).unwrap();

    // Should we still return the method_code now that we are going multi-language? I will leave it not returned for now.
    //return_stream.push(method_code);
    (return_stream, method_code)
    //doc.save_to("test.pdf").unwrap();
}

pub fn find_iobj_pairs(first_page: &[(u32, u16)], second_page: &[(u32, u16)]) -> (usize, usize) {
    let unique_first_page: HashSet<&(u32, u16)> = first_page.iter().collect();
    let unique_second_page: HashSet<&(u32, u16)> = second_page.iter().collect();

    let c: Vec<&&(u32, u16)> = unique_first_page
        .intersection(&unique_second_page)
        .collect();
    if c.len() != 2 {
        return (0, 0);
    }
    let first_index = first_page.iter().position(|&r| r == **c[0]).unwrap();
    let second_index = first_page.iter().position(|&r| r == **c[1]).unwrap();

    if first_index < second_index {
        (first_index, second_index)
    } else {
        (second_index, first_index)
    }
}

pub fn get_xobjs<'a>(doc: &'a Document, page: &ObjectId) -> Result<&'a Dictionary, Box<dyn Error>> {
    let resource = doc.get_page_resources(*page);
    let resource_dict;
    if resource.1.is_empty() {
        resource_dict = resource.0.unwrap();
    } else {
        resource_dict = doc.get_object(resource.1[0])?.as_dict()?;
    }

    let xobjs = resource_dict.get(b"XObject")?.as_dict()?;
    Ok(xobjs)
}

pub fn get_objdict<'a>(
    doc: &'a Document,
    obj: (&Vec<u8>, &Object),
) -> Result<&'a Dictionary, Box<dyn Error>> {
    let objdict = &doc
        .get_object(obj.1.as_reference().unwrap())?
        .as_stream()?
        .dict;

    Ok(objdict)
}

pub fn get_images(doc: &Document, xobjs: &Dictionary) -> Result<Vec<(i64, i64)>, Box<dyn Error>> {
    let mut images = Vec::new();

    for obj in xobjs {
        let objectdict = get_objdict(doc, obj)?;

        let subtype = objectdict.get(b"Subtype").unwrap().as_name().unwrap();
        let sub_s = String::from_utf8_lossy(subtype);

        if sub_s.starts_with("Image") {
            images.push((
                objectdict.get(b"Height").unwrap().as_i64().unwrap(),
                objectdict.get(b"Width").unwrap().as_i64().unwrap(),
            ));
        }
    }

    Ok(images)
}

pub fn remove_logo(doc: &mut Document, page: &ObjectId) -> Result<(), Box<dyn Error>> {
    let xobjs = get_xobjs(doc, page)?.clone();
    let images = get_images(doc, &xobjs)?;

    let has_logo = !page_type::LOGO_DIMS
        .iter()
        .collect::<HashSet<_>>()
        .intersection(&images.iter().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .is_empty();

    if has_logo {
        for obj in &xobjs {
            let objectdict = get_objdict(doc, obj)?;

            let subtype = objectdict.get(b"Subtype")?.as_name()?;

            let sub_s = String::from_utf8_lossy(subtype);
            if sub_s.starts_with("Image")
                && page_type::LOGO_DIMS.contains(&(
                    objectdict.get(b"Height")?.as_i64()?,
                    objectdict.get(b"Width")?.as_i64()?,
                ))
            {
                let mutable_page = &mut doc
                    .get_object_mut(obj.1.as_reference()?)?
                    .as_stream_mut()?
                    .dict;
                mutable_page.set(*b"Height", 0);
            }
            {}
        }
    }

    Ok(())
}

/// Creates a new `Method` instance based on the provided `Document` and `force_naive` flag.
///
/// **Disclamer:** We are cloning the reference so it's not a big performance loss
/// # Arguments
///
/// * `doc` - A reference to the `Document` object.
/// * `force_naive` - A boolean flag indicating whether to force the use of the naive method.
///
/// # Returns
///
/// A `Method` instance representing the chosen method based on the provided `Document` and `force_naive` flag.

fn match_method(doc: &Document, force_naive: bool) -> Method {
    //0 for auto, 1 for wuolah, 2 for studocu 3 for wuolah naive
    if force_naive {
        return Method::Naive(doc.clone());
    }

    let pages = doc.get_pages();
    let content_list: Vec<Vec<(u32, u16)>> = pages
        .iter()
        .map(|x| doc.get_page_contents(*x.1))
        .filter(|x| x.len() > 1)
        .collect();

    let to_delete: Vec<u32> = pages
        .iter()
        .filter(|x| {
            let contents = doc.get_page_contents(*x.1);

            if contents.len() == 1 {
                return true;
            } else {
                return false;
            }
        })
        .map(|x| *x.0)
        .collect();

    if content_list
        .iter()
        .map(|x| x.len())
        .filter(|x| *x == 3)
        .collect::<Vec<_>>()
        .len()
        > 1
    {
        return Method::StuDocu(doc.clone(), content_list);
    }

    if content_list.len() > 1
        && content_list[0]
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&content_list[1].iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .len()
            > 1
    {
        return Method::Wuolah(doc.clone(), content_list, to_delete);
    }
    Method::Naive(doc.clone())
}
