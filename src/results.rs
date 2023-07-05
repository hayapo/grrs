fn main() {
  let result = std::fs::read_to_string("src/main.rs");
  let content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into());}
  };
  println!("file content: {}", content);
  Ok(())
}