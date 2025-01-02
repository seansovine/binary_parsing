/// Utility to read bytes from a file for parsing.
///
use std::fs::File;
use std::io::{BufRead, BufReader};

// In case we're reading a large file, we don't read it into
// memory all at once. Instead we read chunks of size BUFFER_SIZE.
const BUFFER_SIZE: usize = 64;

pub struct FileReader {
  reader: BufReader<File>,
  current_buffer: Vec<u8>,
}

impl FileReader {
  pub fn new(file: File) -> FileReader {
    FileReader {
      reader: BufReader::with_capacity(BUFFER_SIZE, file),
      current_buffer: vec![],
    }
  }

  /// Ensures our buffer contains at least `length` bytes.
  pub fn ensure_length(&mut self, length: usize) -> Result<(), String> {
    while self.current_buffer.len() < length {
      let buffer = self.reader.fill_buf().unwrap();
      self.current_buffer.extend_from_slice(buffer);

      let bytes_read = buffer.len();

      if bytes_read == 0 {
        return Err(String::from(
          "Requested file contains less than requested number of bytes.",
        ));
      }

      self.reader.consume(bytes_read);
    }

    Ok(())
  }

  pub fn buffer(&self) -> &[u8] {
    &self.current_buffer[0..]
  }
}
