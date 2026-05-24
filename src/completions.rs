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

    pub(crate) fn reset(&mut self) {
        self.completion_subset.clear();
        self.base.clear();
        self.iter_state = 0;
    }

    pub(crate) fn _add(&mut self, completion: String) {
        self.available.push(completion);
        self.available
            .sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    }

    pub(crate) fn get_all_completions(&mut self, prefix: &str) -> Vec<String> {
        self.iter_state = 0;
        self.available
            .iter()
            .filter(|c| c.starts_with(prefix))
            .cloned()
            .collect()
    }
}
