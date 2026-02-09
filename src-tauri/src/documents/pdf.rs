use std::path::Path;

use crate::error::{AppError, AppResult};

pub struct PdfExtractionResult {
    pub text: String,
    pub page_count: i32,
}

pub fn extract_text(path: &Path) -> AppResult<PdfExtractionResult> {
    let bytes = std::fs::read(path)?;

    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| AppError::PdfExtraction(format!("Failed to extract text from {}: {}", path.display(), e)))?;

    let page_count = count_pages(&bytes).unwrap_or(1);

    if text.trim().is_empty() {
        return Err(AppError::PdfExtraction(
            "PDF contains no extractable text. It may be a scanned document requiring OCR.".into(),
        ));
    }

    Ok(PdfExtractionResult {
        text: clean_text(&text),
        page_count,
    })
}

fn count_pages(bytes: &[u8]) -> Option<i32> {
    let content = String::from_utf8_lossy(bytes);
    let count = content.matches("/Type /Page").count();
    // Subtract catalog pages — rough heuristic
    let catalog = content.matches("/Type /Pages").count();
    let pages = count.saturating_sub(catalog);
    if pages > 0 {
        Some(pages as i32)
    } else {
        None
    }
}

fn clean_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let input = "  Hello  \n\n  World  \n   \n  Test  ";
        let result = clean_text(input);
        assert_eq!(result, "Hello\nWorld\nTest");
    }

    #[test]
    fn test_extract_nonexistent_file() {
        let result = extract_text(Path::new("/nonexistent/file.pdf"));
        assert!(result.is_err());
    }
}
