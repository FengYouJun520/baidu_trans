#[cfg(feature = "blocking")]
#[test]
fn common_translate_blocking() -> anyhow::Result<()> {
    use baidu_trans::blocking::Client;
    use baidu_trans::config::Config;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    let resp = client.translate(
        "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
    )?;

    if resp.error_code.is_some() {
        return Err(anyhow::Error::msg(resp.error_msg.unwrap()));
    }

    let res: Vec<String> = resp
        .trans_result
        .unwrap()
        .iter()
        .map(|t| t.dst.clone())
        .collect();
    let res = res.join("");
    println!("{}", res);
    Ok(())
}

#[cfg(all(feature = "blocking", feature = "image"))]
#[test]
fn image_translate_blocking() -> anyhow::Result<()> {
    use baidu_trans::blocking::Client;
    use baidu_trans::config::Config;
    use baidu_trans::lang::Lang;
    use std::fs;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));
    client.lang(Lang::Auto, Lang::Zh);

    let data = fs::read("tests/a.png")?;
    let resp = client.image_translate("a.png", data)?;
    assert_eq!(resp.error_code, "0");
    Ok(())
}

#[cfg(feature = "aio")]
#[tokio::test]
async fn common_translate_aio() -> anyhow::Result<()> {
    use baidu_trans::aio::Client;
    use baidu_trans::config::Config;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    let resp = client
        .translate(
            "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
        )
        .await?;

    assert_eq!(resp.error_code, None);
    Ok(())
}

#[cfg(all(feature = "aio", feature = "image"))]
#[tokio::test]
async fn image_translate_aio() -> anyhow::Result<()> {
    use baidu_trans::aio::Client;
    use baidu_trans::config::Config;
    use baidu_trans::lang::Lang;
    use tokio::fs;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));
    client.lang(Lang::Auto, Lang::Zh);

    let data = fs::read("tests/a.png").await?;
    let resp = client.image_translate("a.png", data).await?;
    assert_eq!(resp.error_code, "0");
    Ok(())
}

#[cfg(all(feature = "blocking", feature = "domain"))]
#[test]
fn domain_translate_blocking() -> anyhow::Result<()> {
    use baidu_trans::blocking::Client;
    use baidu_trans::config::Config;
    use baidu_trans::domain::Domain;
    use baidu_trans::lang::Lang;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    client.lang(Lang::Auto, Lang::Zh);

    let resp = client.domain_translate(
        "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
        Domain::Electronics,
    )?;

    assert_eq!(resp.error_code, None);
    dbg!(resp);
    Ok(())
}

#[cfg(all(feature = "aio", feature = "domain"))]
#[tokio::test]
async fn domain_translate_aio() -> anyhow::Result<()> {
    use baidu_trans::aio::Client;
    use baidu_trans::config::Config;
    use baidu_trans::domain::Domain;
    use baidu_trans::lang::Lang;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    client.lang(Lang::Auto, Lang::Zh);

    let resp = client
        .domain_translate(
            "As we discussed in Chapter 1, Hello Rust!, stack variables are preferred thanks to
their low overhead and speed compared to heap-allocated data, which
automatically introduces overhead thanks to the necessary heap pointer.",
            Domain::Electronics,
        )
        .await?;

    assert_eq!(resp.error_code, None);
    dbg!(resp);
    Ok(())
}

#[cfg(all(feature = "blocking", feature = "doc"))]
#[test]
fn doc_translate_blocking() -> anyhow::Result<()> {
    use baidu_trans::blocking::Client;
    use baidu_trans::config::Config;
    use baidu_trans::lang::Lang;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    client.lang(Lang::Auto, Lang::Zh);

    let resp = client.doc_count_translate("tests/a.txt")?;
    assert_eq!(resp.error_code, 52000);
    dbg!(resp);
    Ok(())
}

#[cfg(all(feature = "aio", feature = "doc"))]
#[tokio::test]
async fn doc_translate_aio() -> anyhow::Result<()> {
    use baidu_trans::aio::Client;
    use baidu_trans::config::Config;
    use baidu_trans::lang::Lang;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));

    client.lang(Lang::Auto, Lang::Zh);

    let resp = client.doc_count_translate("tests/a.txt").await?;
    dbg!(resp);
    let resp = client.doc_count_translate("tests/b.txt").await?;
    dbg!(resp);

    Ok(())
}
