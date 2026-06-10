use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const GLYPHS: &[char] = &[
    '\u{2500}', '\u{2502}', '\u{250C}', '\u{2510}', '\u{2514}', '\u{2518}',
    '\u{251C}', '\u{2524}', '\u{252C}', '\u{2534}', '\u{253C}', '\u{256D}',
    '\u{256E}', '\u{256F}', '\u{2570}', '\u{2571}', '\u{2572}', '\u{2573}',
    '\u{25CB}', '\u{25CF}', '\u{25A0}', '\u{25A1}', '\u{25B2}', '\u{25B3}',
    '\u{25B6}', '\u{25BC}', '\u{25C6}', '\u{25C7}', '\u{25D8}', '\u{25D9}',
    '\u{25E6}', '\u{2605}', '\u{2606}', '\u{2660}', '\u{2663}', '\u{2665}',
    '\u{2666}',
];

const FULL: &str = "the software company of hong kong";
const SHORT: &str = "tsc.hk";
const SUFFIX: &str = " \u{2014} copied!";

#[wasm_bindgen]
pub fn crepus_render(bundle_json: &str) -> Result<String, JsValue> {
    crepuscularity_web::render_bundle(bundle_json).map_err(|e| JsValue::from_str(&e.to_string()))
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no window")
}

fn rand_glyph() -> char {
    let r = js_sys::Math::random();
    let idx = (r * GLYPHS.len() as f64).floor() as usize;
    GLYPHS[idx.min(GLYPHS.len() - 1)]
}

type TimerHandle = Rc<RefCell<Option<i32>>>;

fn clear_timer(handle: &TimerHandle) {
    if let Some(id) = handle.borrow_mut().take() {
        window().clear_interval_with_handle(id);
    }
}

fn set_interval(closure: &Closure<dyn FnMut()>, ms: i32) -> i32 {
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            ms,
        )
        .expect("set_interval")
}

/// Scramble all positions with random glyphs, then reveal `target` left-to-right.
fn animate_text(
    el: &web_sys::Element,
    target: &str,
    speed: i32,
    on_done: Option<Closure<dyn FnMut()>>,
) -> TimerHandle {
    let from = el.text_content().unwrap_or_default();
    let max_len = from.len().max(target.len());

    let mut from_padded = from;
    let mut target_padded = target.to_string();
    while from_padded.len() < max_len {
        from_padded.push(' ');
    }
    while target_padded.len() < max_len {
        target_padded.push(' ');
    }

    let chars: Rc<RefCell<Vec<char>>> = Rc::new(RefCell::new(from_padded.chars().collect()));
    let target_chars: Vec<char> = target_padded.chars().collect();
    let phase: Rc<RefCell<u8>> = Rc::new(RefCell::new(0));
    let pos: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let total = max_len;
    let el = el.clone();
    let target_str = target.to_string();
    let timer: TimerHandle = Rc::new(RefCell::new(None));
    let timer_ref = timer.clone();

    let closure = Closure::wrap(Box::new(move || {
        let mut c = chars.borrow_mut();
        let mut p = pos.borrow_mut();
        let mut ph = phase.borrow_mut();

        if *ph == 0 {
            c[*p] = rand_glyph();
            for j in (*p + 1)..total {
                c[j] = rand_glyph();
            }
            el.set_text_content(Some(&c.iter().collect::<String>()));
            *p += 1;
            if *p >= total {
                *ph = 1;
                *p = 0;
            }
        } else {
            c[*p] = target_chars[*p];
            el.set_text_content(Some(&c.iter().collect::<String>()));
            *p += 1;
            if *p >= total {
                el.set_text_content(Some(&target_str));
                clear_timer(&timer_ref);
                if let Some(ref cb) = on_done {
                    let func: &js_sys::Function = cb.as_ref().unchecked_ref();
                    func.call0(&JsValue::NULL).ok();
                }
            }
        }
    }) as Box<dyn FnMut()>);

    let id = set_interval(&closure, speed);
    *timer.borrow_mut() = Some(id);
    closure.forget();
    timer
}

#[wasm_bindgen(start)]
pub fn start() {
    poll_for_heading(0, 100);
}

fn poll_for_heading(delay_ms: i32, remaining: u32) {
    let callback = Closure::<dyn FnMut()>::wrap(Box::new(move || {
        let doc = window().document().expect("document");
        if let Some(el) = doc.get_element_by_id("tsc-heading") {
            bind_heading(el);
        } else if remaining > 0 {
            poll_for_heading(100, remaining - 1);
        }
    }));
    let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        delay_ms,
    );
    callback.forget();
}

fn bind_heading(el: web_sys::Element) {
    let h_timer: TimerHandle = Rc::new(RefCell::new(None));
    let h_animating = Rc::new(RefCell::new(false));
    let click_timer: TimerHandle = Rc::new(RefCell::new(None));

    // mouseenter → scramble to SHORT
    {
        let el_inner = el.clone();
        let timer = h_timer.clone();
        let animating = h_animating.clone();
        let closure = Closure::wrap(Box::new(move || {
            if *animating.borrow() {
                return;
            }
            *animating.borrow_mut() = true;
            clear_timer(&timer);
            let anim = animating.clone();
            let done_cb = Closure::wrap(Box::new(move || {
                *anim.borrow_mut() = false;
            }) as Box<dyn FnMut()>);
            let t = animate_text(&el_inner, SHORT, 20, Some(done_cb));
            *timer.borrow_mut() = t.borrow_mut().take();
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("mouseenter", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // mouseleave → scramble back to FULL
    {
        let el_inner = el.clone();
        let timer = h_timer.clone();
        let animating = h_animating.clone();
        let closure = Closure::wrap(Box::new(move || {
            clear_timer(&timer);
            let anim = animating.clone();
            let done_cb = Closure::wrap(Box::new(move || {
                *anim.borrow_mut() = false;
            }) as Box<dyn FnMut()>);
            let t = animate_text(&el_inner, FULL, 20, Some(done_cb));
            *timer.borrow_mut() = t.borrow_mut().take();
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // click → copy, then scroll " — copied!" to the right
    {
        let el_inner = el.clone();
        let h_timer = h_timer.clone();
        let h_animating = h_animating.clone();
        let click_timer = click_timer.clone();

        let closure = Closure::wrap(Box::new(move || {
            let promise = window().navigator().clipboard().write_text("tsc.hk");

            let el = el_inner.clone();
            let h_timer = h_timer.clone();
            let h_animating = h_animating.clone();
            let click_timer = click_timer.clone();

            let on_write = Closure::wrap(Box::new(move |_val: JsValue| {
                clear_timer(&h_timer);
                clear_timer(&click_timer);
                *h_animating.borrow_mut() = true;

                el.set_text_content(Some(SHORT));

                let suffix: Vec<char> = SUFFIX.chars().collect();
                let suffix_len = suffix.len();
                let scrambled: Rc<RefCell<Vec<char>>> =
                    Rc::new(RefCell::new(vec![' '; suffix_len]));
                let pos = Rc::new(RefCell::new(0usize));
                let phase = Rc::new(RefCell::new(0u8));
                let el2 = el.clone();
                let h_timer2 = h_timer.clone();
                let h_animating2 = h_animating.clone();

                let suffix_closure = Closure::wrap(Box::new(move || {
                    let mut p = pos.borrow_mut();
                    let mut ph = phase.borrow_mut();
                    let mut sc = scrambled.borrow_mut();

                    if *ph == 0 {
                        sc[*p] = rand_glyph();
                        for j in (*p + 1)..suffix_len {
                            sc[j] = rand_glyph();
                        }
                        el2.set_text_content(Some(&format!(
                            "{}{}",
                            SHORT,
                            sc.iter().collect::<String>()
                        )));
                        *p += 1;
                        if *p >= suffix_len {
                            *ph = 1;
                            *p = 0;
                        }
                    } else {
                        sc[*p] = suffix[*p];
                        el2.set_text_content(Some(&format!(
                            "{}{}",
                            SHORT,
                            sc.iter().collect::<String>()
                        )));
                        *p += 1;
                        if *p >= suffix_len {
                            el2.set_text_content(Some(&format!("{}{}", SHORT, SUFFIX)));
                            let el3 = el2.clone();
                            let timer3 = h_timer2.clone();
                            let animating3 = h_animating2.clone();
                            let back_done = Closure::wrap(Box::new(move || {
                                *animating3.borrow_mut() = false;
                            }) as Box<dyn FnMut()>);
                            let t = animate_text(&el3, FULL, 20, Some(back_done));
                            *timer3.borrow_mut() = t.borrow_mut().take();
                        }
                    }
                }) as Box<dyn FnMut()>);

                let tid = set_interval(&suffix_closure, 25);
                suffix_closure.forget();
                *click_timer.borrow_mut() = Some(tid);
            }) as Box<dyn FnMut(JsValue)>);

            let _ = promise.then(&on_write);
            on_write.forget();
        }) as Box<dyn FnMut()>);

        el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }
}
