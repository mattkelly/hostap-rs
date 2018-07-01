//! Host 802.11 access point creation utility

extern crate hostap;

fn main() {
    let args = hostap::cli::Args::parse();
    println!("Arguments: {:#?}", args);

    if !hostap::iproute2::interface_exists(args.interface.as_ref()) {
        println!("interface {} does not exist!", args.interface);
        return;
    }

    let interface = args.interface.as_ref();
    let gateway = args.gateway.as_ref();
    hostap::network_manager::ignore_interface(interface);

    hostap::iptables::down(interface, gateway);
    hostap::dhcpd::down();
    hostap::hostapd::down();
    hostap::iproute2::interface_down(interface);

    hostap::iproute2::interface_up(interface);
    hostap::hostapd::up(interface);
    hostap::dhcpd::up(interface);
    hostap::iptables::up(interface, gateway);
}

/// Temporary debug function
fn sleep(secs: u64) {
    ::std::thread::sleep(::std::time::Duration::new(secs, 0));
}
