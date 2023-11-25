use rss::{Channel, Guid, Item};
use uuid::Uuid;

pub fn create_channel(title: &str, items: Vec<Item>) -> Channel {
    let mut channel = Channel::default();

    channel.set_title(title);
    channel.set_items(items);

    channel
}

pub fn create_default_channel(title: &str) -> Channel {
    create_channel(title, vec![])
}

pub fn get_item(title: Option<String>, description: String) -> Item {
    let uuid: String = Uuid::new_v4().into();

    let derived_title = title.unwrap_or(description.lines().next().unwrap_or(&uuid).to_string());

    let guid = Some(Guid {
        value: uuid,
        permalink: false,
    });

    let current_datetime = chrono::Local::now().to_rfc2822();

    let mut item = Item::default();
    item.set_title(Some(derived_title));
    item.set_pub_date(current_datetime);
    item.set_description(Some(description));
    item.set_guid(guid);

    item
}
