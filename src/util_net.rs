use std::fmt::{self, Display, Formatter};
use std::result::Result;
use std::net::SocketAddr;
use crate::XResult;

const DEFAULT_LISTEN_ADDR: [u8; 4] = [127, 0, 0, 1];

#[derive(Debug, Clone)]
pub enum IpAddress {
    Ipv4([u8; 4]),
}

impl IpAddress {
    pub fn parse_ipv4(addr: &str) -> Option<Self> {
        parse_ipv4_addr(addr).map(IpAddress::Ipv4)
    }

    pub fn to_address(&self) -> String {
        match self {
            IpAddress::Ipv4(ipv4) => ipv4.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("."),
        }
    }

    pub fn is_matches(&self, socket_addr: &SocketAddr) -> bool {
        match self {
            IpAddress::Ipv4(self_ipv4_octets) => IpAddressMask::Ipv4(*self_ipv4_octets, 32).is_matches(socket_addr),
        }
    }
}

impl Display for IpAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_address())
    }
}

#[derive(Debug, Clone)]
pub enum IpAddressMask {
    Ipv4([u8; 4], u8),
}

impl IpAddressMask {
    pub fn parse_ipv4(addr: &str) -> Option<Self> {
        let addr_mask_parts = addr.split('/').collect::<Vec<_>>();
        let (addr_ip, mask) = if addr_mask_parts.len() == 1 {
            (addr_mask_parts[0], 32)
        } else if addr_mask_parts.len() == 2 {
            if let Ok(mask) = addr_mask_parts[1].parse::<u8>() {
                (addr_mask_parts[0], mask)
            } else {
                return None;
            }
        } else {
            return None;
        };
        parse_ipv4_addr(addr_ip).map(|parts| IpAddressMask::Ipv4(parts, mask))
    }

    pub fn to_address(&self) -> String {
        match self {
            IpAddressMask::Ipv4(ipv4, mask) => {
                format!("{}/{}", ipv4.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("."), mask)
            },
        }
    }

    pub fn is_matches(&self, socket_addr: &SocketAddr) -> bool {
        match socket_addr {
            SocketAddr::V4(socket_addr_v4) => {
                let socket_addr_v4_octets = socket_addr_v4.ip().octets();
                match self {
                    IpAddressMask::Ipv4(self_ipv4_octets, mask) => {
                        let self_ipv4_u32 = ipv4_to_u32(&self_ipv4_octets);
                        let addr_ipv4_u32 = ipv4_to_u32(&socket_addr_v4_octets);
                        let mask_u32 = ipv4_mask(*mask);
                        self_ipv4_u32 & mask_u32 == addr_ipv4_u32 & mask_u32
                    },
                }
            },
            SocketAddr::V6(_) => false,
        }
    }
}

impl Display for IpAddressMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_address())
    }
}

#[derive(Debug, Clone)]
pub struct IpAddressMaskGroup {
    pub ip_address_mask_group: Vec<IpAddressMask>,
}

impl IpAddressMaskGroup {
    pub fn parse(ip_mask_group: &[String]) -> Self {
        let mut ret = vec![];
        for ip_mask_addr in ip_mask_group {
            if let Some(ip_mask) = IpAddressMask::parse_ipv4(ip_mask_addr) {
                ret.push(ip_mask);
            }
        }
        Self { ip_address_mask_group: ret }
    }

    pub fn is_empty(&self) -> bool {
        self.ip_address_mask_group.is_empty()
    }

    pub fn is_matches(&self, socket_addr: &SocketAddr) -> bool {
        self.ip_address_mask_group.iter().any(|ip_address_mask| ip_address_mask.is_matches(socket_addr))
    }

    pub fn is_empty_or_matches(&self, socket_addr: &SocketAddr) -> bool {
        self.is_empty() || self.is_matches(socket_addr)
    }
}

impl Display for IpAddressMaskGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[{}]", self.ip_address_mask_group.iter().map(|i| format!("{}", i)).collect::<Vec<_>>().join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct IpAddressAndPort {
    pub ip: IpAddress,
    pub port: u16,
}

impl IpAddressAndPort {
    pub fn parse(ip_address_and_port: &str) -> Option<Self> {
        if let Some((ipv4, port)) = parse_ip_and_port(ip_address_and_port) {
            return Some(IpAddressAndPort {
                ip: IpAddress::Ipv4(ipv4),
                port,
            });
        }
        None
    }

