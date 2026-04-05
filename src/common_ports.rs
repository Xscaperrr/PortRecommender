pub const MIN_PORT: u16 = 1024;
pub const MAX_PORT: u16 = 65535;
pub const CANDIDATE_COUNT: u16 = MAX_PORT - MIN_PORT + 1;

const COMMON_PORTS: &[u16] = &[
    20, 21, 22, 23, 25, 53, 67, 68, 69, 80, 110, 111, 119, 123, 135, 137, 138, 139, 143, 161, 389,
    443, 445, 465, 514, 587, 631, 636, 873, 993, 995, 1080, 1433, 1521, 2049, 2181, 2375, 2376,
    2377, 2379, 2380, 3000, 3001, 3306, 3389, 4000, 5000, 5001, 5432, 5601, 5672, 5900, 5984, 6379,
    6443, 7000, 7001, 8000, 8008, 8080, 8081, 8088, 8443, 8888, 9000, 9090, 9200, 9300, 9418,
    10000, 11211, 15672, 27017,
];

pub fn common_ports() -> &'static [u16] {
    COMMON_PORTS
}

pub fn is_common_port(port: u16) -> bool {
    COMMON_PORTS.binary_search(&port).is_ok()
}
