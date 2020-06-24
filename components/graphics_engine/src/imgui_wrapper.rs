use imgui::{Ui, Context};
use sdl2::video::Window;
use sdl2::VideoSubsystem;

pub use { imgui_sdl2::ImguiSdl2, imgui_opengl_renderer::Renderer };
use sdl2::mouse::MouseState;

pub struct ImGui {
    imgui: Box<imgui::Context>,
    imgui_sdl2: Box<imgui_sdl2::ImguiSdl2>,
    imgui_renderer: Box<imgui_opengl_renderer::Renderer>
}

impl ImGui {
    pub fn new(sdl_video: &VideoSubsystem, window: &Window) -> ImGui {
        let mut context = imgui::Context::create();
        context.fonts().build_rgba32_texture();

        let sdl2 = imgui_sdl2::ImguiSdl2::new(&mut context, window);

        let renderer = imgui_opengl_renderer::Renderer::new(&mut context,
            |s| sdl_video.gl_get_proc_address(s) as _);

        ImGui {
            imgui: Box::new(context),
            imgui_sdl2: Box::new(sdl2),
            imgui_renderer: Box::new(renderer)
        }
    }

    pub fn ui(&mut self, window: &Window, mouse_state: &MouseState) -> Ui<'_> {
        self.imgui_sdl2.prepare_frame(self.imgui.io_mut(), window, &mouse_state);

        self.imgui.frame()
    }

    pub unsafe fn imgui_sdl2(&mut self) -> *mut ImguiSdl2 {
        self.imgui_sdl2.as_mut() as *mut ImguiSdl2
    }

    pub unsafe fn imgui_renderer(&mut self) -> *const Renderer {
        self.imgui_renderer.as_mut() as *const Renderer
    }

    pub unsafe fn imgui(&mut self) -> *mut Context {
        self.imgui.as_mut() as *mut Context
    }
}
