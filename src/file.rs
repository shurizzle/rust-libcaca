use std::io::Read;
use std::io::Write;
use std::path::Path;

use libcaca_sys::caca_file_eof;
use libcaca_sys::caca_file_open;
use libcaca_sys::caca_file_read;
use libcaca_sys::{caca_file_close, caca_file_t, caca_file_tell, caca_file_write};

use crate::utils::lossy_cstring;
use crate::{error::Error, result::Result};

pub struct File(*mut caca_file_t);

impl File {
    pub fn open<P: AsRef<Path>, S: AsRef<str>>(path: P, options: S) -> Result<File> {
        let options = lossy_cstring(options);
        let path = path.as_ref().to_string_lossy();
        let internal = unsafe { caca_file_open(path.as_ptr() as *const _, options.as_ptr()) };

        if internal.is_null() {
            Err(Error::IO(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unable to open file",
            )))
        } else {
            Ok(Self(internal))
        }
    }

    pub fn tell(&self) -> u64 {
        unsafe { caca_file_tell(self.as_internal()) }
    }

    pub fn write(&self, data: &[u8]) -> Result<usize> {
        Ok(self._write(data)?)
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize> {
        Ok(self._read(buffer)?)
    }

    // HINT: if you are searching for gets, stop it. It doesn't exist here.
    //       Use rust.

    pub fn is_eof(&self) -> bool {
        unsafe { caca_file_eof(self.as_internal()) != 0 }
    }

    fn _write(&self, data: &[u8]) -> std::result::Result<usize, std::io::Error> {
        let wrote = unsafe {
            caca_file_write(
                self.as_internal(),
                data.as_ptr() as *const _,
                data.len() as u64,
            )
        };

        if wrote != (data.len() as u64) {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Can't write the whole data",
            ))
        } else {
            Ok(wrote as usize)
        }
    }

    fn _read(&self, buffer: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        let read = unsafe {
            caca_file_read(
                self.as_internal(),
                buffer as *mut _ as *mut _,
                buffer.len() as u64,
            )
        };

        Ok(read as usize)
    }

    fn as_internal(&self) -> *mut caca_file_t {
        self.0
    }
}

impl Write for File {
    fn write(&mut self, data: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self._write(data)
    }

    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}

impl Read for File {
    fn read(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        self._read(buffer)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { caca_file_close(self.as_internal()) };
    }
}
