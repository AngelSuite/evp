use markdown::{ParseOptions, mdast::Node};
use rust_xlsxwriter::{Format, FormatBorder, Image, Note, Workbook, Worksheet};
use uuid::Uuid;

use crate::prelude::{Error, EvidencePackage, Result as EVPResult, TestCase, TestCasePassStatus};

use super::Exporter;

/// An exporter to an Excel document.
#[derive(Default)]
pub struct ExcelExporter;

impl Exporter for ExcelExporter {
    fn export_name() -> String {
        "Excel Workbook".to_string()
    }

    fn export_extension() -> String {
        ".xlsx".to_string()
    }

    fn export_package(
        &mut self,
        package: &mut EvidencePackage,
        path: std::path::PathBuf,
    ) -> EVPResult<()> {
        let mut workbook = Workbook::new();
        workbook.read_only_recommended();

        create_metadata_sheet(workbook.add_worksheet(), package)
            .map_err(Error::OtherExportError)?;

        create_summary_sheet(workbook.add_worksheet(), package).map_err(Error::OtherExportError)?;

        let test_cases: Vec<&TestCase> = package.test_case_iter()?.collect();
        for test_case in test_cases {
            let worksheet = workbook.add_worksheet();
            create_test_case_sheet(worksheet, package.clone(), test_case)
                .map_err(Error::OtherExportError)?;
        }

        workbook
            .save(path)
            .map_err(|e| Error::OtherExportError(e.into()))?;

        Ok(())
    }

    fn export_case(
        &mut self,
        package: &mut EvidencePackage,
        case: Uuid,
        path: std::path::PathBuf,
    ) -> EVPResult<()> {
        let mut workbook = Workbook::new();

        let worksheet = workbook.add_worksheet();
        let case = package
            .test_case(case)?
            .ok_or(Error::OtherExportError("Test case not found!".into()))?;
        create_test_case_sheet(worksheet, package.clone(), case)
            .map_err(Error::OtherExportError)?;

        workbook
            .save(path)
            .map_err(|e| Error::OtherExportError(e.into()))?;

        Ok(())
    }
}

/// Create the worksheet for the metadata
fn create_metadata_sheet(
    worksheet: &mut Worksheet,
    package: &EvidencePackage,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("Creating excel sheet for metadata");
    worksheet.set_name(package.metadata().title())?;
    worksheet.set_screen_gridlines(false);
    worksheet.set_column_width(0, 3)?; // To appear tidy

    let mut row = 1;

    let title = Format::new().set_bold().set_font_size(14);
    let italic = Format::new().set_italic();

    // Write title and execution timestamp
    worksheet.write_string_with_format(row, 1, package.metadata().title(), &title)?;
    row += 1;

    for author in package.metadata().authors() {
        row += 1;
        worksheet.write_string_with_format(row, 1, format!("{author}"), &italic)?;
    }

    row += 2;
    if let Some(description) = package.metadata().description() {
        worksheet.write_string(row, 1, description)?;
    }

    if let Ok(branding_img) = std::env::var("EA_BRAND_IMAGE") {
        row += 2;
        let image = Image::new(branding_img)?;
        worksheet.insert_image(row, 1, &image)?;
    }

    Ok(())
}

