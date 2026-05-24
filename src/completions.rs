// just a wrapper over a sorted vector of completions
pub(crate) struct Completions {
    available: Vec<String>,
    completion_subset: Vec<String>,
    base: String, // input before the prefix (used to detect cycling)
    iter_state: usize,
}

impl Completions {
    pub(crate) fn new(available_completions: &[String]) -> Self {
        let mut available = available_completions.to_owned();
        available.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));

        Self {
            available,
            completion_subset: Vec::new(),
            base: String::new(),
            iter_state: 0,
        }
    }

    pub(crate) fn _add(&mut self, completion: String) {
        self.available.push(completion);
        self.available
            .sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    }

    // Takes the full input buffer, returns the full replacement string (without trailing space).
    pub(crate) fn complete(&mut self, input: &str) -> Option<String> {
        let trimmed = input.trim_end_matches(' ');
        let prefix = get_prefix(trimmed);
        let base = &trimmed[..trimmed.len() - prefix.len()];

        let is_cycling = self.base == base
            && self
                .completion_subset
                .get(self.iter_state)
                .is_some_and(|c| trimmed == format!("{}{}", self.base, c));

        if is_cycling {
            self.iter_state = (self.iter_state + 1) % self.completion_subset.len();
        } else {
            self.base = base.to_string();
            self.iter_state = 0;
            self.completion_subset = self.get_completions(&prefix);
        }

        if self.completion_subset.is_empty() {
            return None;
        }

        Some(format!(
            "{}{}",
            self.base, self.completion_subset[self.iter_state]
        ))
    }

    fn get_completions(&mut self, prefix: &str) -> Vec<String> {
        self.iter_state = 0;
        self.available
            .iter()
            .filter(|c| c.starts_with(prefix))
            .cloned()
            .collect()
    }
}

pub(crate) fn get_prefix(input: &str) -> String {
    let mut chars = input.chars().rev();
    let mut prefix = String::new();
    for c in chars.by_ref() {
        if !c.is_whitespace() {
            prefix.push(c);
        } else {
            break;
        }
    }
    prefix.chars().rev().collect()
}
