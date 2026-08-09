use super::page::{Page, PageId, PAGE_SIZE};
use crate::error::MiniDbError;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;

pub struct DiskManager {
    file: File,
    next_page_id: PageId,
}

impl DiskManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, MiniDbError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        
        let file_len = file.metadata()?.len();
        let next_page_id = (file_len / (PAGE_SIZE as u64)) as u32;

        Ok(Self { file, next_page_id })
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        page_id
    }

    pub fn read_page(&mut self, page_id: PageId, page: &mut Page) -> Result<(), MiniDbError> {
        let offset = (page_id as u64) * (PAGE_SIZE as u64);
        self.file.seek(SeekFrom::Start(offset))?;
        
        // Ensure we handle reading potentially unwritten parts at EOF
        let mut bytes_read = 0;
        while bytes_read < PAGE_SIZE {
            let n = self.file.read(&mut page.data[bytes_read..])?;
            if n == 0 {
                // EOF reached; zero out the rest
                for b in &mut page.data[bytes_read..] {
                    *b = 0;
                }
                break;
            }
            bytes_read += n;
        }
        
        page.id = page_id;
        page.is_dirty = false;
        Ok(())
    }

    pub fn write_page(&mut self, page_id: PageId, page: &Page) -> Result<(), MiniDbError> {
        let offset = (page_id as u64) * (PAGE_SIZE as u64);
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(&page.data)?;
        self.file.sync_data()?;
        Ok(())
    }
}
