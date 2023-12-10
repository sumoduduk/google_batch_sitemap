mod method;

use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

#[cfg(not(target_os = "windows"))]
fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/easy_index/database.sqlite"
}

#[cfg(target_os = "windows")]
fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/Documents/easy_index/database.sqlite"
}

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

pub async fn run_pool() -> sqlx::Result<SqlitePool> {
    let db_file = get_db_path();

    let pool = SqlitePool::connect(&db_file).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> sqlx::Result<()> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}

pub fn init_db() {
    if !db_file_exists() {
        create_db_file();
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use method::*;

    #[tokio::test]
    async fn test_connection() -> sqlx::Result<()> {
        init_db();

        let pool = run_pool().await?;

        run_migrations(&pool).await?;

        let sitemap = Sitemap::new(
            1,
            "https://example.com".to_string(),
            "/path/to/json".to_string(),
            "keyword1,keyword2".to_string(),
            "2 days".to_string(),
        );

        let _ = create_sitemap(&pool, &sitemap).await?;

        let data1 = get_sitemaps(&pool).await?;
        dbg!(&data1);

        assert_eq!(&data1[0].path_json, "/path/to/json");

        Ok(())
    }

    #[tokio::test]
    async fn it_will_check_urlsitemap() -> sqlx::Result<()> {
        init_db();

        let pool = run_pool().await?;

        run_migrations(&pool).await?;

        let arr = get_sitemaps(&pool).await?;
        dbg!(&arr);

        assert_eq!(&arr[0].keywords, "keyword1,keyword2");
        Ok(())
    }
}

