use tonic::{Response, Status};

use crate::database::Database;
use crate::rpc::CommandResponse;

#[derive(Debug)]
pub struct DelCommand {
    pub key: String,
}

pub fn execute(cmd: DelCommand, db: &mut Database) -> Result<Response<CommandResponse>, Status> {
    db.remove(&cmd.key);

    return Ok(Response::new(CommandResponse {
        output: "OK".to_string(),
    }));
}
