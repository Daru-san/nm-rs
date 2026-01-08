use nm_rs::Client;
use nm_rs::prelude::DeviceExt;

fn print_devices(client: &Client) {
    let devices = client.devices();

    for device in devices {
        let name = device.iface();
        println!(
            "The network interface {name} is a {:#?} device.",
            device.device_type()
        );
    }
}

fn main() {
    let context = glib::MainContext::new();
    let mainloop = glib::MainLoop::new(Some(&context), false);

    context.spawn_local(glib::clone!(
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
            print_devices(&client);
            mainloop.quit();
        }
    ));

    mainloop.run();
}
