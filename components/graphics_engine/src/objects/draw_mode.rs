pub enum DrawMode {
    Always,   // Draw always, even if it's not visible
    OnScreen, // Hide if it's off screen otherwise draw it
    Never,    // Never draw.
}
