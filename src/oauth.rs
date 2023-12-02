use yup_oauth2::{read_service_account_key, AccessToken, ServiceAccountAuthenticator};

pub async fn get_token(path: &str) -> eyre::Result<AccessToken> {
    let secret = read_service_account_key(path)
        .await
        .expect("no json secret found");
    let auth = ServiceAccountAuthenticator::builder(secret).build().await?;
    let scopes = &["https://www.googleapis.com/auth/indexing"];

    let token = auth.token(scopes).await?;

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use google_indexing_api::GoogleIndexingApi;
    use std::env::var;

    #[tokio::test]
    async fn should_get_metadata() {
        dotenv().ok();

        let token = get_token("./secret.json").await.unwrap();
        let token = token.token().unwrap();
        let uri = var("URI_TEST").expect("URI_TEST not found on env");
        println!("token : {token}");
        println!("uri : {uri}");

        let metadata = GoogleIndexingApi::url_notifications()
            .get_metadata(&token, &uri)
            .await;

        match metadata {
            Ok(res) => {
                println!(" metadata : {:#?}", res);
            }
            Err(err) => {
                dbg!(err);
            }
        }
    }
}

