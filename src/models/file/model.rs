use crate::schema::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "file_ref"]
pub(crate) struct SlimFile {
    pub(crate) uuid: Uuid,
    pub(crate) filename: String,
    pub(crate) path_file: String,
}

#[derive(Debug)]
pub(crate) struct FileMetadata {
    pub(crate) blake3_hash: Vec<u8>,
    pub(crate) sha256_hash: Vec<u8>,
    pub(crate) id_ext: i32,
}
