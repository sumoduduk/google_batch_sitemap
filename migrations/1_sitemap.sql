CREATE TABLE sitemap (
    number INTEGER,
    url_sitemap TEXT,
    path_json TEXT,
    keywords TEXT,
    time TEXT DEFAULT (strftime('%s', 'now') + 2*24*60*60) -- 2 days in Unix time
);

CREATE TABLE chain_sitemap (
    id INTEGER,
    sitemaps TEXT
);

CREATE TABLE dark_mode (
    dark_mode TEXT DEFAULT 'light'
);
