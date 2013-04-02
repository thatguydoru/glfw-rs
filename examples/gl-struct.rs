/*
This example illustrates how a OpenGL function struct might be
be used for rendering a viewport resize from inside a callback.
*/

extern mod glfw;

type GLint = i32;

// This would be supplied be an external library
struct GL {
    // The fields would actually contain pointers to extern functions
    Viewport: @fn(x: GLint, y: GLint, width: GLint, height: GLint)
}

impl GL {
    fn init() -> GL {
        GL {
            Viewport: |x, y, width, height| {
                // Obviously this doesn't actually call glViewport
                io::println(fmt!("glResize(%?, %?, %?, %?)", x, y, width, height));
            }
        }
    }
    
    #[inline(always)]
    fn Viewport(&self, x: GLint, y: GLint, width: GLint, height: GLint) {
        (self.Viewport)(x, y, width, height)
    }
}

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        let window = glfw::Window::create(640, 480, "Resize the window to call glViewport", glfw::Windowed).unwrap();
        
        window.make_context_current();
        
        let gl = GL::init();
        
        do window.set_size_callback |_, width, height| {
            // Rust doesn't have global state, so we use a borrowed pointer
            // to the gl struct in order to perform rendering operations
            // in other functions
            render_resize(&gl, width, height);
        }
        
        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

fn render_resize(gl: &GL, width: int, height: int) {
    gl.Viewport(0, 0, width as GLint, height as GLint);
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}