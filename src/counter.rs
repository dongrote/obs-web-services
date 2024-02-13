use std::{ffi::OsString, fs::OpenOptions, io::{self, Read, Write}, path::{Path, PathBuf}};

#[derive(Clone, Debug)]
pub struct Counter {
    file_path: OsString,
    count: u32,
}

impl Counter {
    pub fn from_file(file: &Path) -> io::Result<Counter> {
        match OpenOptions::new().read(true).open(PathBuf::from(&file)) {
            Ok(mut f) => {
                let mut buf = String::new();
                match f.read_to_string(&mut buf) {
                    Ok(_) => {
                        let count = match u32::from_str_radix(buf.trim(), 10) {
                            Ok(c) => c,
                            Err(_) => 0,
                        };
                        Ok(Counter {
                            count,
                            file_path: OsString::from(file),
                        })
                    },
                    Err(err) => Err(err),
                }
            },
            Err(_) => Ok(Counter {
                count: 0,
                file_path: OsString::from(file),
            }),
        }
    }

    pub fn value(&self) -> u32 {
        self.count
    }

    pub fn increment(&mut self) -> io::Result<()> {
        self.count += 1;
        self.commit()
    }

    pub fn reset(&mut self) -> io::Result<()> {
        self.count = 0;
        self.commit()
    }

    fn commit(&mut self) -> io::Result<()> {
        match OpenOptions::new().write(true).truncate(true).create(true).open(PathBuf::from(self.file_path.as_os_str()).as_path()) {
            Ok(mut f) => {
                match f.write(self.count.to_string().as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        eprintln!("file::write: {}", err);
                        Err(err)
                    }
                }
            },
            Err(err) => Err(err),
        }
    }
}
