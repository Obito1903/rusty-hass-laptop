# RHL - Rusty Hass Laptop

A rust app to send stats about your laptop to Home-assistant

## Build

```sh
cargo build
```

### Install

```sh
cargo install --path .
```

## Run

```sh
cargo run
```

Or if installed :

```sh
rusty-hass-laptop
```

On the first run RHL, a `config.json` file wile be generated in `~/.config/rusty-hass-laptop`.
Modify it to match your hass installation.

### Config reference

|        Field         |        Default        | Desc                                                               |
| :------------------: | :-------------------: | ------------------------------------------------------------------ |
|     `auth_token`     |        `null`         | Your long live token to authorize the app to send info to hass     |
|     `webhook_id`     |    auto generated     | Id of the webhook created by RHL on you Hass instance              |
|    `hass_address`    |        `null`         | Url to your Home-assistant instance                                |
|     `device_id`      |   `<Hostname>_<OS>`   | Device id of your laptop in Home-assistant                         |
|       `app_id`       |  `rusty-hass-laptop`  | Id of the app in Home-assistant                                    |
|      `app_name`      |  `Rusty Hass Laptop`  | Name of the app                                                    |
|    `app_version`     |    auto generated     | App version                                                        |
|    `device_name`     |     `<Hostname>`      | Name of your device in Home-assistant (Laptop Hostname by default) |
|    `manufacturer`    |       `Unknown`       | Manufacturer of your device                                        |
|       `model`        |       `Unknown`       | Model of the laptop                                                |
|      `os_name`       |        `<OS>`         | Code name of your operating system                                 |
|     `os_version`     | `<OS_kernel_version>` | Kernel version of your OS                                          |
| `support_encryption` |        `false`        | Enable or disable encrypted communication with Hass                |

You at least need to set the `hass_address` and the `auth_token` fields.

then you can restart the app, and it will set up the webhook and start sending data to Home-assistant.

### Run as a user service

You can also run the app as a user service. to do that just copy the `rusty-hass-laptop.service` file in `~/.config/systemd/user/`.

Then you can enable and start it.

```sh
systemctl enable --user rusty-hass-laptop.service
systemctl start --user rusty-hass-laptop.service
```
