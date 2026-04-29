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
        console_error_panic_hook::set_once();
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

    pub fn frame(&self, _dt: f64) {
    let canvas = self.ctx.canvas().unwrap();

    self.ctx.set_fill_style(&"black".into());
    self.ctx.fill_rect(
        0.0,
        0.0,
        canvas.width() as f64,
        canvas.height() as f64,
    );

    self.ctx.set_fill_style(&"red".into());
    self.ctx.fill_rect(50.0, 50.0, 200.0, 200.0);
}

    pub fn send(&self, msg: &str) {
        if let Some(ws) = &self.socket {
            let _ = ws.send_with_str(msg);
        }
    }
}