    pub fn to_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn to_ipv4_and_port(&self) -> ([u8; 4], u16) {
        match self.ip {
            IpAddress::Ipv4(ipv4) => (ipv4, self.port),
        }
    }
}

impl Display for IpAddressAndPort {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

// :8080 -> 127.0.0.1:8080
fn parse_ip_and_port(listen: &str) -> Option<([u8; 4], u16)> {
    let listen_addr = match listen.split(':').next() {
        None => DEFAULT_LISTEN_ADDR,
        Some(addr) if addr.is_empty() => DEFAULT_LISTEN_ADDR,
        Some(addr) => match parse_ipv4_addr(addr) {
            Some(parsed_ip_address) => parsed_ip_address, None => return None,
        },
    };

    let listen_port = match listen.split(':').nth(1) {
        None => return None,
        Some(port) => match port.parse::<u16>() {
            Ok(port) => port, Err(_) => return None,
        },
    };

    Some((listen_addr, listen_port))
}

fn ipv4_mask(mask: u8) -> u32 {
    let mut r = 0_u32;
    for _ in 0..mask {
        r <<= 1;
        r |= 1;
    }
    for _ in mask..32 {
        r <<= 1;
    }
    r
}

fn ipv4_to_u32(ipv4: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*ipv4)
    // ((ipv4[0] as u32) << (8 * 3)) + ((ipv4[1] as u32) << (8 * 2)) + ((ipv4[2] as u32) << 8) + (ipv4[3] as u32)
}

fn parse_ipv4_addr(addr: &str) -> Option<[u8; 4]>  {
    let addr_parts = addr.split('.').collect::<Vec<_>>();
    if addr_parts.len() != 4 {
        return None;
    }
    let parsed_addr = || -> XResult<[u8; 4]> {
        Ok([addr_parts[0].parse::<u8>()?,
            addr_parts[1].parse::<u8>()?,
            addr_parts[2].parse::<u8>()?,
            addr_parts[3].parse::<u8>()?
        ])
    };
    parsed_addr().ok()
}

#[test]
fn test_ip_address_is_matches() {
    let addr = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), 123);
    let addr2 = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 2)), 123);
    assert_eq!(true, IpAddressMask::parse_ipv4("127.0.0.1").unwrap().is_matches(&addr));
    assert_eq!(true, IpAddressMask::parse_ipv4("127.0.0.1/32").unwrap().is_matches(&addr));
    assert_eq!(true, IpAddressMask::parse_ipv4("127.0.0.1/31").unwrap().is_matches(&addr));
    assert_eq!(true, IpAddressMask::parse_ipv4("127.0.0.1/30").unwrap().is_matches(&addr));
    assert_eq!(false, IpAddressMask::parse_ipv4("127.0.0.1").unwrap().is_matches(&addr2));
    assert_eq!(false, IpAddressMask::parse_ipv4("127.0.0.1/32").unwrap().is_matches(&addr2));
    assert_eq!(false, IpAddressMask::parse_ipv4("127.0.0.1/31").unwrap().is_matches(&addr2));
    assert_eq!(true, IpAddressMask::parse_ipv4("127.0.0.1/30").unwrap().is_matches(&addr2));
}

#[test]
fn test_ip_address_port() {
    let ip_address_and_port = IpAddressAndPort::parse(":80");
    assert_eq!("127.0.0.1:80", format!("{}", ip_address_and_port.unwrap()));
    let ip_address_and_port = IpAddressAndPort::parse("0.0.0.0:80");
    assert_eq!("0.0.0.0:80", format!("{}", ip_address_and_port.unwrap()));
    let ip_address_and_port = IpAddressAndPort::parse("1.1.1.1:80");
    assert_eq!("1.1.1.1:80", format!("{}", ip_address_and_port.unwrap()));
}

#[test]
fn test_ip_address_mask_group_is_matches() {
    let group = IpAddressMaskGroup::parse(&vec!["127.0.0.1".to_owned(), "10.0.0.0/24".to_owned()]);
    let addr = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), 123);
    assert_eq!(true, group.is_matches(&addr));
    let addr = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 2)), 123);
    assert_eq!(false, group.is_matches(&addr));
    let addr = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 0, 2)), 123);
    assert_eq!(true, group.is_matches(&addr));
    let addr = SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 1, 2)), 123);
    assert_eq!(false, group.is_matches(&addr));
}
