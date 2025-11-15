use inhibitor::InhibitorState;
use std::error::Error;
use tokio::signal::unix::{signal, SignalKind};
use zbus::interface;
use zbus::object_server::SignalEmitter;

mod inhibitor;

struct AppState {
    inhibitor_state: Option<InhibitorState>,
}

impl AppState {
    fn start_inhibitor(&mut self) {
        let mut state = InhibitorState::default();
        state.setup();
        self.inhibitor_state = Some(state);
    }
}

#[interface(name = "st.contraptioni.IdleInhibitor1")]
impl AppState {
    async fn enable_inhibitor(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> bool {
        if self.inhibitor_state.is_some() {
            println!("Inhibitor already active");
            false
        } else {
            self.start_inhibitor();
            let _ = self.is_inhibitor_active_changed(&emitter).await;
            true
        }
    }

    async fn disable_inhibitor(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> bool {
        if self.inhibitor_state.is_none() {
            println!("Inhibitor not active");
            false
        } else {
            self.inhibitor_state = None;
            let _ = self.is_inhibitor_active_changed(&emitter).await;
            true
        }
    }

    async fn toggle_inhibitor(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> bool {
        if self.inhibitor_state.is_some() {
            self.inhibitor_state = None;
            let _ = self.is_inhibitor_active_changed(&emitter).await;
            false
        } else {
            self.start_inhibitor();
            let _ = self.is_inhibitor_active_changed(&emitter).await;
            true
        }
    }

    #[zbus(property)]
    async fn is_inhibitor_active(&self) -> bool {
        self.inhibitor_state.is_some()
    }
}

impl Drop for AppState {
    fn drop(&mut self) {
        self.inhibitor_state = None;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Idle Inhibitor Service starting...");

    let dbus_path = "/st/contraptioni/IdleInhibitor";

    let _connection = zbus::connection::Builder::session()?
        .name("st.contraptioni.IdleInhibitor")?
        .serve_at(
            dbus_path,
            AppState {
                inhibitor_state: None,
            },
        )?
        .build()
        .await?;

    println!("Listening on {}", dbus_path);

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    tokio::select! {
        _ = sigterm.recv() => {println!("SIGTERM Shuting down");},
        _ = sigint.recv() => {println!("SIGINT Shuting down");},
    };

    Ok(())
}
