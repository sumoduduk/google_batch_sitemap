mod oauth;
mod utils;

use std::env::args;

#[tokio::main]
async fn main() -> Result<(), String> {
    let oauth = oauth::get_token("./secret.json").await.unwrap();
    let token = oauth.token().unwrap();
    let arg = args().collect::<Vec<String>>();

    let sitemap_url = arg.get(1).expect("must provide sitemap_url in argument 1");

    let uri_contain_xml = sitemap_url
        .contains(".xml")
        .then_some(sitemap_url)
        .ok_or_else(|| "URL must contains xml".to_string())?;

    let _ = utils::start_indexing(uri_contain_xml, &token).await;

    Ok(())
}
