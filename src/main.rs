//! SigmaOS-integrity — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_integrity_lib::{Component, ComponentLifecycle};

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-integrity: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-integrity: ok");
}
