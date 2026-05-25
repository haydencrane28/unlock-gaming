use crate::config::FirewallConfig;

pub fn unblock() {
    std::process::Command::new("sudo")
        .arg("nft")
        .arg("flush")
        .arg("chain")
        .arg("inet")
        .arg("gaming_block")
        .arg("output")
        .output()
        .expect("failed to execute nft command");
}

pub fn block(config: &FirewallConfig) {
    let tcp_port_list: String = config.tcp_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
    let tcp_port_set = format!("{{ {} }}", tcp_port_list);

    std::process::Command::new("sudo")
        .arg("nft")
        .arg("add")
        .arg("rule")
        .arg("inet")
        .arg("gaming_block")
        .arg("output")
        .arg("tcp")
        .arg("dport")
        .arg(tcp_port_set)
        .arg("drop")
        .output()
        .expect("failed to execute nft command");

    let udp_port_list: String = config.udp_ports.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ");
    let udp_port_set = format!("{{ {} }}", udp_port_list);

    std::process::Command::new("sudo")
        .arg("nft")
        .arg("add")
        .arg("rule")
        .arg("inet")
        .arg("gaming_block")
        .arg("output")
        .arg("udp")
        .arg("dport")
        .arg(udp_port_set)
        .arg("drop")
        .output()
        .expect("failed to execute nft command");
}