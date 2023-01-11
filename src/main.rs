pub mod commands;
pub mod database;
pub mod types;

use clap::{arg, Parser};
use database::Database;
use std::sync::Arc;

use tokio;
use tonic;

use tokio::sync::RwLock;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use rpc::command_server::{Command, CommandServer};
use rpc::{CommandRequest, CommandResponse};

pub mod rpc {
    tonic::include_proto!("rpc");
}

/// Command line arguments for the server.
/// You can set the server address and the
/// number of shards available to store
/// the data.
#[derive(Parser, Debug)]
struct Args {
    /// Network address for the server
    #[arg(short, long, default_value_t = String::from("127.0.0.1:7890"))]
    addr: String,
    /// Number of shards for the database
    #[arg(short, long, default_value_t = 16)]
    shards: usize,
    /// Initial size of each shard
    #[arg(short, long, default_value_t = 4096)]
    reserve: usize,
}

#[derive(Debug)]
pub struct CommandService {
    database: Arc<RwLock<Database>>,
}

#[tonic::async_trait]
impl Command for CommandService {
    async fn execute(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        let req = request.into_inner();
        return commands::parser::execute(req.command, &self.database.clone()).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!(
        "Creating a new in-memory database with {} shards of {} bytes each",
        args.shards, args.reserve
    );

    let database = Arc::new(RwLock::new(Database::new(args.shards, args.reserve)));

    println!("Initializing server on address: '{}'", args.addr);

    Server::builder()
        .add_service(CommandServer::new(CommandService {
            database: database.clone(),
        }))
        .serve(
            args.addr
                .parse()
                .expect("Couldn't parse the given address into a valid network address"),
        )
        .await?;

    Ok(())
}
