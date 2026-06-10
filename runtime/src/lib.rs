use std::cell::RefCell;
use std::collections::HashMap;
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

fn document() -> web_sys::Document {
    window().document().expect("document")
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

fn set_interval_ms(closure: &Closure<dyn FnMut()>, ms: i32) -> i32 {
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            ms,
        )
        .expect("set_interval")
}

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
    while from_padded.len() < max_len { from_padded.push(' '); }
    while target_padded.len() < max_len { target_padded.push(' '); }

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
            for j in (*p + 1)..total { c[j] = rand_glyph(); }
            el.set_text_content(Some(&c.iter().collect::<String>()));
            *p += 1;
            if *p >= total { *ph = 1; *p = 0; }
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
    let id = set_interval_ms(&closure, speed);
    *timer.borrow_mut() = Some(id);
    closure.forget();
    timer
}

// ── State ─────────────────────────────────────────────────────────────────

struct HeadingState {
    timer: TimerHandle,
    click_timer: TimerHandle,
    animating: Rc<RefCell<bool>>,
}

struct LinkAnim {
    timer: TimerHandle,
    original: String,
}

thread_local! {
    static HEADING_STATE: RefCell<Option<HeadingState>> = RefCell::new(None);
    static LINK_STATE: RefCell<HashMap<usize, LinkAnim>> = RefCell::new(HashMap::new());
    static BOUND_HEADING_KEY: RefCell<Option<usize>> = RefCell::new(None);
    static BOUND_LINK_KEYS: RefCell<HashMap<usize, bool>> = RefCell::new(HashMap::new());
}

fn element_key(el: &web_sys::Element) -> usize {
    let val: &JsValue = el.as_ref();
    val as *const JsValue as usize
}

// ── Init ──────────────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
pub fn start() {
    inject_styles();
    poll_bind(0, 200);
}

fn poll_bind(delay_ms: i32, remaining: u32) {
    let callback = Closure::<dyn FnMut()>::wrap(Box::new(move || {
        let doc = window().document().expect("document");
        if doc.get_element_by_id("tsc-heading").is_some() {
            rebind_all();
            watch_for_renders();
        } else if remaining > 0 {
            poll_bind(100, remaining - 1);
        }
    }));
    let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        delay_ms,
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

// ── MutationObserver — rebind after hot-reload ────────────────────────────

fn watch_for_renders() {
    let doc = document();
    let body = match doc.body() {
        Some(b) => b,
        None => return,
    };

    let callback = Closure::wrap(Box::new(move |_mutations: js_sys::Array, _observer: web_sys::MutationObserver| {
        // Debounce: schedule a rebind after mutations settle
        let cb = Closure::<dyn FnMut()>::wrap(Box::new(move || {
            rebind_all();
        }));
        let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            50,
        );
        cb.forget();
    }) as Box<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>);

    let opts = web_sys::MutationObserverInit::new();
    // We need childList and subtree
    // Unfortunately web_sys::MutationObserverInit doesn't have all setters,
    // so we use the raw js_sys approach
    let opts_obj: &js_sys::Object = opts.unchecked_ref();
    js_sys::Reflect::set(opts_obj, &"childList".into(), &true.into()).ok();
    js_sys::Reflect::set(opts_obj, &"subtree".into(), &true.into()).ok();

    let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()).expect("MutationObserver");
    observer.observe_with_options(&body, &opts).expect("observe");
    callback.forget();
    // observer is kept alive because the callback closure captures a reference
    // Actually we need to forget the observer too... but that leaks it.
    // For now, let the observer be leaked (it's a singleton).
    std::mem::forget(observer);
}

fn rebind_all() {
    let doc = document();

    // Heading
    if let Some(heading) = doc.get_element_by_id("tsc-heading") {
        let key = element_key(&heading);
        let already_bound = BOUND_HEADING_KEY.with(|k| *k.borrow() == Some(key));
        if !already_bound {
            clear_heading_timers();
            bind_heading(heading);
            BOUND_HEADING_KEY.with(|k| *k.borrow_mut() = Some(key));
        }
    }

    // Links — clear old bindings, rebind all
    BOUND_LINK_KEYS.with(|k| k.borrow_mut().clear());
    clear_link_timers();
    if let Some(body) = doc.body() {
        if let Ok(links) = body.query_selector_all("a.proj, a.con") {
            for i in 0..links.length() {
                if let Some(node) = links.item(i) {
                    if let Ok(link) = node.dyn_into::<web_sys::Element>() {
                        let key = element_key(&link);
                        BOUND_LINK_KEYS.with(|k| { k.borrow_mut().insert(key, true); });
                        bind_one_link(link);
                    }
                }
            }
        }
    }
}

fn clear_heading_timers() {
    HEADING_STATE.with(|s| {
        if let Some(ref h) = *s.borrow() {
            clear_timer(&h.timer);
            clear_timer(&h.click_timer);
        }
    });
}

fn clear_link_timers() {
    LINK_STATE.with(|s| {
        for (_, anim) in s.borrow_mut().drain() {
            clear_timer(&anim.timer);
        }
    });
}

// ── Heading binding ───────────────────────────────────────────────────────

