pub struct CartesianIterator<T> {
    pub lists: Vec<Vec<Vec<T>>>,
    pub indices_lists: Vec<usize>,
    pub overflow: bool,
}

impl<T> CartesianIterator<T> {
    #[must_use]
    pub fn new(lists: Vec<Vec<Vec<T>>>) -> Self {
        let indices_lists_size = lists.len();
        let indices_lists = vec![0; indices_lists_size];
        Self {
            lists,
            indices_lists,
            overflow: false,
        }
    }

    fn increase_index(&mut self) -> bool {
        for index_index in (0..self.indices_lists.len()).rev() {
            if self.overflow {
                return false;
            }
            if self.indices_lists[index_index] < self.lists[index_index].len() - 1 {
                self.indices_lists[index_index] += 1;
                break;
            }
            self.indices_lists[index_index] = 0;
            if index_index == 0 && self.indices_lists[index_index] == 0 {
                self.overflow = true;
            }
        }
        true
    }
}

impl<T: Clone> Iterator for CartesianIterator<T> {
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut return_value = Vec::new();
        for index in (0..self.lists.len()).rev() {
            return_value.push(self.lists[index][self.indices_lists[index]].clone());
        }
        if !self.increase_index() {
            return None;
        }
        Some(return_value)
    }
}
