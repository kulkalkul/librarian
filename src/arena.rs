use std::mem::MaybeUninit;

pub use id::*;
pub use iter::*;

mod id {
    pub(super) type Id = u32;
    pub(super) type Generation = u32;

    #[derive(Debug)]
    pub struct ArenaId<T> {
        pub(super) id: Id,
        pub(super) generation: Generation,
        pub(super) _marker: std::marker::PhantomData<T>,
    }

    impl<T> Clone for ArenaId<T> {
        fn clone(&self) -> Self {
            Self {
                id: self.id,
                generation: self.generation,
                _marker: self._marker,
            }
        }
    }
    impl<T> Copy for ArenaId<T> {}
    impl<T> PartialEq for ArenaId<T> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id && self.generation == other.generation
        }
    }
    impl<T> Eq for ArenaId<T> {}
    impl<T> std::hash::Hash for ArenaId<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.generation.hash(state);
        }
    }
    impl<T> PartialOrd for ArenaId<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.id.cmp(&other.id))
        }
    }
    impl<T> Ord for ArenaId<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.id.cmp(&other.id)
        }
    }

    impl<T> ArenaId<T> {
        // Never use this inside Arena, this is for building Default and creating other constants
        pub(super) const DANGLING_GENERATION: u32 = 0;
        pub const DANGLING: ArenaId<T> = ArenaId {
            id: 0,
            generation: Self::DANGLING_GENERATION,
            _marker: std::marker::PhantomData,
        };
        pub fn new() -> Self {
            Self::DANGLING
        }
        pub fn id(&self) -> u32 {
            self.id
        }
    }

    impl<T> Default for ArenaId<T> {
        fn default() -> Self {
            Self::DANGLING
        }
    }
}

struct Entry<T> {
    value: MaybeUninit<T>,
    generation: Generation,
}
pub struct EntryRef<'e, T> {
    pub value: &'e T,
    pub generation: Generation,
}
pub struct EntryMut<'e, T> {
    pub value: &'e mut T,
    pub generation: Generation,
}
struct RemovedEntry {
    index: Id,
    generation: Generation,
}

/// Lookup optimized Arena
pub struct Arena<T> {
    entries: Vec<Entry<T>>,
    removed_entries: Vec<RemovedEntry>,
    count: usize,
}

impl<T> Arena<T> {
    // We should be able to differentiate between DANGLIN_GENERATION vs. TOMBSTONE
    // otherwise Self::remove can call assume_init_drop multiple times, which is UB.
    const TOMBSTONE: Generation = ArenaId::<T>::DANGLING_GENERATION + 1;
    const NEW_ENTRY: Generation = ArenaId::<T>::DANGLING_GENERATION + 2;

    pub fn new() -> Self {
        Self::with_capacity(128)
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            removed_entries: Vec::with_capacity(16),
            count: 0,
        }
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn add(&mut self, value: T) -> ArenaId<T> {
        self.count += 1;
        match self.removed_entries.pop() {
            Some(RemovedEntry { index, generation }) => {
                assert!(index <= const { u32::MAX });

                let entry = &mut self.entries[index as usize];

                let generation = generation + 1;

                *entry = Entry {
                    value: MaybeUninit::new(value),
                    generation,
                };

                ArenaId {
                    id: index as u32,
                    generation,
                    _marker: std::marker::PhantomData,
                }
            }
            None => {
                let index = self.entries.len();

                assert!(index <= const { u32::MAX as usize });

                self.entries.push(Entry {
                    value: MaybeUninit::new(value),
                    generation: Self::NEW_ENTRY,
                });

                ArenaId {
                    id: index as Id,
                    generation: Self::NEW_ENTRY,
                    _marker: std::marker::PhantomData,
                }
            }
        }
    }
    pub fn remove(&mut self, id: ArenaId<T>) {
        self.count -= 1;
        let entry = &mut self.entries[id.id as usize];
        if id.generation == entry.generation {
            // Runs drop of T
            unsafe { entry.value.assume_init_drop() }; // safe because generations match
            self.removed_entries.push(RemovedEntry {
                index: id.id,
                generation: entry.generation,
            });
            entry.generation = Self::TOMBSTONE;
        }
    }
    pub fn entry<'a>(&'a self, id: ArenaId<T>) -> EntryRef<'a, T> {
        self.try_entry(id).expect("Entry should exist in the arena")
    }
    pub fn try_entry<'a>(&'a self, id: ArenaId<T>) -> Option<EntryRef<'a, T>> {
        let entry = &self.entries[id.id as usize];
        if id.generation == entry.generation {
            Some(EntryRef {
                value: unsafe { entry.value.assume_init_ref() }, // safe because generations match
                generation: entry.generation,
            })
        } else {
            None
        }
    }
    pub fn entry_mut<'a>(&'a mut self, id: ArenaId<T>) -> EntryMut<'a, T> {
        self.try_entry_mut(id)
            .expect("Entry should exist in the arena")
    }
    pub fn try_entry_mut<'a>(&'a mut self, id: ArenaId<T>) -> Option<EntryMut<'a, T>> {
        let entry = &mut self.entries[id.id as usize];
        if id.generation == entry.generation {
            Some(EntryMut {
                value: unsafe { entry.value.assume_init_mut() }, // safe because generations match
                generation: entry.generation,
            })
        } else {
            None
        }
    }
    pub fn iter<'a>(&'a self) -> IterArena<'a, T> {
        IterArena {
            entries: self.entries.iter(),
        }
    }
    pub fn iter_mut<'a>(&'a mut self) -> IterArenaMut<'a, T> {
        IterArenaMut {
            entries: self.entries.iter_mut(),
        }
    }
    pub fn iter_ids<'a>(&'a self) -> IterArenaIds<'a, T> {
        IterArenaIds {
            entries: self.entries.iter().enumerate(),
        }
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

mod iter {
    use std::{
        iter::Enumerate,
        slice::{Iter, IterMut},
    };

    use super::{Arena, ArenaId, Entry, EntryMut, EntryRef};

    pub struct IterArena<'a, T> {
        pub(super) entries: Iter<'a, Entry<T>>,
    }

    impl<'a, T> Iterator for IterArena<'a, T> {
        type Item = EntryRef<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(entry) = self.entries.next() {
                if entry.generation == Arena::<T>::TOMBSTONE {
                    continue;
                }

                return Some(EntryRef {
                    // This is safe because we ensure data is init with the check above
                    value: unsafe { entry.value.assume_init_ref() },
                    generation: entry.generation,
                });
            }

            None
        }
    }

    pub struct IterArenaMut<'a, T> {
        pub(super) entries: IterMut<'a, Entry<T>>,
    }

    impl<'a, T> Iterator for IterArenaMut<'a, T> {
        type Item = EntryMut<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(entry) = self.entries.next() {
                if entry.generation == Arena::<T>::TOMBSTONE {
                    continue;
                }

                return Some(EntryMut {
                    // This is safe because we ensure data is init with the check above
                    value: unsafe { entry.value.assume_init_mut() },
                    generation: entry.generation,
                });
            }

            None
        }
    }

    pub struct IterArenaIds<'a, T> {
        pub(super) entries: Enumerate<Iter<'a, Entry<T>>>,
    }

    impl<'a, T> Iterator for IterArenaIds<'a, T> {
        type Item = ArenaId<T>;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some((index, entry)) = self.entries.next() {
                if entry.generation == Arena::<T>::TOMBSTONE {
                    continue;
                }

                return Some(ArenaId {
                    id: index as u32,
                    generation: entry.generation,
                    _marker: std::marker::PhantomData,
                });
            }

            None
        }
    }
}
