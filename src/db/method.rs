use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
pub struct Sitemap {
    pub number: i32,
    pub url_sitemap: String,
    pub path_json: String,
    pub keywords: String,
    pub time: String,
}

pub enum UpdateSitemap {
    URL,
    PATHJSON,
    KEYWORDS,
    TIME,
}

impl Sitemap {
    pub fn new(
        number: i32,
        url_sitemap: String,
        path_json: String,
        keywords: String,
        time: String,
    ) -> Self {
        Sitemap {
            number,
            url_sitemap,
            path_json,
            keywords,
            time,
        }
    }

    pub async fn delete(&self, pool: &SqlitePool) -> sqlx::Result<i32> {
        let num = self.number;
        delete_sitemap(pool, num).await?;
        Ok(num)
    }

    pub fn update(&mut self, update_name: UpdateSitemap, value: String) -> &Self {
        match update_name {
            UpdateSitemap::URL => self.url_sitemap = value,
            UpdateSitemap::PATHJSON => self.path_json = value,
            UpdateSitemap::KEYWORDS => self.keywords = value,
            UpdateSitemap::TIME => self.time = value,
        }

        self
    }

    pub async fn run_update(&self, pool: &SqlitePool) -> sqlx::Result<String> {
        update_sitemap(pool, self).await?;

        Ok("sitemap updated success".to_owned())
    }
}

pub async fn create_sitemap(pool: &SqlitePool, sitemap: &Sitemap) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO sitemap (number, url_sitemap, path_json, keywords, time)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(sitemap.number)
    .bind(&sitemap.url_sitemap)
    .bind(&sitemap.path_json)
    .bind(&sitemap.keywords)
    .bind(&sitemap.time)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_sitemaps(pool: &SqlitePool) -> Result<Vec<Sitemap>, sqlx::Error> {
    let row: Vec<Sitemap> = sqlx::query_as(
        r#"
        SELECT *
        FROM sitemap
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(row)
}

async fn update_sitemap(pool: &SqlitePool, sitemap: &Sitemap) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sitemap
        SET url_sitemap = ?, path_json = ?, keywords = ?, time = ?
        WHERE number = ?
        "#,
    )
    .bind(&sitemap.url_sitemap)
    .bind(&sitemap.path_json)
    .bind(&sitemap.keywords)
    .bind(&sitemap.time)
    .bind(sitemap.number)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_sitemap(pool: &SqlitePool, number: i32) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM sitemap
        WHERE number = ?
        "#,
    )
    .bind(number)
    .execute(pool)
    .await?;

    Ok(())
}

