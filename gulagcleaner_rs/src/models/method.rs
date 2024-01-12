use lopdf::{Document, Object};

use crate::{
    clean::{find_iobj_pairs, remove_logo, Cleaner},
    models::page_type,
};

/// Represents the different methods used in the Gulag Cleaner application.
pub enum Method {
    /// The Wuolah method, which takes a vector of vectors of tuples containing unsigned integers and unsigned shorts,
    /// and a vector of unsigned integers as parameters.
    Wuolah(Document, Vec<Vec<(u32, u16)>>, Vec<u32>),
    /// The StuDocu method, which takes a vector of vectors of tuples containing unsigned integers and unsigned shorts
    /// as a parameter.
    StuDocu(Document, Vec<Vec<(u32, u16)>>),
    /// The Naive method, which does not take any parameters.
    Naive(Document),
}

impl Cleaner for Method {
    fn clean(&mut self) -> (Vec<u32>, u8) {
        match self {
            Method::Wuolah(ref mut doc, content_list, to_delete) => {
                let new_contents: Vec<Vec<(u32, u16)>> = content_list
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let pares = if x == content_list.last().unwrap() {
                            find_iobj_pairs(x, &content_list[i - 1])
                        } else {
                            let check_if_00 = find_iobj_pairs(x, &content_list[i + 1]);
                            if check_if_00 != (0, 0) {
                                check_if_00
                            } else {
                                find_iobj_pairs(x, &content_list[i - 1])
                            }
                        };

                        x[(pares.0) - 2..=(pares.1) + 3].to_vec()
                    })
                    .collect();

                let pages = doc.get_pages();

                let vector: Vec<(&u32, &(u32, u16))> = pages
                    .iter()
                    .filter(|x| doc.get_page_contents(*x.1).len() > 1)
                    .collect();
                for (i, page) in vector.iter().enumerate() {
                    let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                    let contents_objects: Vec<Object> = new_contents[i]
                        .iter()
                        .map(|x| Object::Reference(*x))
                        .collect();

                    mutable_page.set(*b"Contents", lopdf::Object::Array(contents_objects));

                    mutable_page.set("Annots", Object::Array(vec![]));
                    let mediabox = mutable_page.get(b"MediaBox").unwrap().as_array().unwrap();

                    let height_offset = match mediabox[1].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[1].as_i64().unwrap() as f32,
                    };
                    let width_offset = match mediabox[0].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[0].as_i64().unwrap() as f32,
                    };

                    let height = match mediabox[3].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[3].as_i64().unwrap() as f32,
                    };
                    let width = match mediabox[2].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[2].as_i64().unwrap() as f32,
                    };

                    for _box in ["MediaBox", "ArtBox", "TrimBox", "CropBox", "BleedBox"] {
                        mutable_page.set(
                            _box,
                            Object::Array(vec![
                                Object::Real(0.0),
                                Object::Real(0.0),
                                Object::Real(width - width_offset),
                                Object::Real(height - height_offset),
                            ]),
                        );
                    }
                }

                (to_delete.to_vec(), 0)
            }
            Method::StuDocu(ref mut doc, content_list) => {
                let new_contents: Vec<Vec<(u32, u16)>> =
                    content_list.iter().skip(1).map(|x| vec![x[1]]).collect();
                let pages = doc.get_pages();
                let vector: Vec<(&u32, &(u32, u16))> = pages.iter().filter(|x| *x.0 != 1).collect();
                for (i, page) in vector.iter().enumerate() {
                    let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                    let contents_objects: Vec<Object> = new_contents[i]
                        .iter()
                        .map(|x| Object::Reference(*x))
                        .collect();

                    mutable_page.set(*b"Contents", lopdf::Object::Array(contents_objects));

                    mutable_page.set("Annots", Object::Array(vec![]));
                }
                (vec![1], 1)
            }

            Method::Naive(ref mut doc) => {
                println!("Using naive method");
                let mut to_delete = Vec::new();
                let pages = doc.get_pages();

                for page in &pages {
                    let page_type =
                        page_type::PageType::get_page_type(&doc, page.1).unwrap_or_default();
                    let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();

                    let mediabox = mutable_page.get(b"MediaBox").unwrap().as_array().unwrap();
                    let height_offset = match mediabox[1].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[1].as_i64().unwrap() as f32,
                    };
                    let width_offset = match mediabox[0].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[0].as_i64().unwrap() as f32,
                    };

                    let height = match mediabox[3].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[3].as_i64().unwrap() as f32,
                    };
                    let width = match mediabox[2].as_f32() {
                        Ok(h) => h,
                        _ => mediabox[2].as_i64().unwrap() as f32,
                    };

                    match page_type {
                        page_type::PageType::FullPageAds => to_delete.push(*page.0),
                        page_type::PageType::Idk => to_delete.push(*page.0),
                        page_type::PageType::BannerAds => {
                            //1.141
                            let scale = 1.124;
                            for _box in ["MediaBox", "ArtBox", "TrimBox", "CropBox", "BleedBox"] {
                                mutable_page.set(
                                    _box,
                                    Object::Array(vec![
                                        Object::Real(
                                            0.164 * (width - width_offset) + width_offset * scale,
                                        ),
                                        Object::Real(
                                            0.031 * (height - height_offset)
                                                + height_offset * scale,
                                        ),
                                        Object::Real(
                                            0.978 * (width - width_offset) * scale
                                                + width_offset * scale,
                                        ),
                                        Object::Real(
                                            0.865 * (height - height_offset) * scale
                                                + height_offset * scale,
                                        ),
                                    ]),
                                );
                            }

                            let mut contents = doc.get_page_content(*page.1).unwrap();
                            let mut new_contents = Vec::new();
                            let c_prepend = "q\n1.124 0 0 1.124 0 0 cm\n".as_bytes();
                            let c_append = "Q".as_bytes();

                            new_contents.extend_from_slice(c_prepend);
                            new_contents.append(&mut contents);
                            new_contents.extend_from_slice(c_append);

                            doc.change_page_content(*page.1, new_contents).unwrap()
                        }
                        page_type::PageType::Watermark => {
                            for _box in ["MediaBox", "ArtBox", "TrimBox", "CropBox", "BleedBox"] {
                                mutable_page.set(
                                    _box,
                                    Object::Array(vec![
                                        Object::Real(0.015 * (width - width_offset) + width_offset),
                                        Object::Real(
                                            0.05 * (height - height_offset) + height_offset,
                                        ),
                                        Object::Real(0.95 * (width - width_offset) + width_offset),
                                        Object::Real(
                                            0.98 * (height - height_offset) + height_offset,
                                        ),
                                    ]),
                                );
                            }
                        }
                    }
                }

                for page in &pages {
                    // remove the logo
                    let _ = remove_logo(doc, page.1);

                    // remove the annotations
                    let mutable_page = doc.get_object_mut(*page.1).unwrap().as_dict_mut().unwrap();
                    mutable_page.set("Annots", Object::Array(vec![]));
                }

                (to_delete, 2)
            }
        }
    }
}
