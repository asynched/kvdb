use tonic::{Response, Status};

use crate::database::Database;
use crate::rpc::CommandResponse;
use crate::types::Value;

#[derive(Debug)]
pub struct SetCommand {
    pub key: String,
    pub value: Value,
}

pub fn execute(cmd: SetCommand, db: &mut Database) -> Result<Response<CommandResponse>, Status> {
    db.insert(&cmd.key, cmd.value);

    return Ok(Response::new(CommandResponse {
        output: "OK".to_string(),
    }));
}
