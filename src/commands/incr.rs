use tonic::{Response, Status};

use crate::database::Database;
use crate::rpc::CommandResponse;
use crate::types::Value;

#[derive(Debug)]
pub struct IncrCommand {
    pub key: String,
}

pub fn execute(cmd: IncrCommand, db: &mut Database) -> Result<Response<CommandResponse>, Status> {
    if let Some(val) = db.get(&cmd.key) {
        if let Value::Int(int) = val {
            db.insert(&cmd.key, Value::Int(int + 1));

            return Ok(Response::new(CommandResponse {
                output: "OK".into(),
            }));
        }

        return Err(Status::invalid_argument("ERR_INVALID_TYPE"));
    }

    db.insert(&cmd.key, Value::Int(1));

    return Ok(Response::new(CommandResponse {
        output: "OK".into(),
    }));
}
