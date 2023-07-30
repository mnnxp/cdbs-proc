use crate::schema::*;
use uuid::Uuid;

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
