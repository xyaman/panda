use crate::runtime;
use std::{
    collections::HashMap,
    default::Default,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures::lock::Mutex;

use isahc::{http::Response, Body};

#[derive(Default)]
pub(crate) struct RateLimit {
    // Key needs to be {major_parameter}:{channel_id/guild_id}
    // Example:
    // - "channels:639562328521703445"
    // - "channels/messages:639562328521703445"
    buckets: Arc<Mutex<HashMap<String, Bucket>>>,
}

#[derive(Default)]
struct Bucket {
    pub limit: u32,
    pub remaining: u32,
    pub reset: u64,
}

impl RateLimit {
    pub(crate) async fn check_and_sleep(&self, bucket_key: &str) {
        // Get bucket from key
        let mut buckets_hm = self.buckets.lock().await;
        let bucket = buckets_hm.get_mut(bucket_key);

        // If it exists, check
        // If not exists, we assume that it's safe to make the api call
        if let Some(b) = bucket {
            // Get current time
            let current = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            // Sleep reset time
            let difference = Duration::from_secs(b.reset.saturating_sub(current));

            if difference.as_secs() == 0 {
                // We assume that we have waited the reset time
                // but the reset value have not been updated
                b.remaining += 1;
            } else if b.remaining == 0 {
                runtime::sleep(difference).await;

                // Add the default remaining
                b.remaining = b.limit;
            }
            // If passes all then it's safe to make the api call

            b.remaining -= 1;
        }
    }

    pub(crate) async fn update(&self, bucket_key: String, response: &Response<Body>) {
        let headers = response.headers();
        let mut buckets_hm = self.buckets.lock().await;

        // Get the bucket and update all entries
        let bucket = buckets_hm.entry(bucket_key).or_default();

        if let Some(limit) = headers.get("x-ratelimit-limit") {
            bucket.limit = limit.to_str().unwrap().parse::<u32>().unwrap();
        }

        if let Some(remaining) = headers.get("x-ratelimit-remaining") {
            bucket.remaining = remaining.to_str().unwrap().parse::<u32>().unwrap();
        }

        if let Some(reset) = headers.get("x-ratelimit-reset") {
            bucket.reset = reset.to_str().unwrap().parse::<u64>().unwrap();
        }
    }
}
