use crate::errors::ServiceResult;
use crate::database::{PgPool, db_connection};
use crate::models::file::model::SlimFile;
use crate::models::file::service::delete::delete_file;
use crate::models::file::service::update::update_metadata;
use crate::models::file::util::set_skip_file;
use crate::models::user::delete::clear_removed_users;
use crate::cli_args::Opt;
use tokio::time::{sleep, Duration};
use rusoto_s3::S3Client;
use futures::join;

/// Parsing and delete files
/// if not found files for action - return true
pub(crate) async fn play(
    opt: Opt,
    client: S3Client,
    bucket: String,
    pool: PgPool,
) {
    let conn = db_connection(&pool).expect("failed get conn");

    loop {
        let game_meta = metadata_parser(&opt, &client, &bucket, &pool);
        let game_destoy = destroy_parser(&opt, &client, &bucket, &pool);

        match join!(game_meta, game_destoy) {
            (Ok(x), Ok(y)) => {
                debug!("game_meta {}", x);
                debug!("game_destoy {}", y);

                // start clear removed users of database
                clear_removed_users(&conn);

                // start sleeping set time if not found files for action
                if x || y {
                    sleep(Duration::from_millis(opt.sleeping_time)).await;
                    println!("{} ms have elapsed", opt.sleeping_time);
                }
            },
            (meta, destoy) => {
                debug!("Have error:");
                debug!("game_meta {:?}, ", meta);
                debug!("game_destoy {:?}", destoy);
            },
        }
    }
}

/// Get and delete files with flag is_delete
/// return true if not found files for deleting
async fn destroy_parser(
    opt: &Opt,
    client: &S3Client,
    bucket: &str,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = db_connection(pool).expect("failed get conn");

    loop {
        // get part files for delete
        let destroy_list = SlimFile::get_for_delete(&opt.limit_part, &conn)?;

        match destroy_list.is_empty() {
            true => {
                debug!("not found files for delete");
                return Ok(true);
            },
            false => {
                for slim_file in destroy_list {
                    let res = delete_file(client, bucket, &slim_file, pool).await;
                    debug!("delete file {:?} ({:?}): {:?}", slim_file.filename, slim_file.uuid, res);
                }
            },
        }
    }
}

/// Get and parsing files for set hash and extension
/// return true if not found files for parsing
async fn metadata_parser(
    opt: &Opt,
    client: &S3Client,
    bucket: &str,
    pool: &PgPool,
) -> ServiceResult<bool> {
    let conn = db_connection(pool).expect("failed get conn");

    loop {
        // get part files for delete
        let parsing_list = SlimFile::get_without_hash(&opt.limit_part, &conn)?;

        match parsing_list.is_empty() {
            true => {
                debug!("not found files for parsing");
                return Ok(true);
            },
            false => {
                for slim_file in parsing_list {
                    let res = update_metadata(
                        client,
                        bucket,
                        &opt.buffer_capacity,
                        &slim_file,
                        pool
                    ).await;
                    debug!("parsing file {:?} ({:?}): {:?}", slim_file.filename, slim_file.uuid, res);
                    if res.is_err() {
                        set_skip_file(&slim_file.uuid, &conn)?;
                    }
                }
            },
        }
    }
}
