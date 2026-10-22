use dioxus::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

async fn sleep_ms(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }
}

/// Returns a debounced clone of `source` signal that only updates after `delay_ms` of inactivity.
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
pub fn use_throttle<T: Clone + PartialEq + 'static>(
    source: Signal<T>,
    interval_ms: u64,
) -> Signal<T> {
    let initial = source.read().clone();
    let mut throttled = use_signal(|| initial);
    let is_cooling_down = use_hook(|| Arc::new(AtomicBool::new(false)));

    let cooldown_clone = Arc::clone(&is_cooling_down);
    use_effect(move || {
        let val = source.read().clone();
        if !cooldown_clone.swap(true, Ordering::Relaxed) {
            throttled.set(val);
            let cd = Arc::clone(&cooldown_clone);
            spawn(async move {
                sleep_ms(interval_ms).await;
                cd.store(false, Ordering::Relaxed);
            });
        }
    });

    throttled
}
