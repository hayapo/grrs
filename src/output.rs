use std::io::{self, Write};

fn main() {
  let stdout = io::stdout();
  let mut handle = io::BufWriter::new(stdout);
  writeln!(handle, "foo: {}", 42).unwrap();
  progress();
}

fn progress() {
  let pb = indicatif::ProgressBar::new(100);
  for i in 0..100 {
    pb.println(format!("[+] finished #{}", i));
    pb.inc(1);
  }
  pb.finish_with_message("done");
}