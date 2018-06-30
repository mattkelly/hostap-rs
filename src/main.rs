//! Host 802.11 access point creation utility

extern crate hostap;

fn main() {
    let args = hostap::cli::Args::parse();
    println!("Arguments: {:#?}", args);

    if !hostap::core::interface_exists(args.interface.as_ref()) {
        println!("interface {} does not exist!", args.interface);
        return;
    }

    hostap::network_manager::ignore_interface(args.interface.as_ref());
}
