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
        match TcpStream::connect((self.address, port)).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn open_ports_by_range(&self, from: u16, to: u16) -> Vec<u16> {
        let mut open_ports = vec![];

        for port in from..to {
            println!(" port, {} ", port);
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
    use async_std::net::{IpAddr, Ipv4Addr};
    use futures::executor::block_on;

    #[test]
    fn it_should_return_open_ports_by_range() {
        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let op = Oports::new(ip_v4_addr);

        let future = op.open_ports_by_range(0, 10);
        let result = block_on(future);

        println!("Result {:?}", result);
    }

    #[test]
    fn it_should_return_open_ports() {
        let ip_v4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let op = Oports::new(ip_v4_addr);

        let future = op.open_ports();
        let result = block_on(future);

        println!("Result {:?}", result);
    }
}
