use actix_web::web::Data;
use rss::{Channel, Item};
use tokio::sync::Mutex;

use crate::rss::create_default_channel;

#[derive(Debug, Clone)]
pub struct AppState {
    pub rss_channels: Vec<Channel>,
}

impl AppState {
    pub fn add_item(&mut self, title: &str, item: Item) {
        let opt_channel = self.rss_channels.iter_mut().find(|x| x.title() == title);

        match opt_channel {
            Some(channel) => {
                channel.items.push(item);
            }
            None => {
                let mut channel = create_default_channel(title);
                channel.items.push(item);
                self.rss_channels.push(channel);
            }
        };
    }

    pub fn get_channel(&mut self, title: &str) -> Channel {
        let opt_channel = self.rss_channels.iter_mut().find(|x| x.title() == title);

        match opt_channel {
            Some(channel) => {
                let cloned = channel.clone();
                // empty items on purpose
                if !channel.items.is_empty() {
                    channel.items = vec![];
                }
                cloned
            }
            None => {
                let default_channel = create_default_channel(title);
                self.rss_channels.push(default_channel.clone());
                default_channel
            }
        }
    }

    pub fn create_state_mutex() -> Data<Mutex<AppState>> {
        Data::new(Mutex::new(AppState::default()))
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            rss_channels: Vec::new(),
        }
    }
}
