use etherparse::{IpHeader, PacketBuilder, PacketHeaders, TransportHeader};
use serial_test::serial;
#[cfg(target_family = "unix")]
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use utuntap::{tap, tun};

#[cfg(target_os = "linux")]
#[test]
#[serial]
fn tun_sents_packets() {
    let (mut file, filename) = tun::OpenOptions::new()
        .packet_info(false)
        .number(10)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tun10");
    let data = [1; 10];
    let socket = UdpSocket::bind("10.10.10.1:2424").expect("failed to bind to address");
    socket
        .send_to(&data, "10.10.10.2:4242")
        .expect("failed to send data");
    let mut buffer = [0; 50];
    let number = file.read(&mut buffer).expect("failed to receive data");
    assert_eq!(number, 38);
    let packet = &buffer[..number];
    if let PacketHeaders {
        ip: Some(IpHeader::Version4(ip_header)),
        transport: Some(TransportHeader::Udp(udp_header)),
        payload,
        ..
    } = PacketHeaders::from_ip_slice(&packet).expect("failed to parse packet")
    {
        assert_eq!(ip_header.source, [10, 10, 10, 1]);
        assert_eq!(ip_header.destination, [10, 10, 10, 2]);
        assert_eq!(udp_header.source_port, 2424);
        assert_eq!(udp_header.destination_port, 4242);
        assert_eq!(payload, data);
    } else {
        assert!(false, "incorrect packet");
    }
}

#[cfg(target_os = "linux")]
#[test]
#[serial]
fn tun_receives_packets() {
    let (mut file, filename) = tun::OpenOptions::new()
        .packet_info(false)
        .number(10)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tun10");
    let data = [1; 10];
    let socket = UdpSocket::bind("10.10.10.1:2424").expect("failed to bind to address");
    let builder = PacketBuilder::ipv4([10, 10, 10, 2], [10, 10, 10, 1], 20).udp(4242, 2424);
    let packet = {
        let mut packet = Vec::<u8>::with_capacity(builder.size(data.len()));
        builder
            .write(&mut packet, &data)
            .expect("failed to build packet");
        packet
    };
    file.write(&packet).expect("failed to send packet");
    let mut buffer = [0; 50];
    let (number, source) = socket
        .recv_from(&mut buffer)
        .expect("failed to receive packet");
    assert_eq!(number, 10);
    assert_eq!(source.ip(), IpAddr::V4(Ipv4Addr::new(10, 10, 10, 2)));
    assert_eq!(source.port(), 4242);
    assert_eq!(data, &buffer[..number]);
}

#[cfg(target_os = "openbsd")]
use std::io::IoSlice;

#[cfg(target_os = "openbsd")]
#[test]
#[serial]
fn tun_sents_packets() {
    let (mut file, filename) = tun::OpenOptions::new()
        .number(10)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tun10");
    let data = [1; 10];
    let socket = UdpSocket::bind("10.10.10.1:2424").expect("failed to bind to address");
    socket
        .send_to(&data, "10.10.10.2:4242")
        .expect("failed to send data");
    let mut buffer = [0; 50];
    let number = file.read(&mut buffer).expect("failed to receive data");
    assert_eq!(number, 42);
    assert_eq!(&buffer[..4], [0u8, 0, 0, 2]);
    let packet = &buffer[4..number];
    if let PacketHeaders {
        ip: Some(IpHeader::Version4(ip_header)),
        transport: Some(TransportHeader::Udp(udp_header)),
        payload,
        ..
    } = PacketHeaders::from_ip_slice(&packet).expect("failed to parse packet")
    {
        assert_eq!(ip_header.source, [10, 10, 10, 1]);
        assert_eq!(ip_header.destination, [10, 10, 10, 2]);
        assert_eq!(udp_header.source_port, 2424);
        assert_eq!(udp_header.destination_port, 4242);
        assert_eq!(payload, data);
    } else {
        assert!(false, "incorrect packet");
    }
}

#[cfg(target_os = "openbsd")]
#[test]
#[serial]
fn tun_receives_packets() {
    let (mut file, filename) = tun::OpenOptions::new()
        .number(10)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tun10");
    let data = [1; 10];
    let socket = UdpSocket::bind("10.10.10.1:2424").expect("failed to bind to address");
    let builder = PacketBuilder::ipv4([10, 10, 10, 2], [10, 10, 10, 1], 20).udp(4242, 2424);
    let family = [0u8, 0, 0, 2];
    let packet = {
        let mut packet = Vec::<u8>::with_capacity(builder.size(data.len()));
        builder
            .write(&mut packet, &data)
            .expect("failed to build packet");
        packet
    };
    let iovec = [IoSlice::new(&family), IoSlice::new(&packet)];
    file.write_vectored(&iovec).expect("failed to send packet");
    let mut buffer = [0; 50];
    let (number, source) = socket
        .recv_from(&mut buffer)
        .expect("failed to receive packet");
    assert_eq!(number, 10);
    assert_eq!(source.ip(), IpAddr::V4(Ipv4Addr::new(10, 10, 10, 2)));
    assert_eq!(source.port(), 4242);
    assert_eq!(data, &buffer[..number]);
}

#[cfg(target_family = "unix")]
#[test]
#[serial]
fn tun_non_blocking_io() {
    let (mut file, filename) = tun::OpenOptions::new()
        .nonblock()
        .number(11)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tun11");
    let mut buffer = [0; 10];
    let error = file.read(&mut buffer).err().unwrap();
    assert_eq!(error.kind(), ErrorKind::WouldBlock);
}

#[cfg(target_family = "unix")]
#[test]
#[serial]
fn tap_non_blocking_io() {
    let (mut file, filename) = tap::OpenOptions::new()
        .nonblock()
        .number(11)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tap11");
    let mut buffer = [0; 10];
    let error = file.read(&mut buffer).err().unwrap();
    assert_eq!(error.kind(), ErrorKind::WouldBlock);
}
