use crowsa::prelude::*;

fn main() -> Result<(), CrowsaError> {
    let config = CrowsaConfig::new()
        .frontend("./examples/frontend/basic")
        .window_title("Hello World")
        .resizable(SizeHint::FIXED);

    let crowsa = Crowsa::new(config)?;
    crowsa.run();
    Ok(())
}
