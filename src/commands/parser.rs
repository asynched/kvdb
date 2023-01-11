use std::str::FromStr;

use tonic::{Response, Status};

use crate::database::Database;
use crate::rpc::CommandResponse;
use crate::types::{ParseValueError, Value};
use tokio::sync::RwLock;

use super::del::DelCommand;
use super::exists::ExistsCommand;
use super::flushall::FlushAllCommand;
use super::get::GetCommand;
use super::incr::IncrCommand;
use super::set::SetCommand;

use super::{del, exists, flushall, get, incr, set};

#[derive(Debug)]
enum Command {
    Get(GetCommand),
    Set(SetCommand),
    Del(DelCommand),
    Incr(IncrCommand),
    Exists(ExistsCommand),
    FlushAll(FlushAllCommand),
}

#[derive(Debug)]
enum ParseCommandError {
    InvalidCommand,
    NotEnoughArguments,
    InvalidValue(ParseValueError),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.starts_with("GET") {
            let cmd = string.split(" ").collect::<Vec<_>>();

            if cmd.len() < 2 {
                return Err(ParseCommandError::NotEnoughArguments);
            }

            return Ok(Command::Get(GetCommand {
                key: cmd[1].to_string(),
            }));
        }

        if string.starts_with("SET") {
            let cmd = string.split(" ").collect::<Vec<_>>();

            if cmd.len() < 3 {
                return Err(ParseCommandError::NotEnoughArguments);
            }

            return Ok(Command::Set(SetCommand {
                key: cmd[1].into(),
                value: cmd[2..]
                    .join(" ")
                    .parse::<Value>()
                    .map_err(|_| ParseCommandError::InvalidValue(ParseValueError::InvalidValue))?,
            }));
        }

        if string.starts_with("DEL") {
            let cmd = string.split(" ").collect::<Vec<_>>();

            if cmd.len() < 2 {
                return Err(ParseCommandError::NotEnoughArguments);
            }

            return Ok(Command::Del(DelCommand {
                key: cmd[1].to_string(),
            }));
        }

        if string.starts_with("INCR") {
            let cmd = string.split(" ").collect::<Vec<_>>();

            if cmd.len() < 2 {
                return Err(ParseCommandError::NotEnoughArguments);
            }

            return Ok(Command::Incr(IncrCommand {
                key: cmd[1].to_string(),
            }));
        }

        if string.starts_with("EXISTS") {
            let cmd = string.split(" ").collect::<Vec<_>>();

            if cmd.len() < 2 {
                return Err(ParseCommandError::NotEnoughArguments);
            }

            return Ok(Command::Exists(ExistsCommand {
                key: cmd[1].to_string(),
            }));
        }

        if string.starts_with("FLUSHALL") {
            return Ok(Command::FlushAll(FlushAllCommand {}));
        }

        return Err(ParseCommandError::InvalidCommand);
    }
}

pub async fn execute(
    cmd: String,
    db: &RwLock<Database>,
) -> Result<Response<CommandResponse>, Status> {
    match cmd.parse() {
        Ok(cmd) => match cmd {
            Command::Del(cmd) => {
                let mut db = db.write().await;

                return del::execute(cmd, &mut db);
            }
            Command::Get(cmd) => {
                let db = db.read().await;

                return get::execute(cmd, &db);
            }
            Command::Set(cmd) => {
                let mut db = db.write().await;

                return set::execute(cmd, &mut db);
            }
            Command::Incr(cmd) => {
                let mut db = db.write().await;

                return incr::execute(cmd, &mut db);
            }
            Command::Exists(cmd) => {
                let db = db.read().await;

                return exists::execute(cmd, &db);
            }
            Command::FlushAll(cmd) => {
                let mut db = db.write().await;

                return flushall::execute(cmd, &mut db);
            }
        },
        Err(err) => match err {
            ParseCommandError::InvalidCommand => {
                Err(Status::invalid_argument("ERR_INVALID_COMMAND"))
            }
            ParseCommandError::NotEnoughArguments => {
                Err(Status::invalid_argument("ERR_NOT_ENOUGH_ARGUMENTS"))
            }
            ParseCommandError::InvalidValue(_) => {
                Err(Status::invalid_argument("ERR_INVALID_VALUE"))
            }
        },
    }
}
