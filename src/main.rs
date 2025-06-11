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
use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};

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
        ) // As the first argument
        .get_matches();

    let hostname = matches
        .get_one::<String>("hostname")
        .ok_or("Hostname argument is missing; see usage")?;

    // Configure Cloudflare DNS
    let cloudflare_dns_ip = IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1));
    let cloudflare_dns_port = 53;

    let nameserver = NameServerConfig {
        socket_addr: (cloudflare_dns_ip, cloudflare_dns_port).into(),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: false,
        tls_config: None,
        bind_addr: None,
    };

    let mut config = ResolverConfig::new();
    config.add_name_server(nameserver);

    let resolver = Resolver::new(config, ResolverOpts::default())?;

    // Simplified Dig-like output
    println!("; <<>> rdig {} <<>> {}", RDIG_VERSION, hostname);
    println!(
        "; Querying DNS server: {}:{}",
        cloudflare_dns_ip, cloudflare_dns_port
    );
    println!("\n;; QUESTION SECTION:");
    println!(";{}.              IN      A", hostname); // We are querying A records
    println!("\n;; ANSWER SECTION:");

    let start_time = Instant::now();

    let response = match resolver.lookup_ip(hostname) {
        Ok(lookup) => lookup,
        Err(e) => {
            eprintln!(";; Lookup failed for {}: {}", hostname, e);
            // Exit gracefully without panic
            return Ok(());
        }
    };

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    let query_time_ms = duration.as_millis();

    let ips: Vec<IpAddr> = response.iter().collect();

    if ips.is_empty() {
        println!("; No IP addresses found for {}", hostname);
    } else {
        // Print results
        for ip in ips {
            println!("{}.        300     IN      A       {}", hostname, ip);
        }
    }

    // Command time summary
    println!("\n;; Query time: {} msec", query_time_ms);
    println!(";; SERVER: {}#{}", cloudflare_dns_ip, cloudflare_dns_port);
    println!(
        ";; WHEN: {}",
        chrono::Local::now().format("%a %b %d %H:%M:%S %Y")
    );

    Ok(())
}
