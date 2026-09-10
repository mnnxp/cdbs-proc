#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;
mod database;
mod errors;
mod models;
mod parser;
mod schema;
mod storage;

#[tokio::main]
async fn main() {
    // Gets enviroment variables from `.env`
    dotenv::dotenv().ok();

    // Initiates error logger
    env_logger::init();

    // Sets options to enviroment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    // getting storage access data
    let storage_access =
        storage::model::StorageAccess::from_env().expect("Failed to load S3 storage configuration");
    let aws_access = storage::s3::Aws::from(&storage_access);
    let client = rusoto_s3::S3Client::from(&aws_access);

    // Database
    let pool = database::pool::establish_connection(opt.clone());

    eprintln!("Parsing started...");

    parser::play(opt, client, storage_access.bucket, pool).await;
}
