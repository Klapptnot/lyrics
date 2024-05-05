pub fn exit_err(hint: &str) -> ! {
  eprintln!("\x1b[1;31m[lyrics] {}\x1b[0m", hint);
  std::process::exit(1)
}
