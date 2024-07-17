use core::hash;
use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    arena::{Arena, ArenaId, IterArenaIds},
    bit_field::BitField,
};

#[derive(Clone)]
struct Interned {
    rc: Rc<str>,
    id: InternedId,
}

impl PartialEq for Interned {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Interned {}
impl hash::Hash for Interned {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternedId {
    inner: u64,
}

impl InternedId {
    pub const DANGLING: InternedId = InternedId { inner: 0 };
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub title: Rc<str>,
    pub link: Rc<str>,
    pub note: Rc<str>,
}

struct TagContainer {
    tags: BitField,
    tag_count: usize,
}

pub struct Store {
    next_intern_id: u64,
    interned: Vec<Rc<str>>,
    reverse_interned: HashMap<String, Interned>,
    bookmarks: Arena<Bookmark>,
    filtered_items: Vec<Bookmark>,
    tags: HashMap<InternedId, TagContainer>,
    changes: Vec<ArenaId<Bookmark>>,
}

impl Store {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn intern(&mut self, string: &str) -> InternedId {
        match self.reverse_interned.get(string) {
            Some(interned) => interned.id,
            None => {
                let id = InternedId {
                    inner: self.next_intern_id,
                };
                let rc: Rc<str> = Rc::from(string);

                self.next_intern_id += 1;
                self.interned.push(rc.clone());
                self.reverse_interned
                    .insert(string.to_owned(), Interned { rc, id });

                id
            }
        }
    }
    pub fn string(&mut self, id: &InternedId) -> &Rc<str> {
        &self.interned[id.inner as usize]
    }
    pub fn create_tag(&mut self, id: InternedId) {
        let tags = BitField::init(self.bookmarks.len());
        // Currently overwrites, handle sometime
        self.tags.insert(id, TagContainer { tags, tag_count: 0 });
    }
    pub fn remove_tag(&mut self, id: InternedId) {
        self.tags.remove(&id);
    }
    pub fn add_bookmark(&mut self, bookmark: Bookmark) -> ArenaId<Bookmark> {
        let id = self.bookmarks.add(bookmark);
        self.filtered_items.reserve(1);
        id
    }
    pub fn create_bookmark(&mut self, title: &str, link: &str, note: &str) -> ArenaId<Bookmark> {
        let id = self.add_bookmark(Bookmark {
            title: Rc::from(title),
            link: Rc::from(link),
            note: Rc::from(note),
        });
        self.changes.push(id);

        id
    }
    pub fn bookmark(&self, id: ArenaId<Bookmark>) -> &Bookmark {
        self.bookmarks.entry(id).value
    }
    pub fn all<'a>(&'a self) -> IterArenaIds<'a, Bookmark> {
        self.bookmarks.iter_ids()
    }
    pub fn changes<'a>(
        &'a mut self,
    ) -> Option<impl Iterator<Item = (ArenaId<Bookmark>, &'a Bookmark)>> {
        if self.changes.is_empty() {
            None
        } else {
            let changes = self
                .changes
                .drain(..)
                .map(|id| (id, self.bookmarks.entry(id).value));

            Some(changes)
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self {
            next_intern_id: InternedId::DANGLING.inner + 1,
            interned: vec![Rc::from("DANGLING")],
            reverse_interned: HashMap::with_capacity(128),
            bookmarks: Arena::with_capacity(1024),
            filtered_items: Vec::with_capacity(1024),
            tags: HashMap::with_capacity(64),
            changes: Vec::with_capacity(128),
        }
    }
}
