mod bytes;
pub mod error;
mod namespace;

use core::fmt;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

use crate::bytes::{read_bytes, read_i32_le, read_string, read_u32_le, read_u64_le};
use crate::error::{Error, Result};
use crate::namespace::Namespace;


const MAGIC: [u8; 16] = [
    0x0E, 0x14, 0x74, 0x75, 0x67, 0x4A, 0x03, 0xFC, 0x4A, 0x15, 0x90, 0x9D, 0xC3, 0x37, 0x7F, 0x1B,
];


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Version {
    Legacy = 0,
    Compact = 1,
    Optimized = 2,
    OptimizedCityHash = 3,
}

impl TryFrom<u8> for Version {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let val = match value {
            0 => Self::Legacy,
            1 => Self::Compact,
            2 => Self::Optimized,
            3 => Self::OptimizedCityHash,
            _ => return Err(Error::InvalidVersion(value)),
        };

        Ok(val)
    }
}


#[derive(Debug, Clone)]
pub struct LocalizedString {
    pub value: String,
    pub ref_count: Option<i32>,
}

impl fmt::Display for LocalizedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl core::ops::Deref for LocalizedString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for LocalizedString {
    fn from(value: String) -> Self {
        Self {
            value,
            ref_count: None,
        }
    }
}


#[derive(Debug)]
pub struct Localization {
    pub version: Version,
    pub strings: Vec<LocalizedString>,
    pub namespaces: HashMap<String, Namespace>,
    pub entries_count: Option<u32>,
}

impl Localization {
    pub fn from_reader<R>(mut reader: R) -> Result<Self>
    where
        R: Read + Seek,
    {
        let version = read_version(&mut reader)?;

        let strings = read_localization_strings(&mut reader, version)?;

        let entries_count = if version >= Version::Optimized {
            Some(read_u32_le(&mut reader)?)
        } else {
            None
        };

        let namespaces = namespace::read_all(&mut reader, &strings, version)?;

        Ok(Localization {
            version,
            strings,
            namespaces,
            entries_count,
        })
    }

    pub fn get_namespaced_string<N, K>(&self, namespace: N, key: K) -> Option<&LocalizedString>
    where
        N: AsRef<str>,
        K: AsRef<str>,
    {
        self.namespaces
            .get(namespace.as_ref())
            .and_then(|ns| ns.get(key))
    }
}


fn read_version<R>(mut reader: R) -> Result<Version>
where
    R: Read + Seek,
{
    let buf = read_bytes::<16, _>(&mut reader)?;

    let version = if buf == MAGIC {
        let buf = read_bytes::<1, _>(&mut reader)?;
        buf[0].try_into()?
    } else {
        reader.rewind()?;
        Version::Legacy
    };

    Ok(version)
}


fn read_localization_strings<R>(mut reader: R, version: Version) -> Result<Vec<LocalizedString>>
where
    R: Read + Seek,
{
    let mut localized = vec![];

    if version >= Version::Compact {
        let offset = read_u64_le(&mut reader)?;
        let stream_pos_bak = reader.stream_position()?;
        reader.seek(SeekFrom::Start(offset))?;

        let strings_count = read_i32_le(&mut reader)?;

        for _ in 0..strings_count {
            let value = read_string(&mut reader)?;

            let ref_count = if version >= Version::Optimized {
                Some(read_i32_le(&mut reader)?)
            } else {
                None
            };

            localized.push(LocalizedString { value, ref_count });
        }

        reader.seek(SeekFrom::Start(stream_pos_bak))?;
    }

    Ok(localized)
}
