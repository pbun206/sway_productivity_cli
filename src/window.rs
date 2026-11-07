use anyhow::{Context, Result};
use rand::prelude::*;

use smithay_client_toolkit::{
    compositor::CompositorState,
    output::OutputState,
    reexports::client::globals::registry_queue_init,
    registry::RegistryState,
    seat::SeatState,
    shell::{
        WaylandSurface,
        wlr_layer::{Anchor, KeyboardInteractivity, Layer, LayerShell},
    },
    shm::{Shm, slot::SlotPool},
};

use crate::{appstate::AppState, splash::get_splashes};

pub fn window() -> Result<()> {
    // https://github.com/Smithay/client-toolkit/blob/master/examples/simple_layer.rs
    let wayland_connection =
        wayland_client::Connection::connect_to_env().context("wayland socket failed")?;
    let (globals, mut event_queue) = registry_queue_init::<AppState>(&wayland_connection)
        .context("failed to register queue and globals")?;
    let qh = event_queue.handle();
    let compositor =
        CompositorState::bind(&globals, &qh).context("wl_compositor is not available")?;
    let layer_shell = LayerShell::bind(&globals, &qh).expect("layer shell is not available");
    let shm = Shm::bind(&globals, &qh).context("wl_shm is not available")?;
    let surface = compositor.create_surface(&qh);

    let layer =
        layer_shell.create_layer_surface(&qh, surface, Layer::Overlay, Some("meow later"), None);

    layer.set_anchor(Anchor::all());
    layer.set_keyboard_interactivity(KeyboardInteractivity::Exclusive);
    layer.set_size(0, 0);
    layer.set_exclusive_zone(-1);

    layer.commit();

    let pool = SlotPool::new(256 * 256 * 4, &shm).context("Failed to create pool")?;

    let mut rng = rand::rng();
    let splashes = get_splashes()?.content;
    let chosen_splash = splashes[rng.random_range(0..splashes.len())].clone();

    let mut app_state = AppState::new(
        RegistryState::new(&globals),
        SeatState::new(&globals, &qh),
        OutputState::new(&globals, &qh),
        chosen_splash,
        shm,
        pool,
        layer,
    );

    loop {
        event_queue.blocking_dispatch(&mut app_state).unwrap();

        if app_state.exit {
            break;
        }
    }
    Ok(())
}
