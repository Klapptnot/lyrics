use std::collections::HashMap;

type IrcTagsMap<'a> = HashMap<&'a str, Option<&'a str>>;

pub(crate) struct IrcInfo<'a> {
  pub(crate) user: &'a str,
  pub(crate) host: &'a str,
  pub(crate) text: &'a str,
  pub(crate) tags: IrcTagsMap<'a>,
}

pub(crate) enum TwitchIrcMsg<'a> {
  /// Sent when the bot or moderator removes all messages from the chat room or removes all messages for the specified user
  CLEARCHAT(IrcInfo<'a>),
  /// Sent when the bot removes a single message from the chat room
  CLEARMSG(IrcInfo<'a>),
  /// Sent when the bot authenticates with the server
  GLOBALUSERSTATE(IrcInfo<'a>),
  /// Sent to indicate the outcome of an action like banning a user
  NOTICE(IrcInfo<'a>),
  /// Sent when a user posts a message to the chat room
  PRIVMSG(IrcInfo<'a>),
  /// Sent when the bot joins a channel or when the channel’s chat room settings change
  ROOMSTATE(IrcInfo<'a>),
  /// Sent when events like someone subscribing to the channel occurs
  USERNOTICE(IrcInfo<'a>),
  /// Sent when the bot joins a channel or sends a PRIVMSG message
  USERSTATE(IrcInfo<'a>),
  /// Sent when someone sends your bot a whisper message
  WHISPER(IrcInfo<'a>),
  /// Nothing
  NOTHING,
}

impl TwitchIrcMsg<'static> {
  fn from(s: &mut String) -> TwitchIrcMsg {
    // Parse IRC tags if they are present
    let mut tags: IrcTagsMap = HashMap::new();
    let user: &str;
    let host: &str;
    let text: &str;
    if s.chars().next().unwrap_or('_') == '@' {
      let mut ss: String = s.drain(..1 as usize).collect();
      let eix = ss.find(' ').unwrap() + 1;
      let ts: String = ss.drain(eix..).collect();
      let ts = ts.split(';');
      for (_, it) in ts.enumerate() {
        let it = it.split('=').collect::<Vec<_>>();
        if it[1] == "" {
          tags.insert(it[0], None);
        } else {
          tags.insert(it[0], Some(it[1]));
        }
      }
    }
    // Components of user!streamer@streamer.tmi.twitch.tv
    if s.chars().next().unwrap_or('_') == ':' {
      let mut ss: String = s.drain(..1 as usize).collect();
      let eix = ss.find(' ').unwrap() + 1;
      let ts: String = ss.drain(eix..).collect();
      let ts = ts.split('!').collect::<Vec<_>>();
      user = ts[0];
      host = ts[1];
    }

    TwitchIrcMsg::NOTHING
  }
}