fn bind_heading(el: web_sys::Element) {
    HEADING_STATE.with(|state| {
        *state.borrow_mut() = Some(HeadingState {
            timer: Rc::new(RefCell::new(None)),
            click_timer: Rc::new(RefCell::new(None)),
            animating: Rc::new(RefCell::new(false)),
        });
    });

    // mouseover
    {
        let el2 = el.clone();
        let closure = Closure::wrap(Box::new(move || {
            HEADING_STATE.with(|s| {
                let s = s.borrow();
                let s = match s.as_ref() {
                    Some(s) => s,
                    None => return,
                };
                if *s.animating.borrow() { return; }
                *s.animating.borrow_mut() = true;
                clear_timer(&s.timer);
                let anim = s.animating.clone();
                let done = Closure::wrap(Box::new(move || {
                    *anim.borrow_mut() = false;
                }) as Box<dyn FnMut()>);
                let t = animate_text(&el2, SHORT, 20, Some(done));
                *s.timer.borrow_mut() = t.borrow_mut().take();
            });
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("mouseover", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    // mouseout
    {
        let el2 = el.clone();
        let closure = Closure::wrap(Box::new(move || {
            HEADING_STATE.with(|s| {
                let s = s.borrow();
                let s = match s.as_ref() {
                    Some(s) => s,
                    None => return,
                };
                clear_timer(&s.timer);
                let anim = s.animating.clone();
                let done = Closure::wrap(Box::new(move || {
                    *anim.borrow_mut() = false;
                }) as Box<dyn FnMut()>);
                let t = animate_text(&el2, FULL, 20, Some(done));
                *s.timer.borrow_mut() = t.borrow_mut().take();
            });
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    // click
    {
        let el_clone = el.clone();
        let closure = Closure::wrap(Box::new(move || {
            let el2 = el_clone.clone();
            let promise = window().navigator().clipboard().write_text("tsc.hk");
            HEADING_STATE.with(|s| {
                if let Some(ref h) = *s.borrow() {
                    clear_timer(&h.timer);
                    clear_timer(&h.click_timer);
                    *h.animating.borrow_mut() = true;
                }
            });
            let on_write = Closure::wrap(Box::new(move |_val: JsValue| {
                el2.set_text_content(Some(SHORT));
                let suffix: Vec<char> = SUFFIX.chars().collect();
                let suffix_len = suffix.len();
                let scrambled: Rc<RefCell<Vec<char>>> = Rc::new(RefCell::new(vec![' '; suffix_len]));
                let pos = Rc::new(RefCell::new(0usize));
                let phase = Rc::new(RefCell::new(0u8));
                let el3 = el2.clone();
                let suffix_closure = Closure::wrap(Box::new(move || {
                    let mut p = pos.borrow_mut();
                    let mut ph = phase.borrow_mut();
                    let mut sc = scrambled.borrow_mut();
                    if *ph == 0 {
                        sc[*p] = rand_glyph();
                        for j in (*p + 1)..suffix_len { sc[j] = rand_glyph(); }
                        el3.set_text_content(Some(&format!("{}{}", SHORT, sc.iter().collect::<String>())));
                        *p += 1;
                        if *p >= suffix_len { *ph = 1; *p = 0; }
                    } else {
                        sc[*p] = suffix[*p];
                        el3.set_text_content(Some(&format!("{}{}", SHORT, sc.iter().collect::<String>())));
                        *p += 1;
                        if *p >= suffix_len {
                            el3.set_text_content(Some(&format!("{}{}", SHORT, SUFFIX)));
                            let el4 = el3.clone();
                            let back_done = Closure::wrap(Box::new(move || {
                                HEADING_STATE.with(|s| {
                                    if let Some(ref h) = *s.borrow() { *h.animating.borrow_mut() = false; }
                                });
                            }) as Box<dyn FnMut()>);
                            let t = animate_text(&el4, FULL, 20, Some(back_done));
                            HEADING_STATE.with(|s| {
                                if let Some(ref h) = *s.borrow() { *h.timer.borrow_mut() = t.borrow_mut().take(); }
                            });
                        }
                    }
                }) as Box<dyn FnMut()>);
                let tid = set_interval_ms(&suffix_closure, 25);
                suffix_closure.forget();
                HEADING_STATE.with(|s| {
                    if let Some(ref h) = *s.borrow() { *h.click_timer.borrow_mut() = Some(tid); }
                });
            }) as Box<dyn FnMut(JsValue)>);
            let _ = promise.then(&on_write);
            on_write.forget();
        }) as Box<dyn FnMut()>);
        el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}

// ── Link binding ──────────────────────────────────────────────────────────

fn bind_one_link(link: web_sys::Element) {
    // mouseover
    {
        let link2 = link.clone();
        let closure = Closure::wrap(Box::new(move || {
            let key = element_key(&link2);
            let already = LINK_STATE.with(|s| s.borrow().contains_key(&key));
            if already { return; }
            let name_el = match link2.query_selector(".name") {
                Ok(Some(n)) => n,
                _ => return,
            };
            let original = name_el.text_content().unwrap_or_default();
            let link3 = link2.clone();
            let done = Closure::wrap(Box::new(move || {
                link3.set_attribute("style", "text-decoration-line: underline").ok();
                LINK_STATE.with(|s| { s.borrow_mut().remove(&element_key(&link3)); });
            }) as Box<dyn FnMut()>);
            let timer = animate_text(&name_el, &original, 18, Some(done));
            LINK_STATE.with(|s| { s.borrow_mut().insert(key, LinkAnim { timer, original }); });
        }) as Box<dyn FnMut()>);
        link.add_event_listener_with_callback("mouseover", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }

    // mouseout
    {
        let link2 = link.clone();
        let closure = Closure::wrap(Box::new(move || {
            let key = element_key(&link2);
            let name_el = match link2.query_selector(".name") {
                Ok(Some(n)) => n,
                _ => return,
            };
            LINK_STATE.with(|s| {
                if let Some(anim) = s.borrow_mut().remove(&key) {
                    clear_timer(&anim.timer);
                    name_el.set_text_content(Some(&anim.original));
                }
            });
            link2.remove_attribute("style").ok();
        }) as Box<dyn FnMut()>);
        link.add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    }
}
