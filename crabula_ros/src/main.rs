use std::time::Duration;

use safe_drive::{context::Context, error::DynError, msg::RosString};

use builtin_interfaces::msg::Time;
use sensor_msgs::msg::{
    PointCloud2, PointField, PointFieldSeq,
    PointField_Constants::{FLOAT32, UINT8},
};
use std_msgs::msg::header::Header;

use nebula_msgs::msg::NebulaPackets;

fn ros_time(time_ns: u64) -> Time {
    Time {
        sec: (time_ns / 1_000_000_000) as i32,
        nanosec: (time_ns % 1_000_000_000) as u32,
    }
}

fn ros_string(s: &str) -> RosString<0> {
    RosString::new(s).expect("strings shall be convertible")
}

fn gen_dummy_pointcloud(time_ns: u64) -> PointCloud2 {
    let fields = vec![
        PointField {
            name: ros_string("x"),
            offset: 0,
            datatype: FLOAT32,
            count: 4,
        },
        PointField {
            name: ros_string("y"),
            offset: 4,
            datatype: FLOAT32,
            count: 4,
        },
        PointField {
            name: ros_string("z"),
            offset: 8,
            datatype: FLOAT32,
            count: 4,
        },
    ];

    let field_seq = PointFieldSeq::new(fields.len()).expect("point field creation shall succeed");

    PointCloud2 {
        header: Header {
            stamp: ros_time(time_ns),
            frame_id: RosString::new("crabula").expect("strings shall be convertible"),
        },
        height: 1,
        width: 1000,
        fields: (),
        is_bigendian: false,
        point_step: (),
        row_step: (),
        data: (),
        is_dense: true,
    }
}

fn main() -> Result<(), DynError> {
    // Wraps an rcl::rcl_context_t -- a structure holding ROS environment info and logging configs
    let ctx = Context::new()?;
    let node = ctx.create_node("crabula", None, Default::default())?;
    let pointcloud_pub = node.create_publisher::<PointCloud2>("crabula_points", None)?;
    // let packet_pub = node.create_publisher::<NebulaPackets>("crabula_packets", None)?;

    let mut selector = ctx.create_selector()?;
    selector.add_wall_timer(
        "dummy_sensor",
        Duration::from_millis(100),
        Box::new(move || {}),
    );

    Ok(())
}
