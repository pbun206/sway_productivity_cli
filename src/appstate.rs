use std::{cmp::min, num::NonZeroU32};

use anyhow::{Context, Result};
use cosmic_text::{
    Align, Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, SwashCache, Wrap,
};
use ordered_float::OrderedFloat;
use smithay_client_toolkit::{
    compositor::CompositorHandler,
    delegate_registry,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{
        Capability, SeatHandler, SeatState,
        keyboard::{KeyEvent, KeyboardHandler, Keysym, Modifiers, RawModifiers},
        pointer::{PointerEvent, PointerEventKind, PointerHandler},
    },
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShellHandler, LayerSurface, LayerSurfaceConfigure},
    },
    shm::{Shm, ShmHandler, slot::SlotPool},
};
use wayland_client::{
    Connection, QueueHandle,
    protocol::{wl_keyboard, wl_output, wl_pointer, wl_seat, wl_shm, wl_surface},
};

delegate_registry!(AppState);
smithay_client_toolkit::delegate_dispatch2!(AppState);

pub struct AppState {
    pub registry_state: RegistryState,
    pub seat_state: SeatState,
    pub output_state: OutputState,
    pub shm: Shm,

    pub exit: bool,
    pub first_configure: bool,
    pub pool: SlotPool,

    pub width: u32,
    pub height: u32,

    pub layer: LayerSurface,
    pub keyboard: Option<wl_keyboard::WlKeyboard>,
    pub pointer: Option<wl_pointer::WlPointer>,

    pub splash: String,

    pub fontsystem: FontSystem,
    pub swash_cache: SwashCache,
}

impl ProvidesRegistryState for AppState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}

impl CompositorHandler for AppState {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_transform: wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
        self.draw_screen().unwrap();
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }
}

impl OutputHandler for AppState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }
}

impl LayerShellHandler for AppState {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _layer: &LayerSurface) {
        self.exit = true;
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) {
        self.width = NonZeroU32::new(configure.new_size.0).map_or(256, NonZeroU32::get);
        self.height = NonZeroU32::new(configure.new_size.1).map_or(256, NonZeroU32::get);

        // Initiate the first draw.
        if self.first_configure {
            self.first_configure = false;
            self.draw_screen().unwrap();
        }
    }
}

impl SeatHandler for AppState {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_none() {
            let keyboard = self
                .seat_state
                .get_keyboard(qh, &seat, None)
                .expect("Failed to create keyboard");
            self.keyboard = Some(keyboard);
        }

        if capability == Capability::Pointer && self.pointer.is_none() {
            let pointer = self
                .seat_state
                .get_pointer(qh, &seat)
                .expect("Failed to create pointer");
            self.pointer = Some(pointer);
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _: &QueueHandle<Self>,
        _: wl_seat::WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_some() {
            self.keyboard.take().unwrap().release();
        }

        if capability == Capability::Pointer && self.pointer.is_some() {
            self.pointer.take().unwrap().release();
        }
    }

    fn remove_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}
}

impl KeyboardHandler for AppState {
    fn enter(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,
        _: u32,
        _: &[u32],
        _keysyms: &[Keysym],
    ) {
    }

    fn leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _surface: &wl_surface::WlSurface,
        _: u32,
    ) {
    }

    fn press_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _event: KeyEvent,
    ) {
        self.exit = true;
    }

    fn repeat_key(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _keyboard: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _event: KeyEvent,
    ) {
    }

    fn release_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _: KeyEvent,
    ) {
    }

    fn update_modifiers(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _serial: u32,
        _modifiers: Modifiers,
        _raw_modifiers: RawModifiers,
        _layout: u32,
    ) {
    }
}

impl PointerHandler for AppState {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &wl_pointer::WlPointer,
        events: &[PointerEvent],
    ) {
        use PointerEventKind::*;
        for event in events {
            if let Press { .. } = event.kind {
                self.exit = true;
            }
        }
    }
}

impl ShmHandler for AppState {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

impl AppState {
    pub fn new(
        registry_state: RegistryState,
        seat_state: SeatState,
        output_state: OutputState,
        splash: String,
        shm: Shm,
        pool: SlotPool,
        layer: LayerSurface,
    ) -> Self {
        AppState {
            registry_state,
            seat_state,
            output_state,
            shm,
            exit: false,
            first_configure: true,
            pool,
            width: 0,
            height: 0,
            layer,
            keyboard: None,
            pointer: None,
            fontsystem: FontSystem::new(),
            swash_cache: SwashCache::new(),
            splash,
        }
    }

    // font metrics and spacing constants
    pub const TAGLINE_FONT_SIZE: f32 = 20.0;
    pub const TAGLINE_LINE_HEIGHT: f32 = 30.0;

    pub const BODY_FONT_SIZE: f32 = 16.0;
    pub const BODY_LINE_HEIGHT: f32 = 24.0;

    pub const PRESS_GAP_Y: u32 = 150;
    pub const SPLASH_BOTTOM_GAP_Y: u32 = 150;
    pub const SPLASH_WRAP_WIDTH: f32 = 450.0;

