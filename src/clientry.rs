use crate::macros;
use crate::mxm::{MxmAPI, TrackInfo};
use crate::user_agents;
use clap::Parser;
use reqwest::header::HeaderMap;

#[derive(Parser)]
struct Args {
  ///The search string to find the music
  #[clap(short = 's', long = "search", default_value = None)]
  keyword: Option<String>,
  ///Url to use instead of searching one
  #[clap(short = 'u', long = "url", default_value = None)]
  url: Option<String>,
  ///Timeout timeout in milliseconds
  #[clap(short = 't', long = "timeout", default_value = "5000")]
  timeout: u32,
  ///Number of trying attempts to get data
  #[clap(short = 'T', long = "tries", default_value = "5")]
  tries: u32,
  ///URL index, use -a to view all URLs
  #[clap(short = 'i', long = "url-index", default_value = "0")]
  url_index: usize,
  // ---------------------------------------
  // Changes to MxmAPI needed
  //
  // ///Proxy address to use
  // #[clap(short = 'p', long = "proxy", default_value = None)]
  // proxy: Option<String>,
  // ///Proxy list file to read from
  // #[clap(short = 'P', long = "proxylist", default_value = None)]
  // proxylist: Option<String>,
  // ///Cookie string for musixmatch to bypass captcha
  // #[clap(short = 'm', long = "mxm-cookies", default_value = None)]
  // mxm_cookie: Option<String>,
  // ---------------------------------------
  ///User agent string
  #[clap(short = 'U', long = "user-agent", default_value = None)]
  user_agent: Option<String>,
  // ///Show all URL found in case default one is not correct
  // #[clap(short = 'a', long = "all-urls", default_value = "false")]
  // all_urls: bool,
  ///Only print lyrics (With -r is a bit different)
  #[clap(short = 'l', long = "lyrics", default_value = "false")]
  only_lyrics: bool,
  ///Print <Track name> - <Artist> before printing lyrics
  #[clap(short = 'r', long = "repeat", default_value = "false")]
  repeat: bool,
}

/// The entry to the old CLI functionality
pub fn cli() {
  let args = Args::parse();

  // Parse some command line arguments items as groups
  if args.keyword.is_none() && args.url.is_none() {
    macros::exit_err("You must specify a url or keyword to get a url");
  } else if !args.keyword.is_none() && !args.url.is_none() {
    macros::exit_err("Cannot specify a url and keyword at the same time");
  }

  if args.tries == 0 {
    macros::exit_err("--tries/-T cannot accept 0");
  }

  let mut headers = HeaderMap::new();
  if args.user_agent.is_none() {
    headers.insert(
      reqwest::header::USER_AGENT,
      user_agents::random().parse().unwrap(),
    );
  } else {
    headers.insert(
      reqwest::header::USER_AGENT,
      args.user_agent.unwrap().parse().unwrap(),
    );
  }

  let mxm_api = MxmAPI::new(args.tries, args.timeout, Some(headers));
  let track: TrackInfo;

  if !args.keyword.is_none() {
    track = mxm_api.get_from_keywords(&args.keyword.unwrap(), args.url_index);
  } else {
    track = mxm_api.get_from_url(&args.url.unwrap());
  }

  // let track = TrackInfo::from(crate::dummy::get_json()).unwrap_or_else(|| macros::exit_err("Not able to get TrackInfo"));

  if args.only_lyrics {
    if !track.has_lyrics {
      macros::exit_err("This song has no lyrics or lyrics are not available");
    }
    if args.repeat {
      println!(
        "\x1b[38;2;195;79;230m{}\x1b[38;2;223;225;255m - \x1b[38;2;189;147;249m{}\x1b[0m\n",
        track.name, track.artist
      );
    }
    // This has no custom color to use it in pipes properly
    println!("{}", track.lyrics);
    std::process::exit(0);
  }

  // Defaults to all relevant info
  print!(
    "\x1b[38;2;255;169;140mTITLE     : \x1b[38;2;255;232;184m{}\n\x1b[38;2;255;169;140mARTIST    : \x1b[38;2;255;232;184m{}\n\x1b[38;2;255;169;140mALBUM     : \x1b[38;2;255;232;184m{}\n\x1b[38;2;255;169;140mGENRE     : \x1b[38;2;255;232;184m{}\n\x1b[38;2;255;169;140mRELEASED  : \x1b[38;2;255;232;184m{}\n\x1b[38;2;255;169;140mSPOTIFY   : \x1b[38;2;255;232;184m{}\n\x1b[38;2;255;169;140mMUSIXMATCH: \x1b[38;2;255;232;184m{}\n\n\x1b[38;2;255;169;140mLYRICS\x1b[0m\n\n",
    track.name, track.artist, track.album, track.genre, track.released, track.spotify, track.musixmatch
  );

  if !track.has_lyrics {
    print!("Lyrics are not available :(");
    std::process::exit(0);
  }

  if !track.has_lyrics_struct {
    print!("{}\n\nCopyright -> {}\n", track.lyrics, track.lyrics_copyright);
    std::process::exit(0)
  }
  for paragraph in track.lyrics_struct {
    println!("\x1b[38;2;189;147;249m#[section({})]\x1b[0m", paragraph.title);
    for line in paragraph.lines {
      print!("{}\n", line);
    }
    print!("\n\n")
  }
  // For now, it has a trailing '\n'
  print!("Copyright -> {}", track.lyrics_copyright);
}
