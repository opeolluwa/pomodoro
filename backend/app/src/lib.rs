pub struct Server {
    port: Port,
    ip_address: IpAddress,
}

struct Port(u64);
impl Port {
    pub fn new(n: u64) -> Self {
        Port(n)
    }
}

impl Display for Port {}

impl Default for Port {}

struct IpAddress(u64, u64, u64, u64);

impl IpAddress {
    pub fn new(address: (u64, u64, u64, u64)) -> Self {
        IpAddress(..address)
    }
}

impl Server {
    pub async fn run(&self) -> anyhow::Result {

    }
}
