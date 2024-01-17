use crate::clean::clean_pdf;
use lopdf::Document;
use std::fs;

const OUT_PATH: &str = "example_docs/out";

/// Creates out folder if missing so tests won't fail
fn create_out_folder() {
    fs::create_dir_all(OUT_PATH).unwrap();
}

/// Gulag-tester compare method
fn compare(cleaned: &[u8], original: &[u8]) {
    //Checks that the clean pdf is smaller than the original
    assert!(cleaned.len() <= original.len());

    let cleaned_pdf = Document::load_mem(&cleaned).unwrap();
    let original_pdf = Document::load_mem(&original).unwrap();

    let original_pages = original_pdf.get_pages();
    let cleaned_pages = cleaned_pdf.get_pages();

    //Checks that the clean pdf has the same number of pages as the original
    assert!(cleaned_pages.len() <= original_pages.len());

    //Checks that the clean pdf has the same number of objects as the original
    // assert_eq!(
    //     original_pages
    //         .into_iter()
    //         .map(|page| -> usize {
    //             original_pdf
    //                 .get_page_contents(page.1)
    //                 .into_iter()
    //                 .map(|x| x.1)
    //                 .len()
    //         })
    //         .sum::<usize>(),
    //     cleaned_pages
    //         .into_iter()
    //         .map(|page| -> usize {
    //             cleaned_pdf
    //                 .get_page_contents(page.1)
    //                 .into_iter()
    //                 .map(|x| x.1)
    //                 .len()
    //         })
    //         .sum::<usize>()
    // );
}

#[test]
fn test_wuolah() {
    create_out_folder();

    //Load some pdf bytes and clean it
    let data = std::fs::read("example_docs/wuolah-free-example.pdf").expect(
        "Missing Wuolah test PDF, please store one in path `example_docs/wuolah-free-example.pdf",
    );

    let (clean_pdf, method) = clean_pdf(&data, false);

    //Checks that the method used is the correct one
    // assert_eq!(method, 0);
    assert_eq!(method, 2);

    //Checks that the clean pdf is smaller than the original
    compare(&clean_pdf, &data);

    //Stores the clean pdf in the out directory
    std::fs::write(format!("{}/wuolah_clean.pdf", OUT_PATH), clean_pdf).unwrap();
}
#[test]

fn test_studocu() {
    create_out_folder();

    //Load some pdf bytes and clean it
    let data = std::fs::read("example_docs/studocu-example.pdf").expect(
        "Missing Studocu test PDF, please store one in path `example_docs/studocu-example.pdf",
    );

    let (clean_pdf, method) = clean_pdf(&data, false);

    //Checks that the clean pdf is smaller than the original
    assert_eq!(method, 1);

    //Stores the clean pdf in the out directory
    std::fs::write(format!("{}/studocu_clean.pdf", OUT_PATH), clean_pdf).unwrap();
}
#[test]

fn test_wuolah_naive() {
    create_out_folder();

    //Load some pdf bytes and clean it
    let data = std::fs::read("example_docs/wuolah-free-example.pdf").expect(
        "Missing Studocu test PDF, please store one in path `example_docs/studocu-example.pdf",
    );

    let (clean_pdf, method) = clean_pdf(&data, true);

    //Checks that the method used is the correct one
    assert_eq!(method, 2);

    //Checks that the clean pdf is smaller than the original
    compare(&clean_pdf, &data);

    //Stores the clean pdf in the out directory
    std::fs::write(format!("{}/wuolah_naive_clean.pdf", OUT_PATH), clean_pdf).unwrap();
}
