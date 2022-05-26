use web_sys::Storage;

fn local_storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

pub fn get_item(key: &str) -> Option<String> {
    local_storage().get_item(key).unwrap()
}

pub fn set_item(key: &str, value: &str) {
    local_storage().set_item(key, value).unwrap()
}

#[allow(dead_code)]
pub fn remove_item(key: &str) {
    local_storage().remove_item(key).unwrap()
}

#[allow(dead_code)]
pub fn clear() {
    local_storage().clear().unwrap()
}
