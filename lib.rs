use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, WebSocket};

static mut TIME: f64 = 0.0;

#[wasm_bindgen]
pub struct Game {
    ctx: CanvasRenderingContext2d,
    socket: Option<WebSocket>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, ws_url: Option<String>) -> Result<Game, JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let canvas = document
            .get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let socket = if let Some(url) = ws_url {
            let ws = WebSocket::new(&url)?;
            Some(ws)
        } else {
            None
        };

        Ok(Game { ctx, socket })
    }

    pub fn frame(&self, dt: f64) {
        unsafe { TIME += dt };

        let w = self.ctx.canvas().unwrap().width() as f64;
        let h = self.ctx.canvas().unwrap().height() as f64;

        self.ctx.set_fill_style(&"#111".into());
        self.ctx.fill_rect(0.0, 0.0, w, h);

        let x = 100.0 + unsafe { TIME }.sin() * 50.0;
        let y = h / 2.0;

        self.ctx.set_fill_style(&"#4af".into());
        self.ctx.fill_rect(x, y, 50.0, 50.0);
    }

    pub fn send(&self, msg: &str) {
        if let Some(ws) = &self.socket {
            let _ = ws.send_with_str(msg);
        }
    }
}
``
