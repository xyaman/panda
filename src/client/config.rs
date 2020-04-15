/// Config contains all customizable options of the Client
pub struct Config {
    pub(crate) gateway_large_treshold: u8,
    pub(crate) gateway_guilds_subscriptions: bool,
    pub(crate) gateway_shard_id: u64,
    pub(crate) gateway_num_shards: u64,
}

impl Config {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    pub(crate) fn new_default() -> Config {
        Config {
            gateway_large_treshold: 50,
            gateway_guilds_subscriptions: true,
            gateway_shard_id: 0,
            gateway_num_shards: 1,
        }
    }
}

pub struct ConfigBuilder {
    pub(crate) gateway_large_treshold: u8,
    pub(crate) gateway_guilds_subscriptions: bool,
    pub(crate) gateway_shard_id: u64,
    pub(crate) gateway_num_shards: u64,
}

impl ConfigBuilder {
    fn new() -> ConfigBuilder {
        ConfigBuilder {
            gateway_large_treshold: 50,
            gateway_guilds_subscriptions: true,
            gateway_shard_id: 0,
            gateway_num_shards: 1,
        }
    }

    /// Set largue threshold, It's value between 50 and 250, it's total number of members where the gateway
    ///  will stop sending offline members in the guild member list. Default is 50.
    pub fn set_large_threshold(mut self, threshold: u8) -> Self {
        self.gateway_large_treshold = threshold;

        self
    }

    /// A true value enables dispatching of guild subscription events (presence and typing events). Default true.
    pub fn set_guild_subscriptions(mut self, subscriptions: bool) -> Self {
        self.gateway_guilds_subscriptions = subscriptions;

        self
    }

    /// Set shard for [Guild Sharding](https://discordapp.com/developers/docs/topics/gateway#sharding).
    /// Default [0, 1]
    pub fn set_shards(mut self, shard_id: u64, num_shards: u64) -> Self {
        self.gateway_shard_id = shard_id;
        self.gateway_num_shards = num_shards;

        self
    }

    /// Build a Config struct
    pub fn build(self) -> Config {
        Config {
            gateway_large_treshold: self.gateway_large_treshold,
            gateway_guilds_subscriptions: self.gateway_guilds_subscriptions,
            gateway_shard_id: self.gateway_shard_id,
            gateway_num_shards: self.gateway_num_shards,
        }
    }
}
