use derive_more::{Deref, IntoIterator};
use rand::seq::SliceRandom;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Deref, IntoIterator)]
pub struct IndexedVec<T> {
    index: Option<usize>,
    #[deref]
    #[into_iterator(ref)]
    store: Vec<T>,
}

impl<T> Default for IndexedVec<T> {
    fn default() -> Self {
        Self {
            index: None,
            store: vec![],
        }
    }
}

impl<T> IndexedVec<T>
where
    T: std::clone::Clone + std::cmp::PartialEq,
{
    pub fn new(elements: &[T]) -> Self {
        Self {
            index: None,
            store: Vec::from(elements),
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
        } else {
            self.index = None;
            None
        }
    }
    pub fn previous_item(&mut self) -> Option<&mut T> {
        if self.store.is_empty() {
            self.index = None;
            return None;
        }
        let future_index = if let Some(index) = self.index {
            if index == 0 {
                self.store.len() - 1
            } else {
                index - 1
            }
        } else {
            self.store.len() - 1
        };

        self.index = Some(future_index);
        self.store.get_mut(future_index)
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
    let mut simple = IndexedVec::<bool>::new(&[false, true]);
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

    let next = simple.next_item();
    assert!(next.is_none());
    assert!(simple.index().is_none());
    assert!(simple.get().is_none());

    simple.reset();
    assert_eq!(simple.next_item(), Some(&mut false));
    assert_eq!(simple.index(), Some(0));
    assert_eq!(simple.get(), Some(&false));
    assert!(!simple.last());
}
