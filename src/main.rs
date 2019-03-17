//! Host 802.11 access point creation utility

use log::*;

fn main() {
    let args = hostap::cli::Args::parse();
    info!("Arguments: {:#?}", args);

    if !hostap::iproute2::interface_exists(args.interface.as_ref()) {
        eprintln!("interface {} does not exist!", args.interface);
        std::process::exit(1);
    }

    let interface = args.interface.as_ref();
    let gateway = args.gateway.as_ref();
    if args.command == hostap::cli::Command::Info {
        hostap::state::info(interface);
        return;
    }

    hostap::network_manager::ignore_interface(interface);
    hostap::state::down(interface, gateway);

    if args.command == hostap::cli::Command::Up {
        hostap::state::up(interface, gateway);
    }
}

/// Temporary debug function
#[allow(dead_code)]
fn sleep(secs: u64) {
    ::std::thread::sleep(::std::time::Duration::new(secs, 0));
}
