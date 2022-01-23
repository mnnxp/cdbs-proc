use crate::schema::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Insertable)]
#[table_name = "file_ref"]
pub(crate) struct InsertableFile {
    pub(crate) uuid: Uuid,
    pub(crate) parent_file_uuid: Uuid,
    pub(crate) hash: Vec<u8>,
    pub(crate) user_uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) content_type: String,
    pub(crate) id_ext: i32,
    pub(crate) filesize: i64,
    pub(crate) path_file: String,
    pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "file_ref"]
pub(crate) struct SlimFile {
    pub(crate) uuid: Uuid,
    pub(crate) filename: String,
    // pub(crate) filesize: i64,
    pub(crate) path_file: String,
}

#[derive(Debug)]
pub(crate) struct FileMetadata {
    pub(crate) hash: Vec<u8>,
    pub(crate) id_ext: i32,
    // pub(crate) filesize: i32,
}
