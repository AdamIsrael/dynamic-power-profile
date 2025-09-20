extern crate upower_dbus;

use futures::stream::StreamExt;
use upower_dbus::UPowerProxy;
use zbus::zvariant::Value;

use zbus::{Connection, Result, dbus_proxy};

#[dbus_proxy(
    interface = "net.hadess.PowerProfiles",
    default_service = "net.hadess.PowerProfiles",
    default_path = "/net/hadess/PowerProfiles"
)]
trait PowerProfiles {
    /// ActiveProfile property
    #[dbus_proxy(property)]
    fn active_profile(&self) -> Result<String>;

    /// Profiles property
    #[dbus_proxy(property)]
    fn profiles(&self) -> Result<Vec<zbus::zvariant::OwnedValue>>;
}

/// Checks if the system has a "performance" power profile.
async fn has_performance_profile(connection: &Connection) -> Result<bool> {
    let proxy = PowerProfilesProxy::new(connection).await?;

    let profiles = proxy.profiles().await?;
    let mut found_profile: bool = false;

    profiles.iter().for_each(|profile| {
        if profile.to_string().contains("performance") {
            found_profile = true;
            return;
        }
    });

    Ok(found_profile)
}

async fn set_power_profile(connection: &Connection, on_battery: bool) -> zbus::Result<()> {
    // Set the default power profile to balanced mode
    let mut profile = "balanced";

    if !on_battery {
        // Set power profile to performance mode, if available
        if has_performance_profile(connection).await? {
            profile = "performance";
        }
    } else {
        // We're on battery, so conserve power
        profile = "power-saver";
    }

    println!("Setting power profile to {}", profile);
    let _profile_call = connection
        .call_method(
            Some("net.hadess.PowerProfiles"),
            "/net/hadess/PowerProfiles",
            Some("org.freedesktop.DBus.Properties"),
            "Set",
            &(
                "net.hadess.PowerProfiles",
                "ActiveProfile",
                Value::new(profile),
            ),
        )
        .await?;

    Ok(())
}

fn main() -> zbus::Result<()> {
    futures::executor::block_on(async move {
        let connection = zbus::Connection::system().await?;

        if let Ok(upower) = UPowerProxy::new(&connection).await {
            let on_battery = upower.on_battery().await?;

            // The power profile can be set via dbus
            // gdbus call --system --dest net.hadess.PowerProfiles --object-path /net/hadess/PowerProfiles --method org.freedesktop.DBus.Properties.Set 'net.hadess.PowerProfiles' 'ActiveProfile' "Performance"

            set_power_profile(&connection, on_battery).await?;
            let mut stream = upower.receive_on_battery_changed().await;

            while let Some(event) = stream.next().await {
                let on_battery = event.get().await?;
                set_power_profile(&connection, on_battery).await?;
            }
        }

        Ok(())
    })
}
