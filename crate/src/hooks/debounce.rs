use dioxus::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use crate::util::sleep_ms;

/// Returns a debounced clone of `source` signal that only updates after `delay_ms` of inactivity.
#[must_use]
pub fn use_debounced_signal<T: Clone + PartialEq + 'static>(
    source: Signal<T>,
    delay_ms: u64,
) -> Signal<T> {
    let initial = source.read().clone();
    let mut debounced = use_signal(|| initial);
    let version = use_hook(|| Arc::new(AtomicU64::new(0)));

    let ver_clone = Arc::clone(&version);
    use_effect(move || {
        let val = source.read().clone();
        let curr_ver = ver_clone.fetch_add(1, Ordering::Relaxed) + 1;
        let ver_check = Arc::clone(&ver_clone);

        spawn(async move {
            sleep_ms(delay_ms).await;
            if ver_check.load(Ordering::Relaxed) == curr_ver {
                debounced.set(val);
            }
        });
    });

    debounced
}

/// Returns a throttled clone of `source` signal that updates at most once every `interval_ms`.
/// Includes trailing-edge execution so the final value emitted during cooldown is never lost.
#[must_use]
pub fn use_throttle<T: Clone + PartialEq + 'static>(
    source: Signal<T>,
    interval_ms: u64,
) -> Signal<T> {
    let initial = source.read().clone();
    let mut throttled = use_signal(|| initial);
    let is_cooling_down = use_hook(|| Arc::new(AtomicBool::new(false)));
    let trailing_value = use_hook(|| Arc::new(Mutex::new(None::<T>)));

    let cooldown_clone = Arc::clone(&is_cooling_down);
    let trailing_clone = Arc::clone(&trailing_value);
    use_effect(move || {
        let val = source.read().clone();
        if !cooldown_clone.swap(true, Ordering::Relaxed) {
            throttled.set(val);
            let cd = Arc::clone(&cooldown_clone);
            let tr = Arc::clone(&trailing_clone);
            spawn(async move {
                loop {
                    sleep_ms(interval_ms).await;
                    let next = tr.lock().ok().and_then(|mut g| g.take());
                    if let Some(v) = next {
                        throttled.set(v);
                    } else {
                        cd.store(false, Ordering::Relaxed);
                        break;
                    }
                }
            });
        } else if let Ok(mut g) = trailing_clone.lock() {
            *g = Some(val);
        }
    });

    throttled
}
