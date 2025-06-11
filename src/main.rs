//! # rdig by Ajlal Paracha, June 2025
//! This is a clone of the Dig CLI tool for DNS resolution in Rust
//!
//! rdig is a CLI tool to mimic the basic DNS lookup functionality of
//! the dig command. It allows you to query DNS records for a given
//! hostname using Cloudflare's 1.1.1.1 DNS server.
//!
//! ## Usage:
//!
//! ### For Developers:
//!
//! ```bash
//! cargo run -- <hostname>
//! ```
//!
//! ### For Users (with the release binary):
//!
//! ```bash
//! ./rdig <hostname>
//! ```

// Imports:
use chrono;
use clap::{Arg, Command};
use dns_lookup::{
    lookup_addr, lookup_host, lookup_mx, lookup_ns, lookup_soa, lookup_srv, lookup_txt,
};
use std::net::{IpAddr, ToSocketAddrs, UdpSocket};
use std::str::FromStr;
use std::time::Instant;

const RDIG_VERSION: &str = "1.0";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("rdig")
        .version(RDIG_VERSION)
        .about("A simple dig clone written in Rust, using Cloudflare's DNS (1.1.1.1)")
        .arg(
            Arg::new("hostname")
                .help("The hostname to query (e.g., google.com)")
                .required(true) // A mandatory argument
                .index(1),
        ) // As the first agument
        .get_matches();

    let hostname = matches
        .get_one::<String>("hostname")
        .ok_or("Hostname argument is missing; see usage")?;

    let cloudflare_dns_ip = IpAddr::from_str("1.1.1.1")?;
    let cloudflare_dns_port = 53;
    let dns_server_addr = (cloudflare_dns_ip, cloudflare_dns_port);

    // Simplified Dig-esc output
    println!("; <<>> rdig {} <<>> {}", RDIG_VERSION, hostname);
    println!(
        "; Querying DNS server: {}:{}",
        cloudflare_dns_ip, cloudflare_dns_port
    );
    println!("\n;; QUESTION SECTION:");
    println!(";{}.              IN      A", hostname); // We are querying A records
    println!("\n;; ANSWER SECTION:");

    let start_time = Instant::now();

    let ips: Vec<IpAddr> = match lookup_host(hostname, &dns_server_addr) {
        Ok(ip_addrs) => {
            if ip_addrs.is_empty() {
                println!("; No IP addresses found for {}", hostname);
            }
            ip_addrs
        }
        Err(e) => {
            eprintln!(";; Lookup failed for {}: {}", hostname, e);
            // Exit gracefully without panic
            return Ok(());
        }
    };

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    let query_time_ms = duration.as_millis();

    // Print results
    for ip in ips {
        println!("{}.        300     IN      A       {}", hostname, ip);
    }

    // Command time summary
    println!("\n;; Query time: {} msec", query_time_ms);
    println!(";; SERVER: {}#{}", cloudflare_dns_ip, CLOUDFLARE_DNS_PORT);
    println!(
        ";; WHEN: {}",
        chrono::Local::now().format("%a %b %d %H:%M:%S %Y")
    );

    Ok(());
}
