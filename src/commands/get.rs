use tonic::{Response, Status};

use crate::database::Database;
use crate::rpc::CommandResponse;
use crate::types::Value;

#[derive(Debug)]
pub struct GetCommand {
    pub key: String,
}

pub fn execute(cmd: GetCommand, db: &Database) -> Result<Response<CommandResponse>, Status> {
    if let Some(val) = db.get(&cmd.key) {
        return Ok(Response::new(CommandResponse {
            output: val.to_string(),
        }));
    }

    return Ok(Response::new(CommandResponse {
        output: Value::Nil.to_string(),
    }));
}
