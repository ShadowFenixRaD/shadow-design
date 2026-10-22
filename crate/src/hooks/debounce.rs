use dioxus::prelude::*;
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
    let mut version = use_signal(|| 0u64);

    use_effect(move || {
        let val = source.read().clone();
        let curr_ver = *version.read() + 1;
        version.set(curr_ver);

        spawn(async move {
            sleep_ms(delay_ms).await;
            if *version.read() == curr_ver {
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
    let mut is_cooling_down = use_signal(|| false);

    use_effect(move || {
        let val = source.read().clone();
        if !*is_cooling_down.read() {
            throttled.set(val);
            is_cooling_down.set(true);
            spawn(async move {
                sleep_ms(interval_ms).await;
                is_cooling_down.set(false);
            });
        }
    });

    throttled
}
