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
    static HEADING_STATE: RefCell<Option<HeadingState>> = const { RefCell::new(None) };
    static LINK_STATE: RefCell<HashMap<usize, LinkAnim>> = RefCell::new(HashMap::new());
    static HEADING_HOVERED: RefCell<bool> = const { RefCell::new(false) };
}

fn link_key(el: &web_sys::Element) -> usize {
    let val: &JsValue = el.as_ref();
    val as *const JsValue as usize
}

// ── Init ──────────────────────────────────────────────────────────────────

#[wasm_bindgen(start)]
pub fn start() {
    inject_styles();
    init_heading();
    init_links();
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

// ── Heading ───────────────────────────────────────────────────────────────

fn init_heading() {
    HEADING_STATE.with(|state| {
        *state.borrow_mut() = Some(HeadingState {
            timer: Rc::new(RefCell::new(None)),
            click_timer: Rc::new(RefCell::new(None)),
            animating: Rc::new(RefCell::new(false)),
        });
    });

    let doc = document();

    // mouseover (bubbles) — enter #tsc-heading
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let Some(heading) = find_target(&event, "#tsc-heading") else { return };
            // Only trigger on actual enter (not moving between children)
            let already_hovered = HEADING_HOVERED.with(|h| *h.borrow());
            if already_hovered {
                return;
            }
            HEADING_HOVERED.with(|h| *h.borrow_mut() = true);

            HEADING_STATE.with(|state| {
                let s = state.borrow();
                let s = match s.as_ref() {
                    Some(s) => s,
                    None => return,
                };
                if *s.animating.borrow() {
                    return;
                }
                *s.animating.borrow_mut() = true;
                clear_timer(&s.timer);
                let anim = s.animating.clone();
                let done_cb = Closure::wrap(Box::new(move || {
                    *anim.borrow_mut() = false;
                }) as Box<dyn FnMut()>);
                let t = animate_text(&heading, SHORT, 20, Some(done_cb));
                *s.timer.borrow_mut() = t.borrow_mut().take();
            });
        }) as Box<dyn FnMut(web_sys::Event)>);
        doc.add_event_listener_with_callback("mouseover", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // mouseout (bubbles) — leave #tsc-heading
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            // Check if we actually left the heading (not just moving to a child)
            let related = event
                .unchecked_ref::<web_sys::MouseEvent>()
                .related_target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
            let Some(heading) = find_target(&event, "#tsc-heading") else { return };
            // If related_target is still inside heading, we didn't leave
            if let Some(ref related) = related {
                if heading.contains(Some(related.as_ref())) {
                    return;
                }
            }
            HEADING_HOVERED.with(|h| *h.borrow_mut() = false);

            HEADING_STATE.with(|state| {
                let s = state.borrow();
                let s = match s.as_ref() {
                    Some(s) => s,
                    None => return,
                };
                clear_timer(&s.timer);
                let anim = s.animating.clone();
                let done_cb = Closure::wrap(Box::new(move || {
                    *anim.borrow_mut() = false;
                }) as Box<dyn FnMut()>);
                let t = animate_text(&heading, FULL, 20, Some(done_cb));
                *s.timer.borrow_mut() = t.borrow_mut().take();
            });
        }) as Box<dyn FnMut(web_sys::Event)>);
        doc.add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // click
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let Some(heading) = find_target(&event, "#tsc-heading") else { return };
            let promise = window().navigator().clipboard().write_text("tsc.hk");

            HEADING_STATE.with(|state| {
                let s = state.borrow();
                if let Some(s) = s.as_ref() {
                    clear_timer(&s.timer);
                    clear_timer(&s.click_timer);
                    *s.animating.borrow_mut() = true;
                }
            });

            let on_write = Closure::wrap(Box::new(move |_val: JsValue| {
                heading.set_text_content(Some(SHORT));

                let suffix: Vec<char> = SUFFIX.chars().collect();
                let suffix_len = suffix.len();
                let scrambled: Rc<RefCell<Vec<char>>> =
                    Rc::new(RefCell::new(vec![' '; suffix_len]));
                let pos = Rc::new(RefCell::new(0usize));
                let phase = Rc::new(RefCell::new(0u8));
                let el2 = heading.clone();

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
                            let back_done = Closure::wrap(Box::new(move || {
                                HEADING_STATE.with(|s| {
                                    if let Some(ref st) = *s.borrow() {
                                        *st.animating.borrow_mut() = false;
                                    }
                                });
                            }) as Box<dyn FnMut()>);
                            let t = animate_text(&el3, FULL, 20, Some(back_done));
                            HEADING_STATE.with(|s| {
                                if let Some(ref st) = *s.borrow() {
                                    *st.timer.borrow_mut() = t.borrow_mut().take();
                                }
                            });
                        }
                    }
                }) as Box<dyn FnMut()>);

                let tid = set_interval_ms(&suffix_closure, 25);
                suffix_closure.forget();
                HEADING_STATE.with(|s| {
                    if let Some(ref st) = *s.borrow() {
                        *st.click_timer.borrow_mut() = Some(tid);
                    }
                });
            }) as Box<dyn FnMut(JsValue)>);

            let _ = promise.then(&on_write);
            on_write.forget();
        }) as Box<dyn FnMut(web_sys::Event)>);
        doc.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }
}

