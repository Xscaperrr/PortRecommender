use std::net::{TcpListener, UdpSocket};
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;

use port_recommender::hash_name_to_start_port;

fn run_cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_port-recommender"))
        .args(args)
        .output()
        .expect("failed to run CLI")
}

fn stdout_port(output: &Output) -> u16 {
    String::from_utf8(output.stdout.clone())
        .expect("stdout should be utf-8")
        .trim()
        .parse()
        .expect("stdout should contain a port number")
}

fn find_name_for_start_port(target_port: u16) -> String {
    for index in 0..2_000_000 {
        let candidate = format!("target-{target_port}-{index}");
        if hash_name_to_start_port(&candidate).unwrap() == target_port {
            return candidate;
        }
    }

    panic!("could not find a matching name for target port {target_port}");
}

#[test]
fn cli_prints_only_a_port_number() {
    let output = run_cli(&["example-service"]);

    assert!(output.status.success());
    assert!(String::from_utf8(output.stderr.clone()).unwrap().is_empty());
    assert!(stdout_port(&output) >= 1024);
}

#[test]
fn cli_accepts_all_protocol_modes() {
    for protocol in ["tcp", "udp", "both"] {
        let output = run_cli(&["example-service", "--protocol", protocol]);

        assert!(output.status.success(), "protocol={protocol}");
        assert!(stdout_port(&output) >= 1024);
    }
}

#[test]
fn tcp_listener_port_is_excluded_when_it_is_the_hash_start() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let busy_port = listener.local_addr().unwrap().port();
    let name = find_name_for_start_port(busy_port);

    thread::sleep(Duration::from_millis(50));

    let output = run_cli(&[&name, "--protocol", "tcp"]);

    assert!(output.status.success());
    assert_eq!(hash_name_to_start_port(&name).unwrap(), busy_port);
    assert_ne!(stdout_port(&output), busy_port);
}

#[test]
fn udp_listener_port_is_excluded_when_it_is_the_hash_start() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let busy_port = socket.local_addr().unwrap().port();
    let name = find_name_for_start_port(busy_port);

    thread::sleep(Duration::from_millis(50));

    let output = run_cli(&[&name, "--protocol", "udp"]);

    assert!(output.status.success());
    assert_eq!(hash_name_to_start_port(&name).unwrap(), busy_port);
    assert_ne!(stdout_port(&output), busy_port);
}

#[test]
fn udp_listener_port_is_excluded_in_both_mode() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let busy_port = socket.local_addr().unwrap().port();
    let name = find_name_for_start_port(busy_port);

    thread::sleep(Duration::from_millis(50));

    let output = run_cli(&[&name, "--protocol", "both"]);

    assert!(output.status.success());
    assert_eq!(hash_name_to_start_port(&name).unwrap(), busy_port);
    assert_ne!(stdout_port(&output), busy_port);
}
