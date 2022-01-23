use crate::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "extension_ref"]
pub(crate) struct InsertableExtension {
    pub(crate) extension: String,
    pub(crate) program_id: i32,
}
