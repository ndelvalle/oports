use async_std::net::{IpAddr, TcpStream};
use futures::future::FutureExt;
use futures::stream::StreamExt;

pub async fn is_port_open(ip: IpAddr, port: u16) -> bool {
    TcpStream::connect((ip, port)).await.is_ok()
}

pub async fn open_ports_by_range(ip: IpAddr, from: u16, to: u16) -> Vec<u16> {
    let open_ports_futures =
        (from..to).map(|port| is_port_open(ip, port).map(move |is_open| (port, is_open)));

    let stream = futures::stream::iter(open_ports_futures)
        .buffer_unordered(100)
        .filter_map(|item| {
            async move {
                if item.1 {
                    Some(item.0)
                } else {
                    None
                }
            }
        });

    stream.collect::<Vec<u16>>().await
}

pub async fn open_ports(ip: IpAddr) -> Vec<u16> {
    open_ports_by_range(ip, 0, u16::max_value()).await
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use std::net::{IpAddr, Ipv4Addr, TcpListener};

    // TODO: It would be nice to shutdown the connection cleanly after the
    //       assertion was made, but I do not know how to do it.

    #[test]
    fn test_is_port_open() {
        let _listener = TcpListener::bind("127.0.0.1:4040").unwrap();

        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let is_port_4040_open = block_on(super::is_port_open(ip_v4_addr, 4040));
        let is_port_4041_open = block_on(super::is_port_open(ip_v4_addr, 4041));

        assert_eq!(is_port_4040_open, true);
        assert_eq!(is_port_4041_open, false);
    }

    #[test]
    fn test_open_ports_by_range() {
        let _listener = TcpListener::bind("127.0.0.1:4045").unwrap();

        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let open_ports_a = block_on(super::open_ports_by_range(ip_v4_addr, 0, 10));
        let open_ports_b = block_on(super::open_ports_by_range(ip_v4_addr, 4000, 4100));

        assert_eq!(open_ports_a.len(), 0);
        assert_eq!(open_ports_b.len(), 1);
        assert_eq!(open_ports_b[0], 4045);
    }

    // This test is too expensive
    #[test]
    #[ignore]
    fn test_open_ports() {
        let _listener1 = TcpListener::bind("127.0.0.1:4050").unwrap();
        let _listener2 = TcpListener::bind("127.0.0.1:4060").unwrap();

        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let open_ports = block_on(super::open_ports(ip_v4_addr));

        assert!(true);

        assert!(open_ports.len() > 0);
        assert!(open_ports.contains(&4050));
        assert!(open_ports.contains(&4060));
    }
}
