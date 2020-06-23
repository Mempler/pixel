pub use sdl2::event::WindowEvent;
pub use sdl2::keyboard::{Keycode, Scancode, Mod};
pub use sdl2::mouse::{MouseState, MouseButton, MouseWheelDirection};
pub use sdl2::joystick::HatState;
pub use sdl2::controller::{Axis, Button};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    // SDL2 Events
    Quit { timestamp: u32 },
    AppTerminating { timestamp: u32 },
    AppLowMemory { timestamp: u32 },
    AppWillEnterBackground { timestamp: u32 },
    AppDidEnterBackground { timestamp: u32 },
    AppWillEnterForeground { timestamp: u32 },
    AppDidEnterForeground { timestamp: u32 },
    Window {
        timestamp: u32,
        window_id: u32,
        win_event: WindowEvent,
    },
    // Controls
    KeyDown {
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool
    },
    KeyUp {
        timestamp: u32,
        window_id: u32,
        keycode: Option<Keycode>,
        scancode: Option<Scancode>,
        keymod: Mod,
        repeat: bool
    },
    MouseMotion {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mousestate: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32
    },
    MouseButtonDown {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32
    },
    MouseButtonUp {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: i32,
        y: i32
    },
    MouseWheel {
        timestamp: u32,
        window_id: u32,
        which: u32,
        x: i32,
        y: i32,
        direction: MouseWheelDirection,
    },
    JoyAxisMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        axis_idx: u8,
        value: i16
    },
    JoyBallMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        ball_idx: u8,
        xrel: i16,
        yrel: i16
    },
    JoyHatMotion {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        hat_idx: u8,
        state: HatState
    },
    JoyButtonDown {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        button_idx: u8
    },
    JoyButtonUp {
        timestamp: u32,
        /// The joystick's `id`
        which: u32,
        button_idx: u8
    },
    JoyDeviceAdded {
        timestamp: u32,
        /// The newly added joystick's `joystick_index`
        which: u32
    },
    JoyDeviceRemoved {
        timestamp: u32,
        /// The joystick's `id`
        which: u32
    },
    ControllerAxisMotion {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
        axis: Axis,
        value: i16
    },
    ControllerButtonDown {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
        button: Button
    },
    ControllerButtonUp {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32,
        button: Button
    },
    ControllerDeviceAdded {
        timestamp: u32,
        /// The newly added controller's `joystick_index`
        which: u32
    },
    ControllerDeviceRemoved {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32
    },
    ControllerDeviceRemapped {
        timestamp: u32,
        /// The controller's joystick `id`
        which: u32
    },
    FingerDown {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerUp {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerMotion {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },

    Unknown
}

impl Event {
    pub fn from_sdl2_event(ev: sdl2::event::Event) -> Event {
        match ev {
            sdl2::event::Event::Quit { timestamp } => Event::Quit { timestamp },
            sdl2::event::Event::AppTerminating { timestamp } => Event::AppTerminating { timestamp },
            sdl2::event::Event::AppLowMemory { timestamp } => Event::AppLowMemory { timestamp },
            sdl2::event::Event::AppWillEnterBackground { timestamp } => Event::AppWillEnterBackground { timestamp },
            sdl2::event::Event::AppDidEnterBackground { timestamp } => Event::AppDidEnterBackground { timestamp },
            sdl2::event::Event::AppWillEnterForeground { timestamp } => Event::AppWillEnterForeground { timestamp },
            sdl2::event::Event::AppDidEnterForeground { timestamp } => Event::AppDidEnterForeground { timestamp },
            sdl2::event::Event::Window {
                timestamp,
                window_id,
                win_event,
            }  => Event::Window { timestamp, window_id, win_event, },

            sdl2::event::Event::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat
            } => Event::KeyDown { timestamp, window_id, keycode, scancode, keymod, repeat },
            sdl2::event::Event::KeyUp {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat
            } => Event::KeyUp { timestamp, window_id, keycode, scancode, keymod, repeat },
            sdl2::event::Event::MouseMotion {
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel
            } => Event::MouseMotion { timestamp, window_id, which, mousestate, x, y, xrel, yrel },
            sdl2::event::Event::MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y
            } => Event::MouseButtonDown { timestamp, window_id, which, mouse_btn, clicks, x, y },
            sdl2::event::Event::MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y
            } => Event::MouseButtonUp { timestamp, window_id, which, mouse_btn, clicks, x, y },
            sdl2::event::Event::MouseWheel {
                timestamp,
                window_id,
                which,
                x,
                y,
                direction,
            } => Event::MouseWheel { timestamp, window_id, which, x, y, direction },
            sdl2::event::Event::JoyAxisMotion {
                timestamp,
                which,
                axis_idx,
                value
            } => Event::JoyAxisMotion { timestamp, which, axis_idx, value },
            sdl2::event::Event::JoyBallMotion {
                timestamp,
                which,
                ball_idx,
                xrel,
                yrel
            } => Event::JoyBallMotion { timestamp, which, ball_idx, xrel, yrel },
            sdl2::event::Event::JoyHatMotion {
                timestamp,
                which,
                hat_idx,
                state
            } => Event::JoyHatMotion{ timestamp, which, hat_idx, state },
            sdl2::event::Event::JoyButtonDown {
                timestamp,
                which,
                button_idx
            } => Event::JoyButtonDown { timestamp, which, button_idx },
            sdl2::event::Event::JoyButtonUp {
                timestamp,
                which,
                button_idx
            } => Event::JoyButtonUp { timestamp, which, button_idx },
            sdl2::event::Event::JoyDeviceAdded {
                timestamp,
                which
            } => Event::JoyDeviceAdded { timestamp, which },
            sdl2::event::Event::JoyDeviceRemoved {
                timestamp,
                which
            } => Event::JoyDeviceRemoved { timestamp, which },
            sdl2::event::Event::ControllerAxisMotion {
                timestamp,
                which,
                axis,
                value
            } => Event::ControllerAxisMotion { timestamp, which, axis, value },
            sdl2::event::Event::ControllerButtonDown {
                timestamp,
                which,
                button
            } => Event::ControllerButtonDown { timestamp, which, button },
            sdl2::event::Event::ControllerButtonUp {
                timestamp,
                which,
                button
            } => Event::ControllerButtonUp { timestamp, which, button },
            sdl2::event::Event::ControllerDeviceAdded {
                timestamp,
                which
            } => Event::ControllerDeviceAdded { timestamp, which },
            sdl2::event::Event::ControllerDeviceRemoved {
                timestamp,
                which
            } => Event::ControllerDeviceRemoved { timestamp, which },
            sdl2::event::Event::ControllerDeviceRemapped {
                timestamp,
                which
            } => Event::ControllerDeviceRemapped { timestamp, which },
            sdl2::event::Event::FingerDown {
                timestamp,
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure
            } => Event::FingerDown { timestamp, touch_id, finger_id, x, y, dx, dy, pressure },
            sdl2::event::Event::FingerUp {
                timestamp,
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure
            } => Event::FingerUp { timestamp, touch_id, finger_id, x, y, dx, dy, pressure },
            sdl2::event::Event::FingerMotion {
                timestamp,
                touch_id,
                finger_id,
                x,
                y,
                dx,
                dy,
                pressure
            } => Event::FingerMotion { timestamp, touch_id, finger_id, x, y, dx, dy, pressure },

            _ => Event::Unknown
        }
    }
}

