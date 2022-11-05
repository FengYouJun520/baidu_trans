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
automatically introduces overhead thanks to the necessary heap pointer. For
stack variables, Rust's types even allow for zero overhead structures, so no
additional metadata is stored. The following snippet asserts that there are no
additional bytes being used for arrays or user-defined types.",
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
    use std::fs;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));
    client.lang("auto", "zh");

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
automatically introduces overhead thanks to the necessary heap pointer. For
stack variables, Rust's types even allow for zero overhead structures, so no
additional metadata is stored. The following snippet asserts that there are no
additional bytes being used for arrays or user-defined types.",
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
    use tokio::fs;

    dotenv::dotenv()?;
    let app_id = dotenv::var("APP_ID")?;
    let app_secret = dotenv::var("APP_SECRET")?;

    let client = Client::new(Config::new(app_id, app_secret));
    client.lang("auto", "zh");

    let data = fs::read("tests/a.png").await?;
    let resp = client.image_translate("a.png", data).await?;
    assert_eq!(resp.error_code, "0");
    Ok(())
}
