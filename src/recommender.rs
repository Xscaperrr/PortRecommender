use std::collections::HashSet;

use sha2::{Digest, Sha256};

use crate::common_ports::{CANDIDATE_COUNT, MAX_PORT, MIN_PORT, common_ports};
use crate::error::PortRecommenderError;
use crate::ports::{PortUsageProvider, ProtocolScope};

pub fn recommend_port<P: PortUsageProvider>(
    name: &str,
    protocol_scope: ProtocolScope,
    provider: &P,
) -> Result<u16, PortRecommenderError> {
    validate_name(name)?;

    let occupied_ports = provider.occupied_ports(protocol_scope)?;
    let excluded_ports = build_excluded_ports(&occupied_ports);
    let start_port = hash_name_to_start_port(name)?;

    probe_for_available_port(start_port, &excluded_ports)
}

pub fn hash_name_to_start_port(name: &str) -> Result<u16, PortRecommenderError> {
    validate_name(name)?;

    let digest = Sha256::digest(name.as_bytes());
    let hash_prefix = u64::from_be_bytes(digest[..8].try_into().expect("slice length is fixed"));
    let offset = (hash_prefix % u64::from(CANDIDATE_COUNT)) as u16;

    Ok(MIN_PORT + offset)
}

fn validate_name(name: &str) -> Result<(), PortRecommenderError> {
    if name.is_empty() {
        return Err(PortRecommenderError::EmptyName);
    }

    Ok(())
}

fn build_excluded_ports(occupied_ports: &HashSet<u16>) -> HashSet<u16> {
    let mut excluded_ports = HashSet::with_capacity(common_ports().len() + occupied_ports.len());

    for &port in common_ports() {
        if (MIN_PORT..=MAX_PORT).contains(&port) {
            excluded_ports.insert(port);
        }
    }

    for &port in occupied_ports {
        if (MIN_PORT..=MAX_PORT).contains(&port) {
            excluded_ports.insert(port);
        }
    }

    excluded_ports
}

pub(crate) fn probe_for_available_port(
    start_port: u16,
    excluded_ports: &HashSet<u16>,
) -> Result<u16, PortRecommenderError> {
    let normalized_start = normalize_start_port(start_port);
    let start_offset = u32::from(normalized_start - MIN_PORT);

    for step in 0..u32::from(CANDIDATE_COUNT) {
        let candidate_offset = (start_offset + step) % u32::from(CANDIDATE_COUNT);
        let candidate = MIN_PORT + candidate_offset as u16;

        if !excluded_ports.contains(&candidate) {
            return Ok(candidate);
        }
    }

    Err(PortRecommenderError::NoAvailablePort)
}

fn normalize_start_port(start_port: u16) -> u16 {
    if (MIN_PORT..=MAX_PORT).contains(&start_port) {
        start_port
    } else {
        MIN_PORT
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{hash_name_to_start_port, probe_for_available_port, recommend_port};
    use crate::common_ports::{CANDIDATE_COUNT, MAX_PORT, MIN_PORT};
    use crate::error::PortRecommenderError;
    use crate::ports::{PortUsageProvider, ProtocolScope};

    struct MockPortUsageProvider {
        occupied: HashSet<u16>,
    }

    impl PortUsageProvider for MockPortUsageProvider {
        fn occupied_ports(
            &self,
            _protocol_scope: ProtocolScope,
        ) -> Result<HashSet<u16>, PortRecommenderError> {
            Ok(self.occupied.clone())
        }
    }

    #[test]
    fn hash_mapping_is_stable_for_same_name() {
        let first = hash_name_to_start_port("example-service").unwrap();
        let second = hash_name_to_start_port("example-service").unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn different_names_usually_map_to_different_start_ports() {
        let alpha = hash_name_to_start_port("alpha").unwrap();
        let beta = hash_name_to_start_port("beta").unwrap();

        assert_ne!(alpha, beta);
    }

    #[test]
    fn empty_name_is_rejected() {
        let provider = MockPortUsageProvider {
            occupied: HashSet::new(),
        };

        assert!(matches!(
            recommend_port("", ProtocolScope::Both, &provider),
            Err(PortRecommenderError::EmptyName)
        ));
    }

    #[test]
    fn common_port_is_skipped_during_probe() {
        let excluded = HashSet::from([3000]);

        assert_eq!(probe_for_available_port(3000, &excluded).unwrap(), 3001);
    }

    #[test]
    fn occupied_port_is_skipped_during_probe() {
        let excluded = HashSet::from([55000]);

        assert_eq!(probe_for_available_port(55000, &excluded).unwrap(), 55001);
    }

    #[test]
    fn probe_wraps_at_end_of_range() {
        let excluded = HashSet::from([MAX_PORT]);

        assert_eq!(
            probe_for_available_port(MAX_PORT, &excluded).unwrap(),
            MIN_PORT
        );
    }

    #[test]
    fn no_port_is_returned_when_everything_is_excluded() {
        let provider = MockPortUsageProvider {
            occupied: (MIN_PORT..=MAX_PORT).collect(),
        };

        assert!(matches!(
            recommend_port("blocked", ProtocolScope::Both, &provider),
            Err(PortRecommenderError::NoAvailablePort)
        ));
    }

    #[test]
    fn unicode_names_are_supported() {
        let provider = MockPortUsageProvider {
            occupied: HashSet::new(),
        };

        let recommended = recommend_port("服务-数据库", ProtocolScope::Both, &provider).unwrap();

        assert!((MIN_PORT..=MAX_PORT).contains(&recommended));
    }

    #[test]
    fn mapping_covers_full_candidate_range() {
        let port = hash_name_to_start_port("range-check").unwrap();

        assert!((MIN_PORT..=MAX_PORT).contains(&port));
        assert_eq!(
            u32::from(CANDIDATE_COUNT),
            u32::from(MAX_PORT - MIN_PORT + 1)
        );
    }
}
