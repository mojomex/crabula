use static_assertions::assert_eq_size;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};
use zerocopy::little_endian as le;

use crate::sensors::hesai::packet::{Block, Body, DateTime, Header12B, Unit3B};

#[derive(FromBytes, FromZeroes, AsBytes, Unaligned)]
#[repr(C)]
struct TailOT128 {
    _reserved: [u8; 9],
    azimuth_state: le::U16,
    operational_state: u8,
    return_mode: u8,
    motor_speed_rpm: le::U16,
    date_time: DateTime<1900>,
    timestamp_us: le::U32,
    factory_information: u8,
    udp_sequence: le::U32,
    _imu: [u8; 22],
    crc: le::U32
}

#[derive(FromBytes, FromZeroes, AsBytes, Unaligned)]
#[repr(C)]
struct PacketOT128 {
    header: Header12B,
    body: Body<Block<Unit3B, 128>, 2>,
    _functional_safety: [u8; 17],
    tail: TailOT128
}

assert_eq_size!(PacketOT128, [u8; 861]);