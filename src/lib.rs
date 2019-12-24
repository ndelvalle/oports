use async_std::net::{IpAddr, TcpStream};

const MAX_PORT: u16 = 65535;

pub struct Oports {
    address: IpAddr,
}

impl Oports {
    pub fn new(ip: IpAddr) -> Self {
        Self { address: ip }
    }

    pub async fn is_port_open(&self, port: u16) -> bool {
        TcpStream::connect((self.address, port)).await.is_ok()
    }

    pub async fn open_ports_by_range(&self, from: u16, to: u16) -> Vec<u16> {
        let mut open_ports = vec![];

        for port in from..to {
            let is_open = self.is_port_open(port).await;
            if is_open {
                open_ports.push(port)
            }
        }
        open_ports
    }

    pub async fn open_ports(&self) -> Vec<u16> {
        self.open_ports_by_range(0, MAX_PORT).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use std::net::{IpAddr, Ipv4Addr, TcpListener};

    #[test]
    fn it_should_determine_if_a_port_is_open() {
        // TODO: It would be nice to shutdown the connection cleanly after the
        //       assertion was made.
        let _listener = TcpListener::bind("127.0.0.1:4040").unwrap();

        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let op = Oports::new(ip_v4_addr);

        let is_port_8585_open = block_on(op.is_port_open(4040));
        let is_port_2222_open = block_on(op.is_port_open(4041));

        assert_eq!(is_port_8585_open, true);
        assert_eq!(is_port_2222_open, false);
    }

    #[test]
    fn it_should_return_open_ports_by_range() {
        // TODO: It would be nice to shutdown the connection cleanly after the
        //       assertion was made.
        let _listener = TcpListener::bind("127.0.0.1:4045").unwrap();

        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let op = Oports::new(ip_v4_addr);

        let open_ports_a = block_on(op.open_ports_by_range(0, 10));
        let open_ports_b = block_on(op.open_ports_by_range(4000, 4100));

        assert_eq!(open_ports_a.len(), 0);
        assert_eq!(open_ports_b.len(), 1);
        assert_eq!(open_ports_b[0], 4045);
    }
}
