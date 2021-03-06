use super::proto;
use super::server::{PublisherNotFoundError, PublisherStore};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type HashMapPublisherStore = Arc<RwLock<HashMap<String, proto::Registration>>>;

impl PublisherStore for HashMapPublisherStore {
    fn insert_publisher(&self, publisher_name: &str, registration: proto::Registration) {
        self.write()
            .unwrap()
            .insert(publisher_name.to_string(), registration);
    }

    fn remove_publisher(&self, publisher_name: &str) -> Result<(), PublisherNotFoundError> {
        let mut locked_publishers = self.write().unwrap();
        let has_key = locked_publishers.contains_key(publisher_name);

        if !has_key {
            error!("Publisher not found")
        }

        locked_publishers.remove(publisher_name);
        Ok(())
    }

    fn get_publishers(&self) -> Vec<(String, proto::Registration)> {
        let locked_publishers = self.write().unwrap();
        let mut publisher_pairs = vec![];
        for publisher_name_pair in locked_publishers.clone() {
            publisher_pairs.push(publisher_name_pair);
        }

        publisher_pairs
    }

    fn find_publisher(&self, publisher_name: &str) -> Option<proto::Registration> {
        let locked_map = self.read().unwrap();
        let registration = locked_map.get(publisher_name).unwrap().clone();

        Some(registration)
    }

    fn find_publishers(&self, search_str: &str) -> Vec<(String, proto::Registration)> {
        let locked_map = self.read().unwrap();
        let mut new_vec = vec![];
        for pair in locked_map.clone() {
            if (*pair.0).contains(search_str) {
                new_vec.push(pair);
            }
        }

        new_vec
    }
}

#[test]
fn get_all_test() {
    let vec_store = HashMapPublisherStore::new(RwLock::new(HashMap::new()));
    let empty_registration = proto::Registration {
        publisher: None,
        info: None,
    };

    vec_store.insert_publisher(&"test_name".to_string(), empty_registration);

    let publisher_pairs = vec_store.find_publishers(&"test_name".to_string());
    assert_eq!(publisher_pairs.len(), 1);
    assert_eq!(publisher_pairs.get(0).unwrap().0, "test_name".to_string());
}

#[test]
fn get_single_by_name_test() {
    let vec_store = HashMapPublisherStore::new(RwLock::new(HashMap::new()));
    let empty_registration = proto::Registration {
        publisher: None,
        info: None,
    };

    vec_store.insert_publisher(&"test_name".to_string(), empty_registration.clone());

    let publisher_registration = vec_store.find_publisher(&"test_name".to_string());
    assert!(publisher_registration.is_some());
    assert_eq!(publisher_registration.unwrap(), empty_registration);
}

#[test]
fn remove_single_publisher_by_name_test() {
    let vec_store = HashMapPublisherStore::new(RwLock::new(HashMap::new()));
    let empty_registration = proto::Registration {
        publisher: None,
        info: None,
    };

    vec_store.insert_publisher(&"test_name".to_string(), empty_registration);

    let mut publisher_pairs = vec_store.find_publishers(&"test_name".to_string());
    assert_eq!(publisher_pairs.len(), 1);

    let remove_result = vec_store.remove_publisher(&"test_name".to_string());
    assert!(remove_result.is_ok(), "Expected ok removal");

    publisher_pairs = vec_store.get_publishers();
    assert_eq!(
        publisher_pairs.len(),
        0,
        "Incorrect number of publishers returned"
    );
}

#[test]
fn remove_multiple_publishers_by_name_test() {
    let vec_store = HashMapPublisherStore::new(RwLock::new(HashMap::new()));
    let empty_registration = proto::Registration {
        publisher: None,
        info: None,
    };

    vec_store.insert_publisher(&"test_name_1".to_string(), empty_registration.clone());
    vec_store.insert_publisher(&"test_name_2".to_string(), empty_registration.clone());
    vec_store.insert_publisher(&"test_name_3".to_string(), empty_registration.clone());

    let mut publisher_pairs = vec_store.get_publishers();
    assert_eq!(publisher_pairs.len(), 3);

    let names_to_remove = vec![
        "test_name_1".to_string(),
        "test_name_2".to_string(),
        "test_name_3".to_string(),
    ];

    vec_store.remove_publishers(&names_to_remove);
    publisher_pairs = vec_store.get_publishers();
    assert_eq!(
        publisher_pairs.len(),
        0,
        "Incorrect number of publishers returned"
    );
}
