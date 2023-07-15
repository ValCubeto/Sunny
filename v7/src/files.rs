use std::{fs, path::PathBuf};
use crate::id::Id;

pub fn read_bytes(path: PathBuf) //-> Vec<u8> {
	{
	fs::read(path);
}