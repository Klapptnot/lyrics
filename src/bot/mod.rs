mod irc;
mod twitch;
// mod spotify;

use crate::any::macros;
use std::io::BufRead;

pub(crate) fn bot(channel: String, username: String, token: String) {
  let mut twitch = twitch::TwitchHandler::new(channel, username);
  twitch.login(token);

  let reader = twitch.get_reader();
  let mut lines = reader.lines();

  while let Some(Ok(ln)) = lines.next() {
    let msg = irc::parse_message(&ln.as_str());
    macros::log_inf!("Message: {:#?}", msg);
  }
}
