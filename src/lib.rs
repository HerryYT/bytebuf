#![deny(clippy::all)]

use napi::bindgen_prelude::Uint8Array;
use napi::{bindgen_prelude::Buffer, Error, Status};
use napi::Status::GenericFailure;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct ByteBuf {
  buf: Vec<u8>,
  r_pos: usize,
  w_pos: usize,
}

#[napi]
impl ByteBuf {
  #[napi(constructor)]
  pub fn new(buf: Option<Buffer>) -> Self {
    let vec = buf.map_or(Vec::new(), |b| b.to_vec());
    ByteBuf {
      w_pos: vec.len(),
      buf: vec,
      r_pos: 0,
    }
  }

  #[napi(factory)]
  pub fn with_initial_capacity(initial_capacity: u32) -> Self {
    ByteBuf {
      buf: Vec::with_capacity(initial_capacity as usize),
      r_pos: 0,
      w_pos: 0,
    }
  }

  #[napi(factory)]
  pub fn from_byte_array(byte_array: Vec<u8>) -> Self {
    ByteBuf {
      w_pos: byte_array.len(),
      buf: byte_array,
      r_pos: 0,
    }
  }

  #[napi]
  pub fn clear(&mut self) {
    self.buf.clear();
    self.r_pos = 0;
    self.w_pos = 0;
  }

  /// Returns the number of bytes this buffer can contain
  #[napi]
  pub fn get_capacity(&self) -> u32 {
    self.buf.capacity() as u32
  }

  /// u32 is enough, i64 is too much even for general use
  #[napi]
  pub fn set_capacity(&mut self, size: u32) {
    if (size as usize) < self.buf.capacity() {
      self.buf.shrink_to(size as usize);
      unsafe { self.buf.set_len(size as usize) }
      return;
    }
    // TODO: might be better to use try_reserve_exact
    self.buf.reserve_exact(size as usize - self.buf.capacity())
  }

  /// Involves copying, use with caution
  #[napi]
  pub fn get_array(&self) -> Uint8Array {
    Uint8Array::new(self.buf.clone())
  }

  /// Returns the buffer, zero-copy :)
  #[napi]
  pub fn get_buffer(&self) -> Buffer {
    Buffer::from(&self.buf[self.r_pos..self.w_pos])
  }

  #[napi]
  pub fn get_readable_bytes(&self) -> u32 {
    (self.w_pos - self.r_pos) as u32
  }

