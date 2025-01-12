use bytes::Bytes;
use crossbeam;
use governor::clock::FakeRelativeClock;
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use std::{sync::Arc, time::Duration};

fn main() {
    let clock = FakeRelativeClock::default();
    let quota = Quota::per_second(nonzero!(2500u32));
    let lim = Arc::new(RateLimiter::hashmap_with_clock(quota, clock));
    let ms = Duration::from_millis(1);
    let key = Bytes::from("conflict_key");

    crossbeam::scope(|scope| {
        for _ in 0..10 {
            let key = key.clone();
            let lim = lim.clone();
            scope.spawn(move |_| {
                for _ in 0..250 {
                    let ret = lim.check_key(&key).expect("Failed to acquire key");
                    lim.clock().advance(100 * ms);
                }
            });
        }
    })
    .unwrap();
}