/// Create the worksheet for the test case summary
fn create_summary_sheet(
    worksheet: &mut Worksheet,
    package: &EvidencePackage,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("Creating excel sheet for summary");
    worksheet.set_name("Summary")?;
    worksheet.set_screen_gridlines(false);
    worksheet.set_column_width(0, 3)?; // To appear tidy

    let mut row = 1;

    let title = Format::new().set_bold().set_font_size(14);
    let bold_bordered = Format::new().set_bold().set_border(FormatBorder::Thin);
    let bordered = Format::new().set_border(FormatBorder::Thin);

    // Write title and execution timestamp
    worksheet.write_string_with_format(row, 1, "Summary", &title)?;
    row += 2;

    // Write header row
    worksheet.write_string_with_format(row, 1, "Test Case", &bold_bordered)?;
    worksheet.write_string_with_format(row, 2, "Executed At", &bold_bordered)?;
    worksheet.write_string_with_format(row, 3, "Status", &bold_bordered)?;
    let mut custom_keys = vec![];
    if let Some(fields) = package.metadata().custom_metadata() {
        let mut fields = fields.iter().collect::<Vec<_>>();
        fields.sort_by(|(_, a), (_, b)| a.cmp(b));
        for (idx, (key, field)) in fields.iter().enumerate() {
            let col = u16::try_from(4 + idx)?;
            custom_keys.push((*key).clone());
            worksheet.write_string_with_format(row, col, field.name(), &bold_bordered)?;
            if !field.description().is_empty() {
                worksheet.insert_note(row, col, &Note::new(field.description()))?;
            }
        }
    }
    row += 1;

    // Write data rows
    for test_case in package.test_case_iter()? {
        worksheet.write_string_with_format(row, 1, test_case.metadata().title(), &bordered)?;
        worksheet.write_string_with_format(
            row,
            2,
            test_case.metadata().execution_datetime().to_rfc3339(),
            &bordered,
        )?;
        worksheet.write_string_with_format(
            row,
            3,
            match test_case.metadata().passed() {
                None => "",
                Some(TestCasePassStatus::Pass) => "Pass",
                Some(TestCasePassStatus::Fail) => "Fail",
            },
            &bordered,
        )?;
        for (idx, key) in custom_keys.iter().enumerate() {
            let col = u16::try_from(4 + idx)?;
            worksheet.write_string_with_format(row, col, "", &bordered)?;
            if let Some(custom) = test_case.metadata().custom() {
                if let Some(data) = custom.get(key) {
                    worksheet.write_string_with_format(row, col, data, &bordered)?;
                }
            }
        }
        row += 1;
    }
    worksheet.autofit();

    Ok(())
}

/// Create the worksheet that holds the test case's information
fn create_test_case_sheet(
    worksheet: &mut Worksheet,
    mut package: EvidencePackage,
    test_case: &TestCase,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("Creating excel sheet for test case {}", test_case.id());
    worksheet.set_name(test_case.metadata().title())?;
    worksheet.set_screen_gridlines(false);
    worksheet.set_column_width(0, 3)?; // To appear tidy
    worksheet.set_column_width(1, 13)?; // For "Executed at:"
    worksheet.set_column_width(2, 20)?; // For execution date/time

    let mut row = 1;

    let title = Format::new().set_bold().set_font_size(14);
    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let file_data = Format::new()
        .set_font_name("Courier New")
        .set_border_left(FormatBorder::Thick);

    // Write title and execution timestamp
    worksheet.write_string_with_format(row, 1, test_case.metadata().title(), &title)?;
    row += 1;
    worksheet.write(row, 1, "Executed at:")?;
    worksheet.write_with_format(
        row,
        2,
        &test_case.metadata().execution_datetime().naive_local(),
        &Format::new().set_num_format("yyyy-mm-dd hh:mm"),
    )?;
    row += 1;
    match test_case.metadata().passed() {
        None => (),
        Some(s) => {
            let s = match s {
                TestCasePassStatus::Pass => "✅ Pass",
                TestCasePassStatus::Fail => "❌ Fail",
            };
            worksheet.write(row, 1, s)?;
            row += 1;
        }
    }
    if let Some(fields) = test_case.metadata().custom() {
        for (key, value) in fields {
            let field = package
                .metadata()
                .custom_metadata()
                .as_ref()
                // SAFETY: guanteed by EVP spec
                .unwrap()
                .get(key)
                // SAFETY: guanteed by EVP spec
                .unwrap();
            worksheet.write(row, 1, format!("{}: {}", field.name(), value))?;
            row += 1;
        }
    }
    row += 1;

    // Write evidence
    for evidence in test_case.evidence() {
        if let Some(caption) = evidence.caption() {
            worksheet.write_with_format(row, 1, caption, &italic)?;
            row += 1;
        }

        match evidence.kind().as_str() {
            "text/plain" => {
                let data = evidence.value().get_data(&mut package)?;
                let text = String::from_utf8_lossy(data.as_slice());
                for line in text.lines() {
                    worksheet.write_string(row, 1, line)?;
                    row += 1;
                }
            }
            "text/markdown" => {
                let data = evidence.value().get_data(&mut package)?;
                let text = String::from_utf8_lossy(data.as_slice());

                if let Ok(ast) = markdown::to_mdast(&text, &ParseOptions::default()) {
                    let excel_data = markdown_to_excel(ast, &Format::default());
                    for line in excel_data {
                        let line = line
                            .iter()
                            .map(|(a, b)| (a, b.as_str()))
                            .collect::<Vec<_>>();
                        worksheet.write_rich_string_with_format(
                            row,
                            1,
                            line.iter().as_slice(),
                            &Format::default(),
                        )?;
                        row += 1;
                    }
                } else {
                    for line in text.lines() {
                        worksheet.write_string_with_format(
                            row,
                            1,
                            line,
                            &Format::default().set_font_name("Courier New"),
                        )?;
                        row += 1;
                    }
                }
            }
            "text/vnd.angel.http-data" => {
                worksheet.write_string_with_format(row, 1, "HTTP Request", &bold)?;
                row += 1;
                let data = evidence.value().get_data(&mut package)?;
                let text = String::from_utf8_lossy(data.as_slice());
                for line in text.lines() {
                    worksheet.write_string_with_format(row, 1, line, &file_data)?;
                    row += 1;
                }
            }
            mime => {
                if mime.starts_with("image/") {
                    let data = evidence.value().get_data(&mut package)?;
                    let image: Image = Image::new_from_buffer(data.as_slice())?;
                    worksheet.insert_image(row, 1, &image)?;

                    // Calculate row offset
                    let height_in = image.height() / image.height_dpi();
                    let row_units_per_in = 4.87;
                    let num_rows_to_skip = (height_in * row_units_per_in).ceil() as u32;
                    row += num_rows_to_skip;
                } else {
                    let data = evidence.value().get_data(&mut package)?;
                    let text = String::from_utf8_lossy(data.as_slice());

                    if let Some(filename) = evidence.original_filename() {
                        worksheet.write_string(row, 1, filename)?;
                        row += 1;
                    }

                    // Check if plaintext ASCII
                    let mut is_printable = true;
                    for c in text.chars() {
                        if !c.is_ascii() {
                            is_printable = false;
                            break;
                        }
                    }

                    if is_printable {
                        for line in text.lines() {
                            worksheet.write_string_with_format(row, 1, line, &file_data)?;
                            row += 1;
                        }
                    } else {
                        worksheet.write_string_with_format(row, 1, "binary file data", &italic)?;
                        row += 1;
                    }
                }
            }
        }

        row += 1;
    }

    Ok(())
}

