use std::{cell::RefCell, f64::consts::PI, rc::Rc};
use wasm_bindgen::{JsCast, closure::Closure, prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, js_sys};

const S: f64 = 512.0;
type Shared<T> = Rc<RefCell<T>>;
type SharedOpts = Shared<Opts>;
type SharedTick = Shared<i32>;
type SharedLines = Shared<Vec<Line>>;
type FrameClosure = Closure<dyn FnMut()>;
type SharedFrame = Shared<Option<FrameClosure>>;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = Rc::new(web_sys::window().unwrap());
    let document = window.document().unwrap();

    // Canvas setup
    let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
    let ctx: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;
    let container = document.get_element_by_id("container").unwrap();
    container.set_class_name(&format!("{} center-align", container.class_name()));
    container.append_child(&document.create_element("br")?.into())?;
    container.append_child(&canvas)?;
    let h = document.create_element("h4")?;
    h.set_inner_html("Hello from Rust WASM!");
    container.append_child(&h)?;
    canvas.set_width(S as u32);
    canvas.set_height(S as u32);
    ctx.set_fill_style_str("black");
    ctx.fill_rect(0.0, 0.0, S, S);

    // Shared state
    let ctx_rc = Rc::new(ctx);
    let opts_rc: SharedOpts = Rc::new(RefCell::new(Opts {
        cx: S / 2.0,
        cy: S / 2.0,
        ..Default::default()
    }));
    let tick_rc: SharedTick = Rc::new(RefCell::new(0));
    let lines_rc: SharedLines = Rc::new(RefCell::new(vec![]));
    let canvas_rc = Rc::new(canvas);

    // Resize handler
    {
        let (w, c, ctx, o) = (
            window.clone(),
            canvas_rc.clone(),
            ctx_rc.clone(),
            opts_rc.clone(),
        );
        let closure = Closure::wrap(Box::new(move || {
            let wn = w.inner_width().unwrap().as_f64().unwrap();
            let hn = w.inner_height().unwrap().as_f64().unwrap();
            c.set_width(wn as u32);
            c.set_height(hn as u32);
            ctx.set_fill_style_str("black");
            ctx.fill_rect(0.0, 0.0, wn, hn);
            let mut o = o.borrow_mut();
            o.cx = wn / 2.0;
            o.cy = hn / 2.0;
        }) as Box<dyn FnMut()>);
        window.set_onresize(Some(closure.as_ref().unchecked_ref()));
        closure.forget();
    }

    // Animation loop
    {
        let frame: SharedFrame = Rc::new(RefCell::new(None));
        let f_ref = frame.clone();
        let (w1, w2, ctx, opts, tick, lines) = (
            window.clone(),
            window.clone(),
            ctx_rc.clone(),
            opts_rc.clone(),
            tick_rc.clone(),
            lines_rc.clone(),
        );

        *f_ref.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            *tick.borrow_mut() += 1;
            let (t, o) = (*tick.borrow(), *opts.borrow());
            let ws = w1.inner_width().unwrap().as_f64().unwrap();
            let hs = w1.inner_height().unwrap().as_f64().unwrap();

            ctx.set_global_composite_operation("source-over").unwrap();
            ctx.set_shadow_blur(0.0);
            ctx.set_fill_style_str(&format!("rgba(0,0,0,{})", o.repaint_alpha));
            ctx.fill_rect(0.0, 0.0, ws, hs);

            ctx.set_global_composite_operation("lighter").unwrap();
            if lines.borrow().len() < o.count && js_sys::Math::random() < o.spawn_chance {
                lines.borrow_mut().push(Line::new());
            }
            for l in lines.borrow_mut().iter_mut() {
                l.step(&ctx, o, t);
            }

            w2.request_animation_frame(frame.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));

        window
            .request_animation_frame(f_ref.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;
    }

    Ok(())
}

// ----------- Config -----------
#[derive(Clone, Copy)]
struct Opts {
    len: f64,
    count: usize,
    base_time: i32,
    added_time: i32,
    spawn_chance: f64,
    spark_chance: f64,
    spark_dist: f64,
    spark_size: f64,
    base_light: f64,
    added_light: f64,
    shadow_to_time_prop_mult: f64,
    cx: f64,
    cy: f64,
    repaint_alpha: f64,
    hue_change: f64,
}
impl Default for Opts {
    fn default() -> Self {
        Self {
            len: 20.0,
            count: 300,
            base_time: 10,
            added_time: 10,
            spawn_chance: 1.0,
            spark_chance: 0.1,
            spark_dist: 10.0,
            spark_size: 2.0,
            base_light: 50.0,
            added_light: 10.0,
            shadow_to_time_prop_mult: 6.0,
            cx: 0.0,
            cy: 0.0,
            repaint_alpha: 0.04,
            hue_change: 0.1,
        }
    }
}

// ----------- Drawing -----------
struct Line {
    x: f64,
    y: f64,
    added_x: f64,
    added_y: f64,
    rad: f64,
    light_input_multiplier: f64,
    cumulative_time: i32,
    time: i32,
    target_time: i32,
}
impl Line {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            added_x: 0.0,
            added_y: 0.0,
            rad: 0.0,
            light_input_multiplier: 0.0,
            cumulative_time: 0,
            time: 0,
            target_time: 0,
        }
    }
    fn begin_phase(&mut self, o: Opts) {
        self.x += self.added_x;
        self.y += self.added_y;
        self.time = 0;
        self.target_time =
            (o.base_time as f64 + o.added_time as f64 * js_sys::Math::random()) as i32;
        self.rad +=
            PI * if js_sys::Math::random() < 0.5 {
                1.0
            } else {
                -1.0
            } / 3.0;
        self.added_x = self.rad.cos();
        self.added_y = self.rad.sin();
    }
    fn step(&mut self, c: &CanvasRenderingContext2d, o: Opts, t: i32) {
        self.time += 1;
        self.cumulative_time += 1;
        if self.time >= self.target_time {
            self.begin_phase(o);
        }
        let p = self.time as f64 / self.target_time as f64;
        let w = (p * PI / 2.0).sin();
        let (x, y) = (self.added_x * w, self.added_y * w);
        let l = o.base_light
            + o.added_light * ((self.cumulative_time as f64 * self.light_input_multiplier).sin());
        let h = format!("hsl({},{:.0}%,{:.0}%)", t as f64 * o.hue_change, 100.0, l);
        c.set_shadow_blur(p * o.shadow_to_time_prop_mult);
        c.set_fill_style_str(&h);
        c.set_shadow_color(&h);
        let (px, py) = (o.cx + (self.x + x) * o.len, o.cy + (self.y + y) * o.len);
        c.fill_rect(px, py, 2.0, 2.0);
        if js_sys::Math::random() < o.spark_chance {
            let ox = (js_sys::Math::random() * o.spark_dist)
                * if js_sys::Math::random() < 0.5 {
                    1.0
                } else {
                    -1.0
                };
            let oy = (js_sys::Math::random() * o.spark_dist)
                * if js_sys::Math::random() < 0.5 {
                    1.0
                } else {
                    -1.0
                };
            c.fill_rect(
                px + ox - o.spark_size / 2.0,
                py + oy - o.spark_size / 2.0,
                o.spark_size,
                o.spark_size,
            );
        }
    }
}