// ── Links ─────────────────────────────────────────────────────────────────

fn init_links() {
    let doc = document();

    // mouseover on a.proj / a.con → scramble .name then underline
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let Some(link) = find_target(&event, "a.proj, a.con") else { return };
            let name_el = match link.query_selector(".name") {
                Ok(Some(n)) => n,
                _ => return,
            };
            let key = link_key(&link);
            LINK_STATE.with(|state| {
                if state.borrow().contains_key(&key) {
                    return;
                }
                let original = name_el.text_content().unwrap_or_default();
                let link_clone = link.clone();
                let done_cb = Closure::wrap(Box::new(move || {
                    link_clone
                        .set_attribute("style", "text-decoration-line: underline")
                        .ok();
                    LINK_STATE.with(|s| {
                        s.borrow_mut().remove(&key);
                    });
                }) as Box<dyn FnMut()>);
                let timer = animate_text(&name_el, &original, 18, Some(done_cb));
                state.borrow_mut().insert(key, LinkAnim { timer, original });
            });
        }) as Box<dyn FnMut(web_sys::Event)>);
        doc.add_event_listener_with_callback("mouseover", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }

    // mouseout on a.proj / a.con → reset immediately
    {
        let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
            let related = event
                .unchecked_ref::<web_sys::MouseEvent>()
                .related_target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok());
            let Some(link) = find_target(&event, "a.proj, a.con") else { return };
            // If related_target is still inside the link, we didn't leave
            if let Some(ref related) = related {
                if link.contains(Some(related.as_ref())) {
                    return;
                }
            }
            let name_el = match link.query_selector(".name") {
                Ok(Some(n)) => n,
                _ => return,
            };
            let key = link_key(&link);
            LINK_STATE.with(|state| {
                if let Some(anim) = state.borrow_mut().remove(&key) {
                    clear_timer(&anim.timer);
                    name_el.set_text_content(Some(&anim.original));
                }
            });
            link.remove_attribute("style").ok();
        }) as Box<dyn FnMut(web_sys::Event)>);
        doc.add_event_listener_with_callback("mouseout", closure.as_ref().unchecked_ref())
            .ok();
        closure.forget();
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn find_target(event: &web_sys::Event, selector: &str) -> Option<web_sys::Element> {
    let target = event.target()?.unchecked_into::<web_sys::Element>();
    match target.closest(selector) {
        Ok(Some(el)) => Some(el),
        _ => None,
    }
}
