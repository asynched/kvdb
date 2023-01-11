use tonic::{Response, Status};

use crate::database::Database;
use crate::rpc::CommandResponse;

#[derive(Debug)]
pub struct FlushAllCommand {}

pub fn execute(
    _cmd: FlushAllCommand,
    db: &mut Database,
) -> Result<Response<CommandResponse>, Status> {
    db.flush_all();

    return Ok(Response::new(CommandResponse {
        output: "OK".to_string(),
    }));
}
