use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};
use default_net;
use pnet::datalink::MacAddr;
use crate::interface;
use crate::scanner::{scan_hosts, scan_ports};
use crate::base_type::{PortScanType, HostScanResult, PortScanResult, ScanStatus};
use crate::define::DEFAULT_SRC_PORT;

/// Structure for host scan  
/// 
/// Should be constructed using HostScanner::new 
#[derive(Clone)]
pub struct HostScanner {
    /// Source IP Address  
    pub src_ip: IpAddr,
    /// List of target host  
    pub dst_ips: Vec<IpAddr>,
    /// Timeout setting of host scan  
    pub timeout: Duration,
    /// Timeout setting of host scan  
    pub wait_time: Duration,
    /// Result of host scan  
    pub scan_result: HostScanResult,
}

/// Structure for port scan  
/// 
/// Should be constructed using PortScanner::new 
#[derive(Clone)]
pub struct PortScanner {
    /// Index of network interface  
    pub if_index: u32,
    /// Name of network interface  
    pub if_name: String,
    /// Source MAC Address
    pub src_mac: MacAddr,
    /// Destination MAC Address
    pub dst_mac: MacAddr,
    /// Source IP Address  
    pub src_ip: IpAddr,
    /// Destination IP Address  
    pub dst_ip: IpAddr,
    /// Source port
    pub src_port: u16,
    /// Destination port  
    pub dst_ports: Vec<u16>,
    /// Type of port scan. Default is PortScanType::SynScan  
    pub scan_type: PortScanType,
    /// Timeout setting of port scan   
    pub timeout: Duration,
    /// Wait time after send task is finished
    pub wait_time: Duration,
    /// Packet send rate
    pub send_rate: Duration,
    /// Result of port scan  
    pub scan_result: PortScanResult,
}



impl HostScanner{
    /// Construct new HostScanner  
    pub fn new() -> Result<HostScanner, String> {
        let ini_scan_result = HostScanResult{
            up_hosts: vec![],
            scan_time: Duration::from_millis(1),
            scan_status: ScanStatus::Ready,
        };
        let host_scanner = HostScanner{
            src_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            dst_ips: vec![],
            timeout: Duration::from_millis(10000),
            wait_time: Duration::from_millis(200),
            scan_result: ini_scan_result,
        };
        Ok(host_scanner)
    }
    /// Add target host to list
    pub fn add_ipaddr(&mut self, ipaddr: &str) {
        let addr = ipaddr.parse::<IpAddr>();
        match addr {
            Ok(valid_addr) => {
                self.dst_ips.push(valid_addr);
            }
            Err(e) => {
                error!("Error adding ip address {}. Error: {}", ipaddr, e);
            }
        };
    }
    /// Set scan timeout  
    pub fn set_timeout(&mut self, timeout: Duration){
        self.timeout = timeout;
    }
    /// Set scan wait time  
    pub fn set_wait_time(&mut self, wait_time: Duration){
        self.wait_time = wait_time;
    }
    /// Set source IP Address 
    pub fn set_src_ipaddr(&mut self, src_ipaddr:IpAddr){
        self.src_ip = src_ipaddr;
    }
    /// Get source IP Address
    pub fn get_src_ipaddr(&mut self) -> IpAddr {
        return self.src_ip.clone();
    }
    /// Get target hosts
    pub fn get_target_hosts(&mut self) -> Vec<IpAddr> {
        return self.dst_ips.clone();
    }
    /// Get timeout 
    pub fn get_timeout(&mut self) -> Duration {
        return self.timeout.clone();
    }
    /// Get wait time
    pub fn get_wait_time(&mut self) -> Duration {
        return self.wait_time.clone();
    }
    /// Run scan with current settings 
    /// 
    /// Results are stored in HostScanner::scan_result
    pub fn run_scan(&mut self){
        let temp_scanner = self.clone();
        let start_time = Instant::now();
        let (uphosts, status) = scan_hosts(&temp_scanner);
        self.scan_result.up_hosts = uphosts;
        self.scan_result.scan_status = status;
        self.scan_result.scan_time = Instant::now().duration_since(start_time);
    }
    /// Return scan result
    pub fn get_result(&mut self) -> HostScanResult{
        return self.scan_result.clone();
    }
}

