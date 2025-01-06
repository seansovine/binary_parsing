/// Some general utility macros to assist with parsing.
///

/// For converting an array of u8 bytes in little
/// endian order to a given integral type.
macro_rules! from_le_bytes {
    ( $t:ty, $slice:expr, $start:expr ) => {
        <$t>::from_le_bytes(
            $slice[$start..($start + size_of::<$t>())]
                .try_into()
                .unwrap(),
        )
    };
}

pub(crate) use from_le_bytes;

// ----

/// Tries to extract null-terminated string from byte slice.
/// This is maybe no longer needed.
pub fn read_string(bytes: &[u8]) -> Option<String> {
    if bytes[0] == b'\x00' {
        return Some(String::default());
    }

    for i in 0..bytes.len() {
        if bytes[i] == b'\x00' {
            let string = String::from_utf8_lossy(&bytes[..i]).to_string();
            return Some(string);
        }
    }

    None
}
