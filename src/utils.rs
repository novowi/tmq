use std::result;
use zmq;

/// Decode z85-encrypted key for CURVE authentication
pub fn z85_decode(encoded_key: &str) -> result::Result<Vec<u8>, zmq::DecodeError> {
    zmq::z85_decode(encoded_key)
}
