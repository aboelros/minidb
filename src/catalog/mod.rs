
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    Text,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub is_primary_key: bool,
    pub is_not_null: bool,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone)]
pub struct TableInfo {
    pub name: String,
    pub schema: Schema,
    // Typically the first page ID of this table's data
    pub first_page_id: u32,
}
