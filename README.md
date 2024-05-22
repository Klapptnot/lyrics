# **klapptnot/lyrics Rust CLI (lyrics) 💜**

This Rust CLI is your one-stop shop for finding lyrics and managing your Twitch stream music like a pro!  

## **Search, Display and Save Lyrics**

* Find lyrics for any song with the `-s` or `--search` flag 
* Prefer a specific URL? Use `-u` or `--url` to grab lyrics from there instead 
* Bad connection? Set a custom timeout for requests with `-t` or `--timeout` ⏱️
* Need a few extra tries? Control retries with `-T` or `--tries` 
* Want data from other song? Choose the URL you want with `-a` or `--tip-url` 
* Just the lyrics, please? Use `-l` or `--lyrics` to focus on the music 
* Want to repeat the artist and track before each verse? `-r` or `--repeat` has you covered 
* Feeling lost? Get help with all the options using `-h` or `--help` 

## ***Coming soon!***

### **Tag Your Tunes:**

Organize your music library with song tagging, even for m3u playlists!.

### **Twitch Control:**

Control stream music with the power of the Spotify API via a `lyrics.json`
  * **Command Central**
    * `!playlist`: Get the current queue playlist URL (remember, it's temporary!) 
    * `!song`: See the name, artist, Spotify URL, and a MusiXMatch link for the current song 
    * `!play {spotify_song_id}|{name artist song info}`: Add a song to the queue and get notified if it works! ➕
    * `!queue`: Peek at the next song in line 
    * `!lyr` (for the chosen few): Configure things on the fly
      * `!lyr commands [+-]{command_name}`: Enable or disable specific commands (use `+name` to enable, `-name` to disable) ➕➖
      * `!lyr queue [chat|host]`: Decide who can manage the queue (chat or host?) 
      * `!lyr prev/play/pause/next`: Control playback with ease (settings in `lyrics.toml`) ⏪⏯️⏩
      * `!lyr revert`: Oops! Remove the last song you added ⏪
      * `!lyr timeout {user} {minutes}`: Put a user on timeout for a while (negative minutes last until restart) 
      * `!lyr play {spotify_list_id}`: Feeling like a playlist? Play a specific one from Spotify! 
      * `!lyr format [song|playlist|queue] {format}`: Modify the reply format for specific commands (e.g., `!lyr format song {artist} - {title}`) 
      * `!lyr polls [on|off]`: Enable or disable interactive emote polls for song selection ☑️
      * `!lyr tricks [on|off]`: Turn on or off easter eggs triggered by your actions! 

  * **Easter Eggs :** Keep an eye out for these surprise events triggered by your actions! 
  * **Poll Party :** Chat can vote on what song to play or repeat with interactive emote polls!

We're still under development, but with these features, you'll be a lyrics master and Twitch music legend in no time! 

**Let us know what you think!**  This project is all about making your music experience better, so feel free to share your ideas and feedback.
