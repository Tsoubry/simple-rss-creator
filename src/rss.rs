use lazy_static::lazy_static;
use regex::Regex;
use rss::{Channel, Guid, Item};
use uuid::Uuid;

lazy_static! {
    static ref URL_REGEX: Regex =
        Regex::new(r"(?i)(https?://)?([a-zA-Z0-9-]+(\.[a-zA-Z0-9-]+)+)(/[^\s]*)?").unwrap();
}

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

    let link = find_link(&description);

    let guid = Some(Guid {
        value: uuid,
        permalink: false,
    });

    let current_datetime = chrono::Local::now().to_rfc2822();

    let mut item = Item::default();
    item.set_title(Some(derived_title));
    item.set_link(link);
    item.set_pub_date(current_datetime);
    item.set_description(Some(description));
    item.set_guid(guid);

    item
}

fn find_link(description: &str) -> Option<String> {
    URL_REGEX
        .captures(description)
        .map(|cap| cap.get(0).map(|m| m.as_str().to_string()))
        .flatten()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_link() {

        let description = "This is a test description with a link: https://www.example.com";

        let link = find_link(description);

        assert_eq!(link, Some("https://www.example.com".to_string()));

    }

    #[test]
    fn test_find_link_2() {
        let description = "This is a test description without a link";

        let link = find_link(description);

        assert_eq!(link, None);
    
    }

    #[test]
    fn test_find_link_3() {
        
        // multiple links, only return the first one
        let description = "This is a test description with multiple links: example.com/hello and also https://www.google.com";

        let link = find_link(description);

        assert_eq!(link, Some("example.com/hello".to_string()));
    
    }

}