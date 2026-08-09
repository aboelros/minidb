use super::page::{Page, PAGE_SIZE, RecordId};
use crate::error::MiniDbError;

/// Slotted Page Layout:
/// ---------------------------------------------------------
/// | PageId (4) | LSN (4) | FreeSpacePtr (2) | NumSlots (2) | -> Header (12 bytes)
/// ---------------------------------------------------------
/// | Tuple0_Offset (2) | Tuple0_Size (2) | -> Slot 0
/// ---------------------------------------------------------
/// | Tuple1_Offset (2) | Tuple1_Size (2) | -> Slot 1
/// ---------------------------------------------------------
/// | ... Free Space ...                                    |
/// ---------------------------------------------------------
/// | Tuple 1 Data | Tuple 0 Data                           |
/// ---------------------------------------------------------
pub const PAGE_HEADER_SIZE: usize = 12;

pub struct SlottedPage<'a> {
    page: &'a mut Page,
}

impl<'a> SlottedPage<'a> {
    pub fn new(page: &'a mut Page) -> Self {
        Self { page }
    }
    
    pub fn init(&mut self) {
        self.set_free_space_ptr(PAGE_SIZE as u16);
        self.set_num_slots(0);
    }
    
    pub fn num_slots(&self) -> u16 {
        let bytes: [u8; 2] = self.page.data[10..12].try_into().unwrap();
        u16::from_le_bytes(bytes)
    }
    
    fn set_num_slots(&mut self, n: u16) {
        self.page.data[10..12].copy_from_slice(&n.to_le_bytes());
    }
    
    pub fn free_space_ptr(&self) -> u16 {
        let bytes: [u8; 2] = self.page.data[8..10].try_into().unwrap();
        u16::from_le_bytes(bytes)
    }
    
    fn set_free_space_ptr(&mut self, ptr: u16) {
        self.page.data[8..10].copy_from_slice(&ptr.to_le_bytes());
    }
    
    pub fn insert_tuple(&mut self, tuple_data: &[u8]) -> Result<RecordId, MiniDbError> {
        let num_slots = self.num_slots();
        let free_space = self.free_space_ptr();
        let slot_dir_end = PAGE_HEADER_SIZE + (num_slots as usize * 4);
        
        // Check if there is enough space (data size + 4 bytes for the new slot)
        if slot_dir_end + 4 + tuple_data.len() > free_space as usize {
            return Err(MiniDbError::StorageError("Not enough space on page".into()));
        }
        
        let new_free_ptr = free_space - (tuple_data.len() as u16);
        self.set_free_space_ptr(new_free_ptr);
        
        // Write tuple data
        let start = new_free_ptr as usize;
        let end = start + tuple_data.len();
        self.page.data[start..end].copy_from_slice(tuple_data);
        
        // Write slot (offset, size)
        let slot_offset = PAGE_HEADER_SIZE + (num_slots as usize * 4);
        self.page.data[slot_offset..slot_offset + 2].copy_from_slice(&new_free_ptr.to_le_bytes());
        self.page.data[slot_offset + 2..slot_offset + 4].copy_from_slice(&(tuple_data.len() as u16).to_le_bytes());
        
        self.set_num_slots(num_slots + 1);
        self.page.is_dirty = true;
        
        Ok(RecordId {
            page_id: self.page.id,
            slot_num: num_slots,
        })
    }
    
    pub fn get_tuple(&self, slot_num: u16) -> Result<Vec<u8>, MiniDbError> {
        if slot_num >= self.num_slots() {
            return Err(MiniDbError::StorageError("Invalid slot number".into()));
        }
        
        let slot_offset = PAGE_HEADER_SIZE + (slot_num as usize * 4);
        let offset_bytes: [u8; 2] = self.page.data[slot_offset..slot_offset + 2].try_into().unwrap();
        let size_bytes: [u8; 2] = self.page.data[slot_offset + 2..slot_offset + 4].try_into().unwrap();
        
        let offset = u16::from_le_bytes(offset_bytes);
        let size = u16::from_le_bytes(size_bytes);
        
        // If size is 0, it means it's deleted. (Tombstone pattern)
        if size == 0 {
            return Err(MiniDbError::StorageError("Tuple deleted".into()));
        }
        
        let start = offset as usize;
        let end = start + size as usize;
        Ok(self.page.data[start..end].to_vec())
    }
    
    pub fn delete_tuple(&mut self, slot_num: u16) -> Result<(), MiniDbError> {
        if slot_num >= self.num_slots() {
            return Err(MiniDbError::StorageError("Invalid slot number".into()));
        }
        let slot_offset = PAGE_HEADER_SIZE + (slot_num as usize * 4);
        
        // Set size to 0 to mark as deleted
        self.page.data[slot_offset + 2..slot_offset + 4].copy_from_slice(&0u16.to_le_bytes());
        self.page.is_dirty = true;
        Ok(())
    }
}
