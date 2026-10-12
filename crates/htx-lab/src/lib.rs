//! htx-lab crate placeholder

pub mod mixnet_sim;
pub mod control_stream_examples;
pub mod tunnel_mock;

/// Expose canonical bo-htx helpers via htx-lab as examples
pub use mixnet_sim::simulate_paths;
pub use control_stream_examples::{generate_example_control_stream, parse_and_validate};
pub use tunnel_mock::{encode_frame_example as encode_frame, decode_frame_example as decode_frame};

// Placeholder helper
pub fn hello_htx() -> &'static str {
    "htx-lab placeholder"
}
