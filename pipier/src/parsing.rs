use http::Uri;
use urlencoding::decode;

#[derive(thiserror::Error, Debug)]
pub enum ParsingError {
  #[error("Commands should have exactly 2 parts, this one is not parsing: {0}")]
  InvalidCommand(String),
  #[error("Error decoding: {0}")]
  DecodingError(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, PartialEq)]
pub enum Command {
  Jq(String),
  Target(String),
  // Pause(Duration) ?
  // Loop(u8) ?
  //
}

pub fn parse_args(args: &Uri) -> Result<Vec<Command>, ParsingError> {
  let mut commands = vec![];
  let command_parts = args.path().split("/").filter(|s| !s.is_empty());

  for command in command_parts {
    let mut parts = command.split(":");
    match (parts.next(), parts.next(), parts.next()) {
      (Some(command), Some(arg), None) => {
        let decoded = || decode(arg.into()).map(|s| s.to_string());
        let command = match command {
          "jq" => Command::Jq(decoded()?),
          "target" => Command::Target(decoded()?),
          _ => return Err(ParsingError::InvalidCommand(command.to_string())),
        };
        commands.push(command);
      }
      _ => return Err(ParsingError::InvalidCommand(command.to_string())),
    }
  }

  Ok(commands)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() -> anyhow::Result<()> {
    let uri: Uri = "/jq:.%5B0%5D/jq:.%5B0%5D/target:http%3A%2F%2Fgoogle.com".parse()?;
    let commands = parse_args(&uri)?;
    assert_eq!(commands, vec!(Command::Jq(".[0]".to_string()), Command::Jq(".[0]".to_string()), Command::Target("http://google.com".to_string())));
    Ok(())
  }
}
