use serde::Serialize;

use crate::store::Bookmark;

pub const LOCAL_FILE_VERSION: u64 = 0;

#[derive(Serialize)]
pub struct ToLocalFile<'a> {
    pub version: u64,
    pub bookmarks: Vec<&'a Bookmark>,
}
