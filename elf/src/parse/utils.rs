/// Some general utility macros to assist with parsing.
///

// -------------------
// Declarative macros.

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

// ----------
// Functions.

/// Tries to extract null-terminated string from byte slice, starting
/// at the index `start`. Returns None if it runs out of bytes or goes
/// past `MAX_LEN` before finding a null char.
pub fn read_string(bytes: &[u8], start: usize) -> Option<String> {
    // We don't expect any strings > 4 MB. This keeps
    // use from iterating over most of the file if ab
    // error occurs or there is an unterminated string.
    const MAX_LEN: usize = 1024 * 4;

    if bytes[start] == b'\x00' {
        return Some(String::default());
    }

    for i in start..bytes.len() {
        if i > MAX_LEN {
            break;
        }

        if bytes[i] == b'\x00' {
            let string = String::from_utf8_lossy(&bytes[start..i]).to_string();
            return Some(string);
        }
    }

    None
}
