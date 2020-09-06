use std::net::SocketAddr;
use crate::XResult;

#[derive(Debug, Clone)]
pub enum IpAddress {
    Ipv4([u8; 4]),
}

impl IpAddress {
    pub fn parse_ipv4(addr: &str) -> Option<Self> {
        parse_ipv4_addr(addr).map(|parts| IpAddress::Ipv4(parts))
    }

    pub fn to_address(&self) -> String {
        match self {
            IpAddress::Ipv4(ipv4) => ipv4.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("."),
        }
    }

    pub fn is_matches(&self, socket_addr: &SocketAddr) -> bool {
        match self {
            IpAddress::Ipv4(self_ipv4_octets) => IpAddressMask::Ipv4(self_ipv4_octets.clone(), 32).is_matches(socket_addr),
        }
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
    ((ipv4[0] as u32) << (8 * 3)) + ((ipv4[1] as u32) << (8 * 2)) + ((ipv4[2] as u32) << 8) + (ipv4[3] as u32)
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
fn test_is_matches() {
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