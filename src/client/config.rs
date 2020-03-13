/// Config contains all customizable options of the Client
pub struct Config {
    pub(crate) gateway_large_treshold: u8,
    pub(crate) gateway_guilds_subscriptions: bool,
    pub(crate) gateway_shard_id: u64,
    pub(crate) gateway_num_shards: u64,
}

impl Config {
    pub fn new_default() -> Config {
        Config {
            gateway_large_treshold: 50,
            gateway_guilds_subscriptions: true,
            gateway_shard_id: 0,
            gateway_num_shards: 1,
        }
    }
}
