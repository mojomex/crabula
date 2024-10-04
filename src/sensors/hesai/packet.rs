use static_assertions::assert_eq_size;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};
use zerocopy::little_endian as le;

#[derive(FromBytes, FromZeroes, AsBytes, Unaligned)]
#[repr(C)]
pub struct Header12B {
    start_of_packet: le::U16,
    protocol_major: u8,
    protocol_minor: u8,
    _reserved: [u8; 2],
    n_channels: u8,
    n_blocks: u8,
    _first_block_return: u8,
    distance_unit_mm: u8,
    n_returns_max: u8,
    flags: u8
}

assert_eq_size!(Header12B, [u8; 12]);

#[derive(FromBytes, FromZeroes, AsBytes, Unaligned)]
#[repr(C)]
pub struct Unit3B {
  distance: le::U16,
  reflectivity: u8
}

assert_eq_size!(Unit3B, [u8; 3]);

#[derive(FromBytes, FromZeroes, AsBytes, Unaligned)]
#[repr(packed)]
pub struct Block<TUnit, const N_CHANNELS: usize> {
  azimuth: le::U16,
  units: [TUnit; N_CHANNELS]
}

#[derive(FromBytes, FromZeroes, AsBytes, Unaligned)]
#[repr(packed)]
pub struct Body<TBlock, const N_BLOCKS: usize> {
  blocks: [TBlock; N_BLOCKS],
  crc: le::U32
}

#[derive(FromBytes, FromZeroes, AsBytes, Unaligned)]
#[repr(packed)]
pub struct DateTime <const YEAR_BIAS: u32> {
  year: u8,
  month: u8,
  day: u8,
  hour: u8,
  minute: u8,
  second: u8
}

assert_eq_size!(DateTime<0>, [u8; 6]);