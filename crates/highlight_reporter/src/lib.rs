mod line_index;

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

struct TextHighlighter {

}

struct HighlightOptions {
    always_boxed: bool,
    range_text_width: usize
}

struct LineIndex {
    indices: Vec<usize>
}

//impl LineIndex {
//    fn new(text: &str) -> LineIndex {
//        // TODO build index
////        text.chars().
//    }
//
//    fn line(&self, offset: usize) -> usize {
//        let result = self.indices.binary_search(&offset);
//        // TODO
//        panic!("Line for offset {} not found ", offset)
//    }
//}
//
//impl TextHighlighter {
//    fn render_highlight(&self, highlights: Vec<TextHighlight>, options: HighlightOptions, source: &str) -> String {
//        // TODO compute line index
//        let line_count = source.lines().count();
//        let highlight_by_line = Vec::<Vec<TextHighlight>>::new();
//        let mut text = String::new();
//
//    }
//}

