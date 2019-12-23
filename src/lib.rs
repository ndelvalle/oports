use async_std::net::{IpAddr, TcpStream};

const MAX_PORT: u16 = 65535;

pub async fn get_open_ports(ip: IpAddr) -> Vec<u16> {
    let mut open = vec![];

    for port in 0..MAX_PORT {
        match TcpStream::connect((ip, port)).await {
            Ok(_) => open.push(port),
            Err(_) => {}
        }
    }

    open
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::net::{IpAddr, Ipv4Addr};
    use futures::executor::block_on;

    #[test]
    fn it_should_return_open_ports() {
        assert_eq!(2 + 2, 4);
        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let future = get_open_ports(ip);
        let result = block_on(future);

        println!("Result {:?}", result);
    }
}
