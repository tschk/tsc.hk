use std::cell::Cell;
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

fn window() -> web_sys::Window { web_sys::window().expect("no window") }
fn document() -> web_sys::Document { window().document().expect("document") }

fn rand_glyph() -> char {
    let r = js_sys::Math::random();
    GLYPHS[(r * GLYPHS.len() as f64) as usize % GLYPHS.len()]
}

fn clear_interval_id(id: i32) { window().clear_interval_with_handle(id); }

fn pad_chars(chars: &mut Vec<char>, len: usize) {
    while chars.len() < len {
        chars.push(' ');
    }
}

/// Animate text with a generation counter. Returns interval_id.
/// The done callback only fires if `gen` still matches `gen_cell` when the animation completes.
fn animate_text(
    el: &web_sys::Element,
    target: &str,
    speed: i32,
    gen_cell: Rc<Cell<u32>>,
    on_done: impl FnOnce() + 'static,
) -> i32 {
    let gen = gen_cell.get();
    let from = el.text_content().unwrap_or_default();
    let mut from_chars: Vec<char> = from.chars().collect();
    let mut target_chars: Vec<char> = target.chars().collect();
    let total = from_chars.len().max(target_chars.len());
    pad_chars(&mut from_chars, total);
    pad_chars(&mut target_chars, total);

    let chars: Rc<RefCell<Vec<char>>> = Rc::new(RefCell::new(from_chars));
    let phase: Rc<RefCell<u8>> = Rc::new(RefCell::new(0));
    let pos: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let el = el.clone();
    let target_str = target.to_string();
    let gen_cell2 = gen_cell.clone();

    let done_fn = Rc::new(Cell::new(Some(on_done)));

    let closure = Closure::wrap(Box::new(move || {
        if gen_cell2.get() != gen {
            return;
        }
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
                clear_timers(&el);
                if let Some(f) = done_fn.take() {
                    f();
                }
            }
        }
    }) as Box<dyn FnMut()>);

    let id = window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            speed,
        )
        .expect("set_interval");
    closure.forget();
    id
}

// ── State stored as data-* attributes on elements ─────────────────────────

fn get_timer_id(el: &web_sys::Element) -> Option<i32> {
    el.get_attribute("data-timer")
        .and_then(|v| v.parse::<i32>().ok())
}

fn set_timer_id(el: &web_sys::Element, id: i32) {
    el.set_attribute("data-timer", &id.to_string()).ok();
}

fn clear_timers(el: &web_sys::Element) {
    if let Some(id) = get_timer_id(el) {
        clear_interval_id(id);
        el.remove_attribute("data-timer").ok();
    }
}

// ── Init ──────────────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
pub fn start() {
    inject_styles();
    poll_bind(0, 200);
}

fn poll_bind(delay_ms: i32, remaining: u32) {
    let callback = Closure::<dyn FnMut()>::wrap(Box::new(move || {
        let doc = document();
        if doc.get_element_by_id("tsc-heading").is_some() {
            bind_heading(&doc);
            bind_links(&doc);
            watch_root();
        } else if remaining > 0 {
            poll_bind(100, remaining - 1);
        }
    }));
    let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(), delay_ms,
    );
    callback.forget();
}

fn inject_styles() {
    let doc = document();
    let style = doc.create_element("style").expect("create style");
    style.set_text_content(Some(
        "a, a:link, a:visited { color: inherit; text-decoration: none; }\n\
         #tsc-heading { cursor: pointer; font-variant-ligatures: none; }\n\
         .name { font-variant-ligatures: none; }",
    ));
    if let Some(head) = doc.head() {
        head.append_child(&style).ok();
    }
}

// ── Watch #crepus-root for hot-reload ─────────────────────────────────────

