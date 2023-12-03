mod oauth;
mod utils;

use std::env::args;

#[tokio::main]
async fn main() -> Result<(), String> {
    let arg = args().collect::<Vec<String>>();

    let sitemap_url = arg
        .get(1)
        .expect("Error : must provide sitemap_url in argument 1");

    let path_json = arg
        .get(2)
        .expect("Error : must proovide google secret json in argument 2 ");
    let oauth = oauth::get_token(&path_json).await.unwrap();
    let token = oauth.token().unwrap();

    let uri_contain_xml = sitemap_url
        .contains(".xml")
        .then_some(sitemap_url)
        .ok_or_else(|| "URL must contains xml".to_string())?;

    let _ = utils::start_indexing(uri_contain_xml, &token).await;

    Ok(())
}
