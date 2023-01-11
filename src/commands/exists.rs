use tonic::{Response, Status};

use crate::database::Database;
use crate::rpc::CommandResponse;
use crate::types::Value;

#[derive(Debug)]
pub struct ExistsCommand {
    pub key: String,
}

pub fn execute(cmd: ExistsCommand, db: &Database) -> Result<Response<CommandResponse>, Status> {
    let exists = db.exists(&cmd.key);

    return Ok(Response::new(CommandResponse {
        output: Value::Bool(exists).to_string(),
    }));
}
