extern crate netscan;
use netscan::blocking::PortScanner;
use netscan::setting::{ScanType, Destination};
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let mut port_scanner = match PortScanner::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 4))) {
        Ok(scanner) => (scanner),
        Err(e) => panic!("Error creating scanner: {}", e),
    };
    // Add scan target
    let dst_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 8));
    //let dst: Destination = Destination::new(dst_ip, vec![22, 80, 443, 5000, 8080]);
    let dst: Destination = Destination::new_with_port_range(dst_ip, 1, 1000);
    port_scanner.add_destination(dst);
    // Multiple target hosts can be configured
    //let dst_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10));
    //let dst: Destination = Destination::new_with_port_range(dst_ip, 1, 1000);
    //port_scanner.add_destination(dst);
    // Set options
    port_scanner.set_scan_type(ScanType::TcpSynScan);
    port_scanner.set_timeout(Duration::from_millis(10000));
    port_scanner.set_wait_time(Duration::from_millis(100));
    //port_scanner.set_send_rate(Duration::from_millis(1)); 
    // Run scan 
    let result = port_scanner.scan();
    // Print results 
    println!("Status: {:?}", result.scan_status);
    println!("Results:");
    for (ip, ports) in result.result_map {
        println!("{}", ip);
        for port in ports {
            println!("{:?}", port);
        }
    }
    println!("Scan Time: {:?}", result.scan_time);
}
