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
