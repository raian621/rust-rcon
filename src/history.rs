use std::rc::Rc;

pub struct History {
    history: Vec<Rc<String>>,
    index: isize
}

impl History {
    pub fn iter(&self) -> core::slice::Iter<Rc<String>> {
        self.history.iter()
    }

    pub fn new() -> Self {
        Self { history: Vec::new(), index: 0 }
    }

    pub fn push(&mut self, cmd: String) {
        self.history.push(Rc::new(cmd));
        self.index = self.history.len() as isize;
    }

    pub fn prev(&mut self) -> Option<Rc<String>> {
        if self.index >= 0 {
            self.index -= 1;
            return match self.index {
                -1 => None,
                idx => Some(self.history[idx as usize].clone())
            };
        }

        None
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for History {
    type Item = Rc<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.history.len() as isize {
            self.index += 1;
            if self.index == self.history.len() as isize {
                return None;
            } else {
                return Some(self.history[self.index as usize].clone());
            }
        }

        None
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
            assert_eq!(input.to_string(), history_iter.next().unwrap().as_ref().clone());
        }

        assert_eq!(history.prev().unwrap().as_ref().clone(), inputs[2].to_string());
        assert_eq!(history.prev().unwrap().as_ref().clone(), inputs[1].to_string());
        assert_eq!(history.prev().unwrap().as_ref().clone(), inputs[0].to_string());
        assert_eq!(history.prev(), None);
        assert_eq!(history.next().unwrap().as_ref().clone(), inputs[0].to_string());
        assert_eq!(history.next().unwrap().as_ref().clone(), inputs[1].to_string());
        assert_eq!(history.next().unwrap().as_ref().clone(), inputs[2].to_string());
        assert_eq!(history.next(), None);

        history.index = -2;
        assert_eq!(history.prev(), None);
        history.index = 5;
        assert_eq!(history.next(), None);
    }
}
