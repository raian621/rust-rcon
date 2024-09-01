pub struct History {
    history: Vec<String>,
    curr_pos: isize,
}

impl History {
    pub fn new() -> Self {
        Self {
            history: Vec::<String>::new(),
            curr_pos: 0
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.curr_pos < self.history.len() as isize {
            self.curr_pos += 1;
            if self.curr_pos == self.history.len() as isize {
                return None;
            } else {
                return Some(&self.history[self.curr_pos as usize]);
            }
        }

        None
    }

    pub fn prev(&mut self) -> Option<&String> {
        if self.curr_pos >= 0 {
            self.curr_pos -= 1;
            return match self.curr_pos {
                -1 => None,
                idx => Some(&self.history[idx as usize])
            };
        }

        None
    }

    pub fn push(&mut self, cmd: String) {
        self.history.push(cmd);
        self.curr_pos = self.history.len() as isize;
    }

    pub fn iter(&self) -> core::slice::Iter<'_, String> {
        self.history.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_navigation() {
        let mut history = History::new();

        let inputs = vec![
            "list",
            "give @a apple",
            "effect give @a strength 1 2",
        ];

        for input in inputs.iter() {
            history.push(input.to_string());
        }

        let mut history_iter = history.iter();
        for input in inputs.iter() {
            assert_eq!(input.to_string(), history_iter.next().unwrap().clone());
        }

        assert_eq!(history.prev().unwrap().clone(), inputs[2].to_string());
        assert_eq!(history.prev().unwrap().clone(), inputs[1].to_string());
        assert_eq!(history.prev().unwrap().clone(), inputs[0].to_string());
        assert_eq!(history.prev(), None);
        assert_eq!(history.next().unwrap().clone(), inputs[0].to_string());
        assert_eq!(history.next().unwrap().clone(), inputs[1].to_string());
        assert_eq!(history.next().unwrap().clone(), inputs[2].to_string());
        assert_eq!(history.next(), None);

        history.curr_pos = -2;
        assert_eq!(history.prev(), None);
        history.curr_pos = 5;
        assert_eq!(history.next(), None);
    }
}
