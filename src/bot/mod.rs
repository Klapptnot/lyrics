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

  // Here we could implement a IRC Parser for Twitch
  // https://dev.twitch.tv/docs/irc/tags/

  while let Some(Ok(ln)) = lines.next() {
    let msg = ln[1..].splitn(2, ':').nth(1).unwrap();
    macros::log_inf!("Message: {}", msg);
  }
}
