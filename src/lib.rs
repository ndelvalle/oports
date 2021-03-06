//! A crate to asynchronously retrieve open ports for a given IP address.
//!
//! All methods return a future that can be awaited, if you do not want to use
//! futures, you can simple run the `block_on` method from the futures executor
//! to block the caller until the given future has completed.

use async_std::net::{IpAddr, TcpStream};
use futures::future::FutureExt;
use futures::stream::StreamExt;

const CONCURRENCY: usize = 100;

/// Check if the given port is open for a specified IP address.
/// # Examples
///
/// ```
/// # use async_std::net::TcpListener;
/// use async_std::net::{IpAddr, Ipv4Addr};
///
/// # #[async_std::main]
/// # async fn main() {
/// # let listener = TcpListener::bind("127.0.0.1:4040").await.unwrap();
/// let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// let is_port_4040_open = oports::is_port_open(ip_v4_addr, 4040).await;
///
/// assert_eq!(is_port_4040_open, true);
/// # }
/// ```
pub async fn is_port_open(ip: IpAddr, port: u16) -> bool {
    TcpStream::connect((ip, port)).await.is_ok()
}

/// Retrieves a vec with open ports for a given vec of port numbers an IP
/// adresses. The default concurrency is `100` if the `Option` resolves to a
/// `None` value.
///
/// # Examples
///
/// ```
/// # use async_std::net::TcpListener;
/// use async_std::net::{IpAddr, Ipv4Addr};
///
/// # #[async_std::main]
/// # async fn main() {
/// # let listener = TcpListener::bind("127.0.0.1:4040").await.unwrap();
/// let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// let ports_to_check = (4000..4050).collect::<Vec<u16>>();
/// let concurrency = Some(10);
///
/// let open_ports = oports::get_open_ports(ip_v4_addr, ports_to_check, concurrency).await;
///
/// assert_eq!(vec![4040], open_ports);
/// # }
/// ```
pub async fn get_open_ports(ip: IpAddr, ports: Vec<u16>, concurrency: Option<usize>) -> Vec<u16> {
    let open_ports_futures = ports
        .into_iter()
        .map(|port| is_port_open(ip, port).map(move |is_open| (port, is_open)));

    futures::stream::iter(open_ports_futures)
        .buffer_unordered(concurrency.unwrap_or(CONCURRENCY))
        .filter_map(|item| async move {
            if item.1 {
                Some(item.0)
            } else {
                None
            }
        })
        .collect::<Vec<u16>>()
        .await
}

/// Retrieves a vec with all open ports for a given IP address. The default
/// concurrency is `100` if the `Option` resolves to a `None` value.
///
/// # Examples
///
/// ```no_run
/// # use async_std::net::TcpListener;
/// use async_std::net::{IpAddr, Ipv4Addr};
///
/// # #[async_std::main]
/// # async fn main() {
/// # let listener = TcpListener::bind("127.0.0.1:4040").await.unwrap();
/// let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// let concurrency = Some(100);
///
/// let open_ports = oports::get_all_open_ports(ip_v4_addr, concurrency).await;
///
/// assert_eq!(vec![4040], open_ports);
/// # }
/// ```
pub async fn get_all_open_ports(ip: IpAddr, concurrency: Option<usize>) -> Vec<u16> {
    let range = (0..u16::max_value()).collect::<Vec<u16>>();
    get_open_ports(ip, range, concurrency).await
}

#[cfg(test)]
mod tests {
    use async_std::net::{IpAddr, Ipv4Addr, TcpListener};
    use pretty_assertions::assert_eq;

    // TODO: It would be nice to shutdown the connection cleanly after the
    //       assertion was made.

    #[async_std::test]
    async fn is_port_open() {
        let _listener = TcpListener::bind("127.0.0.1:4040").await.unwrap();

        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let is_port_4040_open = super::is_port_open(ip_v4_addr, 4040).await;
        let is_port_4041_open = super::is_port_open(ip_v4_addr, 4041).await;

        assert_eq!(is_port_4040_open, true);
        assert_eq!(is_port_4041_open, false);
    }

    #[async_std::test]
    async fn get_open_ports() {
        let _listener = TcpListener::bind("127.0.0.1:4045").await.unwrap();
        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let open_ports_from_0_to_10 =
            super::get_open_ports(ip_v4_addr, (0..10).collect::<Vec<u16>>(), Some(100)).await;
        let open_ports_from_4000_to_4100 =
            super::get_open_ports(ip_v4_addr, (4000..4100).collect::<Vec<u16>>(), None).await;

        assert_eq!(open_ports_from_0_to_10.len(), 0);
        assert_eq!(open_ports_from_4000_to_4100.len(), 1);
        assert_eq!(open_ports_from_4000_to_4100[0], 4045);
    }

    // This test is too time expensive to be runned by the CI.
    #[async_std::test]
    #[ignore]
    async fn get_all_open_ports() {
        let _listener1 = TcpListener::bind("127.0.0.1:4050").await.unwrap();
        let _listener2 = TcpListener::bind("127.0.0.1:4060").await.unwrap();
        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        let open_ports = super::get_all_open_ports(ip_v4_addr, Some(100)).await;

        assert!(open_ports.len() > 0);
        assert!(open_ports.contains(&4050));
        assert!(open_ports.contains(&4060));
    }
}
