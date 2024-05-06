
pub struct ChatIter<'a> {
    lines: &'a mut dyn Iterator<Item=String>,
    current_line: Option<String>,
    line_index: usize,
}

impl<'a> ChatIter<'a> {
    pub fn from_lines(lines: &'a mut dyn Iterator<Item=String>) -> Self {
        ChatIter {
            lines,
            current_line: None,
            line_index: 0,
        }
    }
}

impl<'a> Iterator for ChatIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_line.is_none() {
            self.line_index = 0;
            self.current_line = Some(self.lines.next()?);
        }

        let line = self.current_line.as_mut().unwrap();
        match line.chars().nth(self.line_index) {
            Some(c) => {
                self.line_index += 1;
                Some(c)
            },
            None => {
                self.current_line = None;
                Some('\n')
            }
        }
    }
}