impl PortScanner{
    /// Construct new PortScanner (with network interface name)
    /// 
    /// Specify None for default. `PortScanner::new(None)`
    pub fn new(if_name: Option<&str>) -> Result<PortScanner, String> {
        let mut port_scanner = PortScanner{
            if_index: 0,
            if_name: String::new(),
            src_mac: MacAddr::zero(),
            dst_mac: MacAddr::zero(),
            src_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            dst_ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 
            src_port: DEFAULT_SRC_PORT,
            dst_ports: vec![],
            scan_type: PortScanType::SynScan,
            timeout: Duration::from_millis(30000),
            wait_time: Duration::from_millis(100),
            send_rate: Duration::from_millis(0),
            scan_result: PortScanResult::new(),
        };
        if let Some(if_name) = if_name {
            let if_index = interface::get_interface_index_by_name(if_name.to_string());
            if let Some(if_index) = if_index{
                port_scanner.if_index = if_index;
                port_scanner.if_name = if_name.to_string();
            }else{
                return Err("Failed to get interface info by name.".to_string());
            }
        }else{
            let def_if_index = default_net::get_default_interface_index();
            if let Some(def_if_index) = def_if_index {
                port_scanner.if_index = def_if_index;
            }else{
                return Err("Failed to get default interface info.".to_string());
            }
        }
        Ok(port_scanner)
    }
    /// Set IP address of target host
    pub fn set_target_ipaddr(&mut self, ip_addr: IpAddr){
        self.dst_ip = ip_addr;
    }
    /// Set range of target ports (by start and end)
    pub fn set_range(&mut self, start: u16, end: u16){
        for i in start..end + 1 {
            self.add_target_port(i);
        }
    }
    /// Add target port 
    pub fn add_target_port(&mut self, port_num: u16){
        self.dst_ports.push(port_num);
    }
    /// Set PortScanType. Default is PortScanType::SynScan
    pub fn set_scan_type(&mut self, scan_type: PortScanType){
        self.scan_type = scan_type;
    }
    /// Set scan timeout  
    pub fn set_timeout(&mut self, timeout: Duration){
        self.timeout = timeout;
    }
    /// Set scan wait-time  
    pub fn set_wait_time(&mut self, wait_time: Duration){
        self.wait_time = wait_time;
    }
    /// Set packet send rate
    pub fn set_send_rate(&mut self, send_rate: Duration){
        self.send_rate = send_rate;
    }
    /// Set source port number 
    pub fn set_src_port(&mut self, src_port: u16){
        self.src_port = src_port;
    }
    /// Get network interface index
    pub fn get_if_index(&mut self) -> u32 {
        return self.if_index.clone();
    }
    /// Get network interface name
    pub fn get_if_name(&mut self) -> String {
        return self.if_name.clone();
    }
    /// Get target ip address
    pub fn get_target_ipaddr(&mut self) -> IpAddr {
        return self.dst_ip.clone();
    }
    /// Get target ports
    pub fn get_target_ports(&mut self) -> Vec<u16> {
        return self.dst_ports.clone();
    }
    /// Get PortScanType
    pub fn get_scan_type(&mut self) -> PortScanType {
        return self.scan_type.clone();
    }
    /// Get source port number
    pub fn get_src_port_num(&mut self) -> u16 {
        return self.src_port.clone();
    }
    /// Get timeout
    pub fn get_timeout(&mut self) -> Duration {
        return self.timeout.clone();
    }
    /// Get wait-time
    pub fn get_wait_time(&mut self) -> Duration {
        return self.wait_time.clone();
    }
    /// Get send rate
    pub fn get_send_rate(&mut self) -> Duration {
        return self.send_rate.clone();
    }
    /// Run scan with current settings 
    /// 
    /// Results are stored in PortScanner::scan_result
    pub fn run_scan(&mut self) {
        let dst_mac = match self.scan_type {
            PortScanType::ConnectScan => {
                pnet::datalink::MacAddr::zero()
            },
            _ => {
                interface::get_default_gateway_macaddr()
            },
        };
        let interfaces = pnet::datalink::interfaces();
        let interface = interfaces.into_iter().filter(|interface: &pnet::datalink::NetworkInterface| interface.index == self.if_index).next().expect("Failed to get Interface");    
        let mut iface_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        for ip in &interface.ips {
            match ip.ip() {
                IpAddr::V4(ipv4) => iface_ip = IpAddr::V4(ipv4),
                IpAddr::V6(ipv6) => iface_ip = IpAddr::V6(ipv6),
            }
        }
        self.src_mac = interface.mac.unwrap();
        self.dst_mac = dst_mac;
        self.src_ip = iface_ip;
        let start_time = Instant::now();
        let (open_ports, status) = scan_ports(&interface, &self.clone());
        self.scan_result.ports = open_ports;
        self.scan_result.scan_status = status;
        self.scan_result.scan_time = Instant::now().duration_since(start_time);
    }
    /// Return scan result
    pub fn get_result(&mut self) -> PortScanResult{
        return self.scan_result.clone();
    }
}
