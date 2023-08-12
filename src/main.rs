use tracing;
use tracing_subscriber::{self, EnvFilter};

fn main() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut gba = gba::GbaCore::default();
    let bytes = include_bytes!("../tests/roms/arm.gba");

    gba.load_rom(bytes);
    gba.skip_bios();

    let mut i = 0;
    loop {
        log::debug!("Tick {i}");
        gba.tick();

        i += 1;
    }
}