  #[napi]
  pub fn skip_bytes(&mut self, length: u32) -> Result<(), Error> {
    if length > self.get_readable_bytes() {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "cannot skipBytes, given length {} is greater than readableBytes {}",
          length,
          self.get_readable_bytes()
        ),
      ));
    }
    self.r_pos += length as usize;
    Ok(())
  }

  // READ METHODS

  #[napi]
  pub fn read_boolean(&mut self) -> Result<bool, Error> {
    // TODO: Waiting for stable is_ok_and
    let res = self.read_byte();
    if let Ok(..) = res {
      return Ok(res.unwrap() != 0);
    }
    Err(Error::new(
      GenericFailure,
      "cannot readBoolean, readableBytes is less than 1".to_string(),
    ))
  }

  #[napi]
  pub fn read_byte(&mut self) -> Result<i32, Error> {
    if self.get_readable_bytes() < 1 {
      return Err(Error::new(
        GenericFailure,
        "cannot readByte, readableBytes is less than 1".to_string(),
      ));
    }
    self.r_pos += 1;
    Ok(self.buf[self.r_pos - 1] as i8 as i32)
  }

  #[napi]
  pub fn read_unsigned_byte(&mut self) -> Result<u32, Error> {
    if self.get_readable_bytes() < 1 {
      return Err(Error::new(
        GenericFailure,
        "cannot readUnsignedByte, readableBytes is less than 1".to_string(),
      ));
    }
    self.r_pos += 1;
    Ok(self.buf[self.r_pos - 1] as u32)
  }

  #[napi]
  pub fn read_short(&mut self) -> Result<i32, Error> {
    if self.get_readable_bytes() < 2 {
      return Err(Error::new(
        GenericFailure,
        "cannot readShort, readableBytes is less than 2".to_string(),
      ));
    }
    self.r_pos += 2;
    Ok(i16::from_be_bytes(
      self.buf[self.r_pos - 2..self.r_pos]
        .try_into()
        .unwrap(),
    ) as i32)
  }

  #[napi(js_name = "readShortLE")]
  pub fn read_short_le(&mut self) -> Result<i32, Error> {
    if self.get_readable_bytes() < 2 {
      return Err(Error::new(
        GenericFailure,
        "cannot readShortLE, readableBytes is less than 2".to_string(),
      ));
    }
    self.r_pos += 2;
    Ok(i16::from_le_bytes(
      self.buf[self.r_pos - 2..self.r_pos]
        .try_into()
        .unwrap(),
    ) as i32)
  }

  #[napi]
  pub fn read_unsigned_short(&mut self) -> Result<u32, Error> {
    if self.get_readable_bytes() < 2 {
      return Err(Error::new(
        GenericFailure,
        "cannot readUnsignedShort, readableBytes is less than 2".to_string(),
      ));
    }
    self.r_pos += 2;
    Ok(u16::from_be_bytes(
      self.buf[self.r_pos - 2..self.r_pos]
        .try_into()
        .unwrap(),
    ) as u32)
  }

  #[napi(js_name = "readUnsignedShortLE")]
  pub fn read_unsigned_short_le(&mut self) -> Result<u32, Error> {
    if self.get_readable_bytes() < 2 {
      return Err(Error::new(
        GenericFailure,
        "cannot readUnsignedShortLE, readableBytes is less than 2".to_string(),
      ));
    }
    self.r_pos += 2;
    Ok(u16::from_le_bytes(
      self.buf[self.r_pos - 2..self.r_pos]
        .try_into()
        .unwrap(),
    ) as u32)
  }

  #[napi]
  pub fn read_medium(&mut self) -> Result<i32, Error> {
    if self.get_readable_bytes() < 3 {
      return Err(Error::new(
        GenericFailure,
        "cannot readMedium, readableBytes is less than 3".to_string(),
      ));
    }
    self.r_pos += 3;
    let res = &self.buf[(self.r_pos - 3)..self.r_pos];
    Ok(((res[0] as i32) << 16) | ((res[1] as i32) << 8) | res[2] as i32)
    // Ok((res[0] & 0xFF | ((res[2] & 0xFF) << 8) | ((res[3] & 0x0F) << 16)) as i32)
  }

  /// Appends data to the end of the buffer
  /// tries to max out performance by using
  /// direct memory pointers in a unsafe context
  /// implies minimal copy.
  pub fn write_bytes(&mut self, buf: &[u8]) {
    let len = buf.len();
    self.buf.reserve(len);
    unsafe {
      std::ptr::copy(buf.as_ptr(), self.buf.as_mut_ptr(), len);
      self.buf.set_len(len);
    }

    // self.buf.extend_from_slice(buf);
    self.w_pos += buf.len();
  }

  #[napi]
  pub fn write_boolean(&mut self, val: bool) {
    self.write_byte(val as i32);
  }

  /// Writes both a signed / unsigned byte
  #[napi]
  pub fn write_byte(&mut self, val: i32) {
    self.buf.push(val as u8);
    self.w_pos += 1;
  }

  #[napi]
  pub fn write_short(&mut self, val: i32) {
    self.write_bytes(&(val as i16).to_be_bytes());
  }

  // TODO
  #[napi]
  pub fn write_medium(&mut self, val: i32) {
    // TODO
    // bytes.push((num >> 16) as u8);
    // bytes.push((num >> 8) as u8);
    // bytes.push(num as u8);
    self.buf.resize(3, 0);
    self.buf[self.w_pos] = (val >> 16) as u8;
    self.buf[self.w_pos + 1] = (val >> 8) as u8;
    self.buf[self.w_pos + 2] = val as u8;
    self.w_pos += 3;
  }

  #[napi]
  pub fn set_reader_index(&mut self, index: u32) -> Result<(), Error> {
    // This comparison is useless as we're using unsigned integers, but still, i'll keep it here
    // if index < 0 {
    //  return Err(Error::new(Status::InvalidArg, format!("cannot set reader index, given readerIndex {} is less than 0", index)))
    if (index as usize) > self.w_pos {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "cannot set readerIndex, given readerIndex {} is greater than writerIndex {}",
          index, self.w_pos
        ),
      ));
    }
    self.r_pos = index as usize;
    Ok(())
  }

  #[napi]
  pub fn get_reader_index(&self) -> u32 {
    self.r_pos as u32
  }

  /* #[napi]
  pub fn discard_read_bytes(&mut self) {
    self.buf.as_ptr_range().start = &self.buf[self.r_pos];
    unsafe {
      self.buf.set_len(self.w_pos - self.r_pos);
    }
    self.w_pos -= self.r_pos;
    self.r_pos = 0;
  } */

  #[napi]
  pub fn set_writer_index(&mut self, index: u32) -> Result<(), Error> {
    if (index as usize) < self.r_pos {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "cannot setWriterIndex, given writerIndex {} is less than readerIndex {}",
          index, self.r_pos
        ),
      ));
    } else if (index as usize) > self.buf.capacity() {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "cannot setWriterIndex, given writerIndex {} is greater than capacity {}",
          index,
          self.buf.capacity()
        ),
      ));
    }
    self.w_pos = index as usize;
    Ok(())
  }

  #[napi]
  pub fn get_writer_index(&self) -> u32 {
    self.w_pos as u32
  }

  #[napi]
  pub fn set_index(&mut self, r_index: u32, w_index: u32) -> Result<(), Error> {
    // if r_index < 0 {
    //  return Err(Error::new(Status::InvalidArg, format!("cannot setIndex, given readerIndex {} is less than 0", r_index)))
    if w_index < r_index {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "cannot setIndex, given writerIndex {} is less than given readerIndex {}",
          r_index, w_index
        ),
      ));
    } else if w_index as usize > self.buf.capacity() {
      return Err(Error::new(
        Status::InvalidArg,
        format!(
          "cannot setIndex, given writerIndex {} is greater than capacity {}",
          w_index,
          self.buf.capacity()
        ),
      ));
    }

    self.w_pos = w_index as usize;
    self.r_pos = r_index as usize;
    Ok(())
  }
}
