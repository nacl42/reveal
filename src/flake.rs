//! Typed, unique 64 bit numbers, loosely resembling Twitter Snowflake
//! IDs.
//!
//! Id consists of (time | machine id | sequence number)
//!
//! sequence numbers are atomic numbers up to 4096 (12 bits)
//! and are increased with each id generation.
//! node id is up to 1024 (10 bits)
//! timestamp is relative to DEFAULT_EPOCH

use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH, Duration};
use core::sync::atomic::{AtomicU64, Ordering::Relaxed, Ordering};

// DEFAULT_EPOCH is 2016-01-01T00:00:00.000
const DEFAULT_EPOCH: u64 = 1451602800;
const NODE_ID_LEFT_SHIFT: i8 = 10;
const SEQUENCE_BITS: i8 = 12;
const TIMESTAMP_LEFT_SHIFT: i8 = NODE_ID_LEFT_SHIFT + SEQUENCE_BITS;
const SEQUENCE_MASK: u64 = !0 ^ (!0 << SEQUENCE_BITS);
const ONE_MILLISEC: Duration = Duration::from_millis(1);
const NEXT_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Returns an unsigned 64-bit integer between 0 and 4095, i.e. a 12
/// bit value. The number is increased with each call, if it exceeds
/// 4095, then it is reset to 0 and the system waits for one
/// milisecond. Call this function before epoch_timestamp() to
/// guarantee that the combination between timestamp and sequence is
/// always unique.
#[inline]
fn next_seq() -> u64 {
    let seq = NEXT_SEQUENCE.fetch_add(1, Relaxed);
    if seq > 4095 {
        NEXT_SEQUENCE.store(0, Ordering::Relaxed);
        std::thread::sleep(ONE_MILLISEC);
        0
    } else {
        seq
    }
}

#[inline]
fn epoch_timestamp() -> Result<u64, SystemTimeError> {
    let t = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(((t.as_secs() - DEFAULT_EPOCH) * 1000) + t.subsec_nanos() as u64 / 1000000)
}

#[inline]
pub fn new_flake() -> Result<u64, SystemTimeError> {
    let node_id = 1;
    let sequence_id = next_seq();
    let timestamp = epoch_timestamp()?;
    
    Ok((timestamp << TIMESTAMP_LEFT_SHIFT)
       | (node_id << NODE_ID_LEFT_SHIFT)
       | (sequence_id & SEQUENCE_MASK))
}



