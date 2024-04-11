use std::io::Read;

use crate::error::Result;


pub fn read_bytes<const BYTES: usize, R>(mut reader: R) -> Result<[u8; BYTES]>
where
    R: Read,
{
    let mut buf = [0u8; BYTES];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}


pub fn read_i32_le<R>(reader: R) -> Result<i32>
where
    R: Read,
{
    let buf = read_bytes::<4, _>(reader)?;
    Ok(i32::from_le_bytes(buf))
}


pub fn read_u32_le<R>(reader: R) -> Result<u32>
where
    R: Read,
{
    let buf = read_bytes::<4, _>(reader)?;
    Ok(u32::from_le_bytes(buf))
}


pub fn read_i64_le<R>(reader: R) -> Result<i64>
where
    R: Read,
{
    let buf = read_bytes::<8, _>(reader)?;
    Ok(i64::from_le_bytes(buf))
}


pub fn read_u64_le<R>(reader: R) -> Result<u64>
where
    R: Read,
{
    let buf = read_bytes::<8, _>(reader)?;
    Ok(u64::from_le_bytes(buf))
}


pub fn read_string<R>(mut reader: R) -> Result<String>
where
    R: Read,
{
    let len = read_i32_le(&mut reader)?;

    let string = if len > 0 {
        let len = len as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf)?
    } else if len < 0 {
        let len = (len * -2) as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8_lossy(&buf).to_string()
    } else {
        String::new()
    };

    let string = string.trim_end_matches('\0').to_owned();

    Ok(string)
}
