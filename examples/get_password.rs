use nm_rs::Client;
use nm_rs::prelude::ConnectionExt;

fn get_password(client: &Client) {
    let connections = client.connections();
    let Some(remote_connection) = connections.first() else {
        return;
    };
    println!(
        "Password for connection {} is {}",
        remote_connection.id(),
        remote_connection.setting_wireless_security().psk()
    )
}

fn main() {
    let mainloop = glib::MainLoop::new(None, false);
    mainloop.context().spawn_local(glib::clone!(
        #[strong]
        mainloop,
        async move {
            let client = match Client::new_future().await {
                Ok(client) => client,
                Err(e) => {
                    glib::g_error!("app", "Failed to create new client: {e}");
                    return;
                }
            };
            get_password(&client);
            mainloop.quit();
        }
    ));
    mainloop.run();
}
