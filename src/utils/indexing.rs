use google_indexing_api::{GoogleIndexingApi, UrlNotificationsType};

pub async fn begin(uris: Vec<String>, token: &str) {
    let ress = GoogleIndexingApi::url_notifications()
        .batch(token, uris, UrlNotificationsType::UPDATED)
        .await;

    match ress {
        Ok(result) => println!("{:#?}", result),
        Err(err) => println!("error  : {:#?}", err),
    }
}

