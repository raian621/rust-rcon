use std::rc::Rc;

/// Segment of a line in the scroll buffer. Represents a line that will be
/// printed out on the terminal
pub struct LineChunk {
    chunk: String,
    line: usize,
}

impl LineChunk {
    pub fn segment(line: Rc<String>, length: usize, line_index: usize) -> Vec<Self> {
        let mut segments = Vec::new();
        let mut start = 0;

        while start < line.len() {
            let end = std::cmp::min(start+length, line.len());
            segments.push(
                LineChunk{
                    chunk: line.as_ref()[start..end].to_string(),
                    line: line_index
                }
            );
            start += length;
        }

        segments
    }
}

pub struct Scroller {
    lines: Vec<Rc<String>>,
    buffer: Rc<Vec<LineChunk>>,
    index: isize
}

impl Scroller {
    pub fn add_line(&mut self, line: String) {
        self.lines.push(Rc::new(line));
        self.index = self.lines.len() as isize - 1;
    }

    pub fn new() -> Self {
        Self { lines: Vec::new(), index: 0, buffer: Rc::new(Vec::new()) }
    }

    /// Moves the scroller's internal index to the line before the current line
    /// if it exists. If the internal index is less than `0`, `None` is
    /// returned.
    pub fn prev(&mut self) -> Option<()> {
        if self.index >= 0 {
            self.index -= 1;
            if self.index == -1 {
                return None;
            }
            return Some(());
        }

        None
    }

    pub fn render_lines(&mut self, cols: usize) -> Rc<Vec<LineChunk>> {
        if self.buffer.len() > 0 && cols == self.buffer[0].chunk.len() {
            return self.buffer.clone();
        }
        
        // rerender buffer
        Rc::<Vec<LineChunk>>::get_mut(&mut self.buffer).unwrap().clear();
        for (i, line) in self.lines.iter().enumerate() {
            let mut segments = LineChunk::segment(line.clone(), cols, i);
            Rc::<Vec<LineChunk>>::get_mut(&mut self.buffer)
                .unwrap()
                .append(&mut segments);
        }

        self.buffer.clone()
    }
}

impl Iterator for Scroller {
    type Item = ();

    /// Moves the scroller's internal index to the line after the current line
    /// if it exists. If the internal index is greater than or equal to the
    /// length of the lines array, `None` is returned.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.lines.len() as isize {
            self.index += 1;
            if self.index == self.lines.len() as isize {
                return None;
            }
            return Some(());
        }

        None
    }
}

impl Default for Scroller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrolling() {
        let mut scroller = Scroller::new();
        let lines = vec![
            "effect give @s minecraft:strength 1 244",
            "effect give @a minecraft:slowness 1 233",
            "tp @s Notch",
            "list",
        ];

        lines.iter().for_each(|line| scroller.add_line(line.to_string()));

        assert_eq!(scroller.index, 3);
        assert_eq!(scroller.prev(), Some(()));
        assert_eq!(scroller.index, 2);
    }

    #[test]
    fn test_line_segmentation() {
        let line = "this is a line yeah alright".to_string();
        let length = 10;

        let expected = vec![
            "this is a ".to_string(),
            "line yeah ".to_string(),
            "alright".to_string(),
        ];

        let chunks = LineChunk::segment(Rc::new(line), length, 0);
        let strings = chunks.iter().map(|c| c.chunk.clone()).collect::<Vec<String>>();

        chunks.iter().for_each(|chunk| assert_eq!(chunk.line, 0));
        assert_eq!(expected, strings);
    }

    #[test]
    fn test_lines_segmentation() {
        let lines = vec![
            "effect give @s minecraft:strength 1 244",
            "effect give @a minecraft:slowness 1 233",
            "tp @s Notch",
            "list",
        ].iter().map(|line| Rc::new(line.to_string()))
            .collect::<Vec<Rc<String>>>();
        let mut scroller = Scroller::new();
        scroller.lines = lines;
        scroller.render_lines(10);
        let buffer_lines = scroller.buffer
            .iter()
            .map(|line| line.chunk.clone())
            .collect::<Vec<String>>();
        let expected = vec![
            "effect giv",
            "e @s minec",
            "raft:stren",
            "gth 1 244",
            "effect giv",
            "e @a minec",
            "raft:slown",
            "ess 1 233",
            "tp @s Notc",
            "h",
            "list",
        ].iter().map(|e| e.to_string()).collect::<Vec<String>>();
        assert_eq!(expected, buffer_lines);

        let buffer_line_nums = scroller.buffer
            .iter()
            .map(|line| line.line)
            .collect::<Vec<usize>>();
        let expected = vec![0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3];
        assert_eq!(expected, buffer_line_nums);
    }
}
