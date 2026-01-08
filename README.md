# libnm

NetworkManager bindings for Rust.

These expose the NetworkManager library features to Rust for usage in Rust-based
applications.

See <https://networkmanager.dev/docs/libnm/latest/> for documentation on how to
use libnm.

Docs are also available at <https://docs.rs/nm-rs>.

## Important notes

- This library is currently incomplete with features missing, if you encounter
  any issues with this library, please drop an issue.
- To use the asynchronous methods provided by NetworkManager, ensure that they
  are spawned in a glib
  [MainContent](https://gtk-rs.org/gtk-rs-core/stable/latest/docs/glib/struct.MainContext.html).

## Dependencies

Ensure the following libraries are installed before building this crate

- libnm, from network-manager
- glib

## Example usage

Below is an example that prints all the devices registered by NetworkManager.

```rust
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
```

More examples are available in the
[examples directory](https://github.com/Daru-san/nm-rs/tree/master/examples).

## Useful links

- <https://github.com/gtk-rs/gir>
- <https://networkmanager.dev/docs/libnm/latest/>
