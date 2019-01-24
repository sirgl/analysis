use crate::line_index::LinearLineIndex;
use std::collections::HashMap;
use crate::line_index::LineIndex;
use std::cmp::max;


/// Offsets of
pub struct HighlightRange {
    start_offset: usize,
    end_offset: usize
}

impl HighlightRange {
    pub fn new(start: usize, end: usize) -> Self {
        HighlightRange { start_offset: start, end_offset: end }
    }
}

struct TextHighlight {
    range: HighlightRange,
    text: String
}

impl TextHighlight {
    pub fn new(range: HighlightRange, text: String) -> Self {
        TextHighlight { range, text }
    }
}




struct TextHighlighter<'a> {
    text: &'a str,
    line_index: LinearLineIndex
}

impl<'a> TextHighlighter<'a> {
    pub fn new(text: &'a str) -> Self {
        TextHighlighter {
            text,
            line_index : LinearLineIndex::new(text)
        }
    }
}

struct HighlightOptions {
    always_boxed: bool,
    range_text_width: usize,
    prefix: String
}

impl <'a>TextHighlighter<'a> {
    fn render_highlights(&self, highlights: Vec<TextHighlight>, options: HighlightOptions) -> String {
        let line_count = self.text.lines().count();
        let mut text = String::new();
        let line_index = LinearLineIndex::new(&text);
        let line_to_highlights = group_by_line(highlights, &line_index);

        // TODO
        text
    }
}

fn group_by_line(highlights: Vec<TextHighlight>, line_index: &LinearLineIndex) -> HashMap<usize, Vec<TextHighlight>> {
    let mut line_to_highlights: HashMap<usize, Vec<TextHighlight>> = HashMap::new();
    let mut max_line = 0;
    for highlight in highlights {
        let offset = highlight.range.end_offset;
        let line = line_index.line(offset).unwrap();
        max_line = max(line, max_line);
        if !line_to_highlights.contains_key(&line) {
            let mut highlights = Vec::new();
            highlights.push(highlight);
            line_to_highlights.insert(line, highlights);
        } else {
            line_to_highlights.get_mut(&line).unwrap().push(highlight)
        }
    }
    line_to_highlights
}