fn watch_root() {
    let doc = document();
    let root = match doc.get_element_by_id("crepus-root") {
        Some(r) => r,
        None => return,
    };

    let callback = Closure::wrap(Box::new(move |_: js_sys::Array, _: web_sys::MutationObserver| {
        let cb = Closure::<dyn FnMut()>::wrap(Box::new(move || {
            let doc = document();
            bind_heading(&doc);
            bind_links(&doc);
        }));
        let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(), 50,
        );
        cb.forget();
    }) as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);

    let opts = web_sys::MutationObserverInit::new();
    let opts_obj: &js_sys::Object = opts.unchecked_ref();
    js_sys::Reflect::set(opts_obj, &"childList".into(), &true.into()).ok();

    let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref())
        .expect("MutationObserver");
    observer.observe_with_options(&root, &opts).expect("observe");
    callback.forget();
    std::mem::forget(observer);
}

// ── Heading ───────────────────────────────────────────────────────────────

fn bind_heading(doc: &web_sys::Document) {
    let Some(el) = doc.get_element_by_id("tsc-heading") else { return };
    if el.get_attribute("data-bound").is_some() { return };
    el.set_attribute("data-bound", "1").ok();
    clear_timers(&el);
    bind_heading_events(el);
}

fn bind_heading_events(el: web_sys::Element) {
    let gen: Rc<Cell<u32>> = Rc::new(Cell::new(0));

    // mouseenter → scramble to SHORT
    {
        let el2 = el.clone();
        let gen2 = gen.clone();
        let closure = Closure::wrap(Box::new(move || {
            gen2.set(gen2.get().wrapping_add(1));
            clear_timers(&el2);
            el2.set_attribute("data-animating", "true").ok();
            let g = gen2.get();
            let el3 = el2.clone();
            let gen3 = gen2.clone();
            let id = animate_text(&el2, SHORT, 20, gen2.clone(), move || {
                if gen3.get() == g {
                    el3.remove_attribute("data-animating").ok();
                }
            });
            set_timer_id(&el2, id);
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("mouseenter", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    // mouseleave → scramble back to FULL
    {
        let el2 = el.clone();
        let gen2 = gen.clone();
        let closure = Closure::wrap(Box::new(move || {
            gen2.set(gen2.get().wrapping_add(1));
            clear_timers(&el2);
            el2.remove_attribute("data-animating").ok();
            let g = gen2.get();
            let el3 = el2.clone();
            let gen3 = gen2.clone();
            let id = animate_text(&el2, FULL, 20, gen2.clone(), move || {
                if gen3.get() == g {
                    el3.remove_attribute("data-animating").ok();
                }
            });
            set_timer_id(&el2, id);
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    // click → copy + suffix animate + back to FULL
    {
        let el2 = el.clone();
        let gen2 = gen.clone();
        let closure = Closure::wrap(Box::new(move || {
            gen2.set(gen2.get().wrapping_add(1));
            clear_timers(&el2);
            el2.set_attribute("data-animating", "true").ok();
            let promise = window().navigator().clipboard().write_text("tsc.hk");

            let el3 = el2.clone();
            let gen3 = gen2.clone();
            let on_write = Closure::wrap(Box::new(move |_: JsValue| {
                gen3.set(gen3.get().wrapping_add(1));
                clear_timers(&el3);
                el3.set_attribute("data-animating", "true").ok();
                el3.set_text_content(Some(SHORT));
                suffix_animate(&el3, gen3.clone());
            }) as Box<dyn FnMut(JsValue)>);

            let el4 = el2.clone();
            let gen4 = gen2.clone();
            let on_fail = Closure::wrap(Box::new(move |_: JsValue| {
                gen4.set(gen4.get().wrapping_add(1));
                clear_timers(&el4);
                el4.remove_attribute("data-animating").ok();
                el4.set_text_content(Some(FULL));
            }) as Box<dyn FnMut(JsValue)>);
            let _ = promise.then(&on_write).catch(&on_fail);
            on_write.forget();
            on_fail.forget();
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}

fn suffix_animate(el: &web_sys::Element, gen: Rc<Cell<u32>>) {
    let suffix: Vec<char> = SUFFIX.chars().collect();
    let len = suffix.len();
    let scrambled: Rc<RefCell<Vec<char>>> = Rc::new(RefCell::new(vec![' '; len]));
    let pos: Rc<RefCell<usize>> = Rc::new(RefCell::new(0));
    let phase: Rc<RefCell<u8>> = Rc::new(RefCell::new(0));
    let el2 = el.clone();
    let gen_val = gen.get();

    let closure = Closure::wrap(Box::new(move || {
        if gen.get() != gen_val {
            return;
        }
        let mut p = pos.borrow_mut();
        let mut ph = phase.borrow_mut();
        let mut sc = scrambled.borrow_mut();
        if *ph == 0 {
            sc[*p] = rand_glyph();
            for j in (*p + 1)..len {
                sc[j] = rand_glyph();
            }
            el2.set_text_content(Some(&format!("{}{}", SHORT, sc.iter().collect::<String>())));
            *p += 1;
            if *p >= len {
                *ph = 1;
                *p = 0;
            }
        } else {
            sc[*p] = suffix[*p];
            el2.set_text_content(Some(&format!("{}{}", SHORT, sc.iter().collect::<String>())));
            *p += 1;
            if *p >= len {
                el2.set_text_content(Some(&format!("{}{}", SHORT, SUFFIX)));
                clear_timers(&el2);
                gen.set(gen.get().wrapping_add(1));
                let g = gen.get();
                let el3 = el2.clone();
                let gen2 = gen.clone();
                let id = animate_text(&el2, FULL, 20, gen2.clone(), move || {
                    if gen2.get() == g {
                        el3.remove_attribute("data-animating").ok();
                    }
                });
                set_timer_id(&el2, id);
            }
        }
    }) as Box<dyn FnMut()>);

    let id = window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            25,
        )
        .expect("set_interval");
    closure.forget();
    set_timer_id(el, id);
}

// ── Links ─────────────────────────────────────────────────────────────────

fn bind_links(doc: &web_sys::Document) {
    let Some(body) = doc.body() else { return };
    let Ok(links) = body.query_selector_all("a.proj, a.con") else { return };
    for i in 0..links.length() {
        let Some(node) = links.item(i) else { continue };
        let Ok(link) = node.dyn_into::<web_sys::Element>() else { continue };
        if link.get_attribute("data-bound").is_some() { continue; }
        link.set_attribute("data-bound", "true").ok();
        bind_one_link(link);
    }
}

fn bind_one_link(link: web_sys::Element) {
    let gen: Rc<Cell<u32>> = Rc::new(Cell::new(0));

    // mouseenter → scramble .name text
    {
        let link2 = link.clone();
        let gen2 = gen.clone();
        let closure = Closure::wrap(Box::new(move || {
            gen2.set(gen2.get().wrapping_add(1));
            let name_el = match link2.query_selector(".name") {
                Ok(Some(n)) => n,
                _ => return,
            };
            clear_timers(&name_el);
            link2.set_attribute("data-animating", "true").ok();
            let original = name_el.text_content().unwrap_or_default();
            name_el.set_attribute("data-original", &original).ok();
            let link3 = link2.clone();
            let g = gen2.get();
            let gen3 = gen2.clone();
            let id = animate_text(&name_el, &original, 18, gen2.clone(), move || {
                if gen3.get() == g {
                    link3.set_attribute("style", "text-decoration-line: underline").ok();
                    link3.remove_attribute("data-animating").ok();
                }
            });
            set_timer_id(&name_el, id);
        }) as Box<dyn FnMut()>);
        link.add_event_listener_with_callback("mouseenter", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    // mouseleave → reset immediately, no animation
    {
        let link2 = link.clone();
        let gen2 = gen.clone();
        let closure = Closure::wrap(Box::new(move || {
            gen2.set(gen2.get().wrapping_add(1));
            link2.remove_attribute("data-animating").ok();
            link2.remove_attribute("style").ok();
            if let Ok(Some(name_el)) = link2.query_selector(".name") {
                clear_timers(&name_el);
                if let Some(orig) = name_el.get_attribute("data-original") {
                    name_el.set_text_content(Some(&orig));
                }
            }
        }) as Box<dyn FnMut()>);
        link.add_event_listener_with_callback("mouseleave", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}