fn markdown_to_excel(node: Node, format: &Format) -> Vec<Vec<(Format, String)>> {
    match node {
        Node::Root(root) => root
            .children
            .into_iter()
            .map(|c| markdown_to_excel(c, format))
            .flatten()
            .collect(),
        Node::Break(_) => vec![],
        Node::Paragraph(para) => para
            .children
            .into_iter()
            .map(|c| markdown_to_excel(c, format))
            .flatten()
            .collect(),
        Node::List(list) => list
            .children
            .into_iter()
            .map(|c| markdown_to_excel(c, format))
            .flatten()
            .collect(),
        Node::ListItem(item) => {
            let mut item = item
                .children
                .into_iter()
                .map(|c| markdown_to_excel(c, format))
                .flatten()
                .collect::<Vec<_>>();
            if let Some(fir) = item.first_mut() {
                fir.insert(0, (format.clone(), "• ".to_string()));
            }
            item
        }
        Node::Heading(hdg) => {
            let font_sizes = [32, 28, 24, 18, 16, 14];
            let size = font_sizes.get(usize::from(hdg.depth)).unwrap_or(&14);
            hdg.children
                .into_iter()
                .map(|c| markdown_to_excel(c, &format.clone().set_font_size(*size)))
                .flatten()
                .collect()
        }
        Node::Code(code) => {
            vec![vec![(
                format.clone().set_font_name("Courier New"),
                code.value,
            )]]
        }
        Node::Text(text) => {
            vec![vec![(format.clone(), text.value)]]
        }
        Node::InlineCode(code) => {
            vec![vec![(
                format.clone().set_font_name("Courier New"),
                code.value,
            )]]
        }
        Node::Strong(strong) => strong
            .children
            .into_iter()
            .map(|c| markdown_to_excel(c, &format.clone().set_bold()))
            .flatten()
            .collect(),
        Node::Emphasis(emph) => emph
            .children
            .into_iter()
            .map(|c| markdown_to_excel(c, &format.clone().set_italic()))
            .flatten()
            .collect(),
        _ => vec![],
    }
}
