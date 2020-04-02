fn main() -> Result<(), std::io::Error> {
    let _sentry = sentry::init((
        "https://public@example.com/42",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));
    sentry::integrations::panic::register_panic_handler();

    let mut app = tide::new();
    app.at("/").get(|_| async move {
        panic!("this is a panic");
        ""
    });

    async_std::task::block_on(app.listen("127.0.0.1:8080"))?;

    Ok(())
}



