use std::collections::HashSet;

use clap::ValueEnum;
use netstat2::{
    AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, SocketInfo, TcpState, get_sockets_info,
};

use crate::error::PortRecommenderError;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum ProtocolScope {
    Tcp,
    Udp,
    Both,
}

pub trait PortUsageProvider {
    fn occupied_ports(
        &self,
        protocol_scope: ProtocolScope,
    ) -> Result<HashSet<u16>, PortRecommenderError>;
}

pub struct NetstatPortUsageProvider;

impl PortUsageProvider for NetstatPortUsageProvider {
    fn occupied_ports(
        &self,
        protocol_scope: ProtocolScope,
    ) -> Result<HashSet<u16>, PortRecommenderError> {
        let sockets = get_sockets_info(
            AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
            protocol_flags(protocol_scope),
        )
        .map_err(|err| PortRecommenderError::PortInspection(err.to_string()))?;

        Ok(collect_ports(sockets, protocol_scope))
    }
}

fn protocol_flags(protocol_scope: ProtocolScope) -> ProtocolFlags {
    match protocol_scope {
        ProtocolScope::Tcp => ProtocolFlags::TCP,
        ProtocolScope::Udp => ProtocolFlags::UDP,
        ProtocolScope::Both => ProtocolFlags::TCP | ProtocolFlags::UDP,
    }
}

fn collect_ports(sockets: Vec<SocketInfo>, protocol_scope: ProtocolScope) -> HashSet<u16> {
    let mut ports = HashSet::new();

    for socket in sockets {
        match socket.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp)
                if matches!(protocol_scope, ProtocolScope::Tcp | ProtocolScope::Both)
                    && tcp.state == TcpState::Listen =>
            {
                ports.insert(tcp.local_port);
            }
            ProtocolSocketInfo::Udp(udp)
                if matches!(protocol_scope, ProtocolScope::Udp | ProtocolScope::Both) =>
            {
                ports.insert(udp.local_port);
            }
            _ => {}
        }
    }

    ports
}
