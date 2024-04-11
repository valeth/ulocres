use std::collections::HashMap;
use std::io::Read;

use crate::bytes::{read_i32_le, read_string, read_u32_le};
use crate::error::{Error, Result};
use crate::{LocalizedString, Version};


#[derive(Debug)]
pub struct Namespace {
    pub key_hash: Option<i32>,
    pub members: HashMap<String, (LocalizedString, Option<u32>, u32)>,
}

impl Namespace {
    pub fn new() -> Self {
        Self {
            key_hash: None,
            members: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        key: String,
        key_hash: Option<u32>,
        source_hash: u32,
        localized: LocalizedString,
    ) {
        self.members.insert(key, (localized, key_hash, source_hash));
    }

    pub fn get<K>(&self, key: K) -> Option<&LocalizedString>
    where
        K: AsRef<str>,
    {
        self.members.get(key.as_ref()).map(|(k, _, _)| k)
    }

    pub fn for_each(&self) -> impl Iterator<Item = (&String, &LocalizedString)> {
        self.members.iter().map(|(k, (v, _, _))| (k, v))
    }
}


pub(crate) fn read_all<R>(
    mut reader: R,
    localized: &[LocalizedString],
    version: Version,
) -> Result<HashMap<String, Namespace>>
where
    R: Read,
{
    let namespace_count = read_u32_le(&mut reader)?;
    let mut namespaces = HashMap::new();

    for _ in 0..namespace_count {
        let (namespace_key, namespace) = read_one(&mut reader, localized, version)?;
        namespaces.insert(namespace_key, namespace);
    }

    Ok(namespaces)
}


fn read_one<R>(
    mut reader: R,
    localized: &[LocalizedString],
    version: Version,
) -> Result<(String, Namespace)>
where
    R: Read,
{
    let mut namespace = Namespace::new();

    if version >= Version::Optimized {
        namespace.key_hash = Some(read_i32_le(&mut reader)?);
    }

    let namespace_key = read_string(&mut reader)?;
    let entries = read_u32_le(&mut reader)?;

    for _ in 0..entries {
        let string_key_hash = if version >= Version::Optimized {
            Some(read_u32_le(&mut reader)?)
        } else {
            None
        };

        let string_key = read_string(&mut reader)?;
        let source_string_hash = read_u32_le(&mut reader)?;

        let localized_string = if version >= Version::Compact {
            let index = read_i32_le(&mut reader)? as usize;
            localized
                .get(index)
                .ok_or_else(|| Error::InvalidLocalizedStringIndex(index))?
                .clone()
        } else {
            read_string(&mut reader)?.into()
        };

        namespace.add(
            string_key,
            string_key_hash,
            source_string_hash,
            localized_string,
        );
    }

    Ok((namespace_key, namespace))
}