type HandlerPtr = Box<dyn FnMut(&Event)>;

pub struct EventPipeline {
    event_handlers: Vec<HandlerPtr>,
    event_queue: Vec<Event>
}

impl EventPipeline {
    pub fn new() -> EventPipeline {
        EventPipeline {
            event_queue: vec![],
            event_handlers: vec![],
        }
    }

    pub fn push_event(&mut self, ev: Event) {
        self.event_queue.push(ev);
    }

    /// Registers an event handler
    ///
    /// # Example:
    ///
    /// ```
    /// use event_pipeline::EventPipeline;
    ///
    /// let mut ev_pipeline = EventPipeline::new();
    /// ev_pipeline.register_handler(|ev| {
    ///     match ev {
    ///         _ => unimplemented!("Unimplemented event: {}", ev)
    ///     }
    /// });
    /// ```
    pub fn register_handler<F: 'static + Fn(&Event)>(&mut self, handler: F) {
        self.event_handlers.push(Box::new(handler));
    }

    // This gets called after update frame
    // TODO: improve performance by sort each event in a hashmap
    pub fn handle(&mut self) {
        // NOTE: this can slow down quite a bit if there are a
        // of events and a lot of handlers!
        // TODO: improve
        for ev in &self.event_queue {
            for event_handler in &mut self.event_handlers {
                event_handler(ev);
            }
        }
    }

    // This gets called every end of frame
    pub fn finish(&mut self) {
        self.event_queue.clear();
    }
}