    pub fn draw_screen(&mut self) -> Result<()> {
        let width = self.width;
        let height = self.height;
        let stride = self.width as i32 * 4;

        let white = Color::rgb(255, 255, 255);
        let default_attrs = Attrs::new().color(white).family(Family::Monospace);

        let mut tagline_buffer = Buffer::new(
            &mut self.fontsystem,
            Metrics::new(Self::TAGLINE_FONT_SIZE, Self::TAGLINE_LINE_HEIGHT),
        );
        tagline_buffer.set_text(
            "take sigh",
            &default_attrs,
            Shaping::Basic,
            Some(Align::Center),
        );

        let mut desp_buffer = Buffer::new(
            &mut self.fontsystem,
            Metrics::new(Self::BODY_FONT_SIZE, Self::BODY_LINE_HEIGHT),
        );
        desp_buffer.set_text(
            "a long inhale through the nose\na quick inhale through the nose\nexhale",
            &default_attrs,
            Shaping::Basic,
            None,
        );

        let mut press_buffer = Buffer::new(
            &mut self.fontsystem,
            Metrics::new(Self::BODY_FONT_SIZE, Self::BODY_LINE_HEIGHT),
        );
        press_buffer.set_text(
            "press any key to exit",
            &default_attrs,
            Shaping::Basic,
            Some(Align::Center),
        );

        let mut splash_buffer = Buffer::new(
            &mut self.fontsystem,
            Metrics::new(Self::BODY_FONT_SIZE, Self::BODY_LINE_HEIGHT),
        );
        splash_buffer.set_text(
            &self.splash,
            &default_attrs,
            Shaping::Basic,
            None,
        );
        splash_buffer.set_wrap(Wrap::Word);
        splash_buffer.set_size(Some(Self::SPLASH_WRAP_WIDTH), None);

        let (buffer, canvas) = self
            .pool
            .create_buffer(
                width as i32,
                height as i32,
                stride,
                wl_shm::Format::Argb8888,
            )
            .expect("create buffer");

        for chunk in canvas.chunks_exact_mut(4) {
            chunk[0] = 0; // Blue
            chunk[1] = 0; // Green
            chunk[2] = 0; // Red
            chunk[3] = 255; // Alpha (fully opaque)
        }

        splash_buffer.shape_until_scroll(&mut self.fontsystem, false);
        desp_buffer.shape_until_scroll(&mut self.fontsystem, false);
        tagline_buffer.shape_until_scroll(&mut self.fontsystem, false);
        press_buffer.shape_until_scroll(&mut self.fontsystem, false);

        let screen_center: i32 = self.width as i32 / 2;

        let tagline_height = Self::TAGLINE_LINE_HEIGHT as u32;
        let desp_height = (3.0 * Self::BODY_LINE_HEIGHT) as u32;
        let text_block_height = tagline_height + desp_height;
        let splash_height =
            splash_buffer.layout_runs().count() as u32 * Self::BODY_LINE_HEIGHT as u32;
        let center_height = self.height / 2;
        let tagline_y = center_height - text_block_height / 2;
        let desp_y = tagline_y + tagline_height;
        let press_y = desp_y + Self::PRESS_GAP_Y;
        let splash_y = height - Self::SPLASH_BOTTOM_GAP_Y - splash_height;

        let splash_width = splash_buffer
            .layout_runs()
            .max_by_key(|line| OrderedFloat(line.line_w))
            .context("splash with no lines")?
            .line_w;

        let desp_width = desp_buffer
            .layout_runs()
            .max_by_key(|line| OrderedFloat(line.line_w))
            .context("descp with no lines")?
            .line_w;

        let tagline_width = tagline_buffer
            .layout_runs()
            .max_by_key(|line| OrderedFloat(line.line_w))
            .context("tagline with no lines")?
            .line_w;

        let press_width = press_buffer
            .layout_runs()
            .max_by_key(|line| OrderedFloat(line.line_w))
            .context("press with no lines")?
            .line_w;

        tagline_buffer.draw(
            &mut self.fontsystem,
            &mut self.swash_cache,
            white,
            |x, y, w, h, color| {
                Self::draw_rect(
                    canvas,
                    self.width,
                    self.height,
                    (x + screen_center - tagline_width as i32 / 2) as u32,
                    y as u32 + tagline_y,
                    w,
                    h,
                    color,
                );
            },
        );

        desp_buffer.draw(
            &mut self.fontsystem,
            &mut self.swash_cache,
            white,
            |x, y, w, h, color| {
                Self::draw_rect(
                    canvas,
                    self.width,
                    self.height,
                    (x + screen_center - desp_width as i32 / 2) as u32,
                    y as u32 + desp_y,
                    w,
                    h,
                    color,
                );
            },
        );

        press_buffer.draw(
            &mut self.fontsystem,
            &mut self.swash_cache,
            white,
            |x, y, w, h, color| {
                Self::draw_rect(
                    canvas,
                    self.width,
                    self.height,
                    (x + screen_center - press_width as i32 / 2) as u32,
                    y as u32 + press_y,
                    w,
                    h,
                    color,
                );
            },
        );

        splash_buffer.draw(
            &mut self.fontsystem,
            &mut self.swash_cache,
            white,
            |x, y, w, h, color| {
                Self::draw_rect(
                    canvas,
                    self.width,
                    self.height,
                    (x + screen_center - splash_width as i32 / 2) as u32,
                    y as u32 + splash_y,
                    w,
                    h,
                    color,
                );
            },
        );

        // Damage the entire window
        self.layer
            .wl_surface()
            .damage_buffer(0, 0, width as i32, height as i32);

        // Attach and commit to present.
        buffer
            .attach_to(self.layer.wl_surface())
            .expect("buffer attach");
        self.layer.commit();
        Ok(())
    }

    // does not support negatives
    // x and y -> top left position of thing
    pub fn draw_rect(
        canvas: &mut [u8],
        canvas_width: u32,
        canvas_height: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
        color: Color,
    ) {
        for j in y..min(y + h, canvas_height) {
            for i in x..min(x + w, canvas_width) {
                let idx = 4 * (canvas_width * j + i) as usize;
                let a = color.a() as u32;
                canvas[idx] += ((color.b() as u32 * a) / 255) as u8;
                canvas[idx + 1] += ((color.g() as u32 * a) / 255) as u8;
                canvas[idx + 2] += ((color.r() as u32 * a) / 255) as u8;
            }
        }
    }
}
