use superslice::Ext;

pub trait LineIndex {
    /// returns line number for given offset
    fn line(&self, offset: usize) -> Option<usize>;

    /// returns char offset of given line index
    fn offset(&self, line: usize) -> Option<usize>;
}

pub struct LinearLineIndex {
    line_offsets: Vec<usize>,
    len: usize
}

impl LinearLineIndex {
    pub fn new(text: &str) -> Self {
        LinearLineIndex {
            line_offsets: LinearLineIndex::calculate_offsets(text),
            len: text.len()
        }
    }

    pub fn line_offsets(&self) -> impl Iterator<Item = &usize> {
        self.line_offsets.iter()
    }

    fn calculate_offsets(text: &str) -> Vec<usize> {
        let mut new_line_offsets = Vec::new();
        new_line_offsets.push(0);
        for (offset, ch) in text.chars().enumerate() {
            match ch {
                '\n' => {
                    new_line_offsets.push(offset + 1)
                }
                _ => {}
            }
        }
        new_line_offsets
    }
}

impl LineIndex for LinearLineIndex {
    fn line(&self, offset: usize) -> Option<usize> {
        if offset >= self.len {
            return None
        }
        let line = self.line_offsets.upper_bound(&offset);
        Some(line)
    }

    fn offset(&self, line: usize) -> Option<usize> {
        if line >= self.line_offsets.len() {
            None
        } else {
            Some(self.line_offsets[line])
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_index_lines(text: &str, expected_line_offsets: Vec<usize>) {
        let index = LinearLineIndex::new(text);
        let actual_line_offsets: Vec<usize> = index.line_offsets()
            .map(|it| it.clone())
            .collect();
        assert_eq!(actual_line_offsets, expected_line_offsets)
    }

    fn test_line_of_offset(text: &str, offset: usize, expected_line: Option<usize>) {
        let index = LinearLineIndex::new(text);
        let actual_line = index.line(offset);
        assert_eq!(actual_line, expected_line)
    }

    fn test_offset_of_line(text: &str, line: usize, expected_offset: Option<usize>) {
        let index = LinearLineIndex::new(text);
        let actual_line = index.offset(line);
        assert_eq!(actual_line, expected_offset)
    }

    #[test]
    fn empty_text() {
        test_index_lines("", vec![0])
    }

    #[test]
    fn new_line() {
        test_index_lines("\n", vec![0, 1])
    }

    #[test]
    fn text_and_nl() {
        test_index_lines("foo\nbar\nbaz", vec![0, 4, 8])
    }

    #[test]
    fn text_line_1() {
        test_line_of_offset("foo\nbar\nbaz", 1, Some(1))
    }

    #[test]
    fn text_line_2() {
        test_line_of_offset("foo\nbar\nbaz", 0, Some(1))
    }

    #[test]
    fn text_line_3() {
        test_line_of_offset("foo\nbar\nbaz", 100, None)
    }

    #[test]
    fn text_offset() {
        test_offset_of_line("foo\nbar\nbaz", 1, Some(4))
    }

    #[test]
    fn text_offset_too_big() {
        test_offset_of_line("foo\nbar\nbaz", 100, None)
    }
}