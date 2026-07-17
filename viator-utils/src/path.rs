use std::fs;
use std::path::PathBuf;
use anyhow::{anyhow, Error};

pub trait PathBufLifeHack {
    fn ensure_dir(self) -> Result<Self, Error> where Self: Sized;
    fn ensure_file(self) -> Result<Self, Error> where Self: Sized;

    fn mkdir(self) -> Result<Self, Error> where Self: Sized;
    fn touch(self) -> Result<Self, Error> where Self: Sized;
}

impl PathBufLifeHack for PathBuf {
    fn ensure_dir(self) -> Result<Self, Error> where Self: Sized {
        if self.exists() {
            if !self.is_dir() {
                anyhow::bail!("Expected {:?} to be a directory!", self)
            }
        }

        Ok(self)
    }

    fn ensure_file(self) -> Result<Self, Error> where Self: Sized {
        if self.exists() {
            if !self.is_file() {
                anyhow::bail!("Expected {:?} to be a file!", self)
            }
        }

        Ok(self)
    }

    fn mkdir(self) -> Result<Self, Error> where Self: Sized {
        fs::create_dir_all(&self)?;

        Ok(self)
    }

    fn touch(self) -> Result<Self, Error> where Self: Sized {
        drop(fs::OpenOptions::new().create_new(true).open(&self)?);

        Ok(self)
    }
}