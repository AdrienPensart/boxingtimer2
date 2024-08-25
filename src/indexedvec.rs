use derive_more::Deref;
use rand::seq::SliceRandom;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Deref)]
pub struct IndexedVec<T> {
    index: Option<usize>,
    #[deref]
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

impl<T> IndexedVec<T>
where
    T: std::clone::Clone + std::cmp::PartialEq,
{
    pub fn simple(elements: &[T]) -> Self {
        Self {
            index: None,
            store: Vec::from(elements),
            ..Default::default()
        }
    }
    pub fn cycle(elements: &[T]) -> Self {
        Self {
            index: None,
            store: Vec::from(elements),
            circular: true,
        }
    }
    pub fn apply<F: FnMut(&mut T)>(&mut self, f: F) {
        self.store.iter_mut().for_each(f)
    }
    pub fn reset(&mut self) {
        self.index = None
    }
    pub fn last(&self) -> bool {
        if self.is_empty() {
            return true;
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
    pub fn set_index(&mut self, index: usize) -> Option<&mut T> {
        if index < self.store.len() {
            self.index = Some(index);
        } else {
            self.index = None;
        }
        self.store.get_mut(index)
    }
    pub fn circular(&self) -> bool {
        self.circular
    }
    pub fn retain(&mut self, value: &T) {
        self.store.retain(|i| i != value);
        self.index = None;
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.store.shuffle(&mut rng);
    }
    pub fn shuffled(&mut self) -> Vec<T> {
        let mut rng = rand::thread_rng();
        let mut store = self.store.clone();
        store.shuffle(&mut rng);
        store
    }
    pub fn get(&self) -> Option<&T> {
        if let Some(index) = self.index {
            self.store.get(index)
        } else {
            None
        }
    }
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if let Some(index) = self.index {
            self.store.get_mut(index)
        } else {
            None
        }
    }
    pub fn next_item(&mut self) -> Option<&mut T> {
        if self.store.is_empty() {
            self.index = None;
            return None;
        }

        let index = match self.index {
            None => {
                self.index = Some(0);
                return self.store.get_mut(0);
            }
            Some(index) => index,
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
    assert!(default.get().is_none());
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
    let mut simple = IndexedVec::<bool>::simple(&[false, true]);
    assert!(!simple.circular());
    assert!(!simple.is_empty());
    assert!(simple.index().is_none());
    assert!(simple.get().is_none());
    assert!(!simple.last());

    let next = simple.next_item();
    assert_eq!(next, Some(&mut false));
    assert_eq!(simple.index(), Some(0));
    assert_eq!(simple.get(), Some(&false));
    assert!(!simple.last());

    let next = simple.next_item();
    assert_eq!(next, Some(&mut true));
    assert_eq!(simple.index(), Some(1));
    assert_eq!(simple.get(), Some(&true));
    assert!(simple.last());

    assert!(simple.next_item().is_none());
    // assert!(simple.get().is_none());
    assert_eq!(simple.get(), Some(&true));

    simple.reset();
    assert_eq!(simple.next_item(), Some(&mut false));
    assert_eq!(simple.index(), Some(0));
    assert_eq!(simple.get(), Some(&false));
    assert!(!simple.last());
}

#[test]
fn indexedvec_cycle_tests() {
    let mut circular = IndexedVec::<bool>::cycle(&[false, true]);
    assert!(circular.circular());
    assert!(!circular.is_empty());
    assert!(circular.index().is_none());
    assert!(circular.get().is_none());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut false));
    assert_eq!(circular.index(), Some(0));
    assert_eq!(circular.get(), Some(&false));
    assert!(!circular.last());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut true));
    assert_eq!(circular.index(), Some(1));
    assert_eq!(circular.get(), Some(&true));
    assert!(circular.last());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut false));
    assert_eq!(circular.index(), Some(0));
    assert_eq!(circular.get(), Some(&false));
    assert!(!circular.last());

    let next = circular.next_item();
    assert_eq!(next, Some(&mut true));
    assert_eq!(circular.index(), Some(1));
    assert_eq!(circular.get(), Some(&true));
    assert!(circular.last());
}
