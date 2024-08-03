#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

/**
Bare-bones NTP client, just for fun.

*/

use std::mem::zeroed;
use std::net::UdpSocket;
use std::time::Duration;

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{TimeZone};
use chrono::{DateTime, Local, Utc};


const NTP_MESSAGE_LENGTH: usize = 48;       // 12 32-bit integers
const NTP_TO_EPOCH: i64 = 2_208_988_800;    // NTP epoch starts 1 Jan 1900
const LOCAL_ADDR: &str = "0.0.0.0:12300";   // Default NTP port


/// Network message
pub struct NTPMessage {
    pub data: [u8; NTP_MESSAGE_LENGTH],
}


impl NTPMessage {

    /// Create message with zeroed message packet
    pub fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LENGTH],
        }
    }

    /// Fill in first byte of message with version and mode
    pub fn client() -> Self {
        const VERSION: u8 = 0b00_011_000;
        const MODE: u8 = 0b00_000_011;

        let mut msg = NTPMessage::new();
        msg.data[0] |= VERSION;
        msg.data[0] |= MODE;
        msg
    }

    /// Extract NTP timestamp from message packet from given start
    fn parse_timestamp(&self, i: usize) -> Result<NTPTimestamp, std::io::Error> {
        // Read two 32-bit integers from given starting position
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimestamp {
            seconds:  seconds,
            fraction: fraction,
        })
    }

    pub fn rx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(32)
    }

    pub fn tx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
        self.parse_timestamp(40)
    }
}


/**
Result obtained after full request/reponse cycle.

In request/response mode NTP generates four timestamps:

- *T1* Client timestamp when request was sent.
- *T2* Server timestamp when request was received.
- *T3* Server timestamp when response sent.
- *T4* Client timestamp when response received.

*/
#[derive(Debug)]
pub struct NTPResult {
    pub t1: DateTime<Utc>,
    pub t2: DateTime<Utc>,
    pub t3: DateTime<Utc>,
    pub t4: DateTime<Utc>,
}


impl NTPResult {

    /// Create fake result, with all timestamps just set to now
    pub fn fake() -> Self {
        let now = Utc::now();
        NTPResult {
            t1: now,
            t2: now,
            t3: now,
            t4: now,
        }
    }
}


impl NTPResult {
    /// Offset from server clock in milliseconds.
    /// Calculation taken from book code, but I'm suprised to see
    /// a call to abs() - how does this deal with a negative offset?
    pub fn offset(&self) -> i64 {
        let delta = self.delay();
        delta.abs() / 2
    }

    /// Round-trip delay, in milliseconds.
    pub fn delay(&self) -> i64 {
        // Total delay, minus server-side minus processing time.
        let duration = (self.t4 - self.t1) - (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}


/**
Represents the 136 years from 1900 to 2036, with a precision of 232 picoseconds.

An NTP timestamp is a truncated NTP date expressed as an unsigned 64-bit
integer including the low order 32 bits of the seconds field concatenated with
the high-order 32 bits of the fraction field.
*/
#[derive(Clone, Copy, Debug, Default)]
pub struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}


/// Convert from NTPTimestamp to UTC Datetime
impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_EPOCH;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2_f64.powi(32);
        DateTime::from_timestamp(secs, nanos as u32).expect("Invalid epoch")
    }
}


/// Convert from UTC Datetime to NTPTimestamp
impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc: DateTime<Utc>) -> Self {
        let secs = utc.timestamp() + NTP_TO_EPOCH;
        let mut fraction = utc.timestamp_subsec_nanos() as f64;
        fraction *= 2_f64.powi(32);
        fraction /= 1e9;

        NTPTimestamp {
            seconds: secs as u32,
            fraction: fraction as u32,
        }
    }
}
