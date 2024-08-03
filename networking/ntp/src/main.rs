#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use ntp::{NTPMessage, NTPResult};
use std::net::UdpSocket;
use std::time::Duration;

use chrono::{DateTime, Utc};

const LOCAL_ADDR: &str = "0.0.0.0:12300";   // Default NTP port
const NTP_PORT: u16 = 123;


fn ntp_roundtrip( host: &str, port: u16) -> Result<NTPResult, std::io::Error> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);

    let request = NTPMessage::client();
    let mut response = NTPMessage::new();
    let message = request.data;

    // Connect to server
    let udp = UdpSocket::bind(LOCAL_ADDR)?;
    udp.connect(&destination).expect("unable to connect");

    // Send request
    let t1 = Utc::now();
    udp.send(&message)?;
    udp.set_read_timeout(Some(timeout))?;

    // Receive response
    udp.recv_from(&mut response.data)?;
    let t4 = Utc::now();

    let t2: DateTime<Utc> = response.rx_time().unwrap().into();
    let t3: DateTime<Utc> = response.tx_time().unwrap().into();

    Ok(NTPResult {
        t1: t1,
        t2: t2,
        t3: t3,
        t4: t4,
    })
}

fn main() {
    let servers = [
        "2.nz.pool.ntp.org",
        "time.nist.gov",
        "time.apple.com",
        "time.euro.apple.com",
        "time.google.com",
        "time2.google.com",
        //"time.windows.com",
    ];
    let server = &servers[0];
    eprintln!("Connecting to {}", server);
    let calc = ntp_roundtrip(&servers[0], NTP_PORT).unwrap();
    dbg!(&calc);
    println!("Delay is {}ms", calc.delay());
    println!("Offset is {}ms", calc.offset());
}
