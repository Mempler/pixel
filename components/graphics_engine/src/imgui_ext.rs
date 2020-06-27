use imgui::*;

pub trait UiChildExt {
    fn child<S: AsRef<ImStr>, F: FnOnce()>(&self, str_id: S, size: [f32; 2], border: bool, flags: WindowFlags, f: F);
}

impl<'ui> UiChildExt for Ui<'ui> {
    fn child<S: AsRef<ImStr>, F: FnOnce()>(&self, str_id: S, size: [f32; 2], border: bool, flags: WindowFlags, f: F) {
        unsafe { sys::igBeginChild(str_id.as_ref().as_ptr(), sys::ImVec2::new(size[0], size[1]), border, flags.bits() as i32) };
        f();
        unsafe { sys::igEndChild() };
    }
}
