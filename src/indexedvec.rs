pub struct IndexedVec<T> {
    index: Option<usize>,
    store: Vec<T>,
    circular: bool,
}

impl<T> Default for IndexedVec<T> {
    fn default() -> Self {
        Self {
            index: None,
            store: vec![],
            circular: false,
        }
    }
}

impl<T> IndexedVec<T> {
    pub fn simple(elements: Vec<T>) -> Self {
        Self {
            index: None,
            store: elements,
            ..Default::default()
        }
    }
    pub fn reset(&mut self) {
        self.index = None
    }
    pub fn last(&self) -> bool {
        if self.is_empty() {
            return true;
        }
        if self.circular {
            return false;
        }
        if let Some(index) = self.index {
            index == self.store.len() - 1
        } else {
            false
        }
    }
    pub fn index(&self) -> Option<usize> {
        self.index
    }
    pub fn cycle(elements: Vec<T>) -> Self {
        Self {
            index: None,
            store: elements,
            circular: true,
        }
    }
    pub fn circular(&self) -> bool {
        self.circular
    }
    pub fn next_item(&mut self) -> Option<&mut T> {
        if self.store.is_empty() {
            self.index = None;
            return None;
        }
        let Some(index) = self.index else {
            self.index = Some(0);
            return self.store.get_mut(0);
        };

        if index < self.store.len() - 1 {
            self.index = Some(index + 1);
            self.store.get_mut(index + 1)
        } else if index == self.store.len() - 1 && self.circular {
            self.index = Some(0);
            self.store.get_mut(0)
        } else {
            None
        }
    }
    pub fn previous_item(&mut self) -> Option<&mut T> {
        if self.store.is_empty() {
            self.index = None;
            return None;
        }
        if let Some(index) = self.index {
            let future_index = if index > 0 { index - 1 } else { 0 };
            self.index = Some(future_index);
            self.store.get_mut(future_index)
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.store.len()
    }
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

#[test]
fn indexedvec_default_tests() {
    let mut default = IndexedVec::<bool>::default();
    assert!(!default.circular());
    assert!(default.is_empty());
    assert!(default.index().is_none());
    assert!(default.last());
    let next = default.next_item();
    assert!(next.is_none());
    assert!(default.index().is_none());
    let next = default.next_item();
    assert!(next.is_none());
    assert!(default.index().is_none());
}

#[test]
fn indexedvec_simple_tests() {
    let mut simple = IndexedVec::<bool>::simple(vec![false, true]);
    assert!(!simple.circular());
    assert!(!simple.is_empty());
    assert!(simple.index().is_none());
    assert!(!simple.last());

    let next = simple.next_item();
    assert_eq!(next, Some(&mut false));
    assert_eq!(simple.index(), Some(0));
    assert!(!simple.last());

    let next = simple.next_item();
    assert_eq!(next, Some(&mut true));
    assert_eq!(simple.index(), Some(1));
    assert!(simple.last());

    let next = simple.next_item();
    assert!(next.is_none());
    let next = simple.next_item();
    assert!(next.is_none());

    simple.reset();
    let next = simple.next_item();
    assert_eq!(next, Some(&mut false));
    assert_eq!(simple.index(), Some(0));
    assert!(!simple.last());
}

#[test]
fn indexedvec_cycle_tests() {
    let mut circular = IndexedVec::<bool>::cycle(vec![false, true]);
    assert!(circular.circular());
    assert!(!circular.is_empty());
    assert!(circular.index().is_none());
    assert!(!circular.last());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut false));
    assert_eq!(circular.index(), Some(0));
    assert!(!circular.last());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut true));
    assert_eq!(circular.index(), Some(1));
    assert!(!circular.last());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut false));
    assert_eq!(circular.index(), Some(0));
    assert!(!circular.last());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut true));
    assert_eq!(circular.index(), Some(1));
    assert!(!circular.last());
}
