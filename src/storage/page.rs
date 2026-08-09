pub const PAGE_SIZE: usize = 4096;

pub type PageId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordId {
    pub page_id: PageId,
    pub slot_num: u16,
}

/// A generic 4KB page
#[derive(Clone)]
pub struct Page {
    pub id: PageId,
    pub data: [u8; PAGE_SIZE],
    pub is_dirty: bool,
}

impl Page {
    pub fn new(id: PageId) -> Self {
        Self {
            id,
            data: [0; PAGE_SIZE],
            is_dirty: false,
        }
    }
}

impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("id", &self.id)
            .field("is_dirty", &self.is_dirty)
            .finish() // Exclude data array for brevity
    }
}
