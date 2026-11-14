//! Each boot's control daemon is told where the memory its VM shares with the host is, after
//! its handshake, so the guest kernel can map it: through the session's public surface over
//! the in-memory transport, with a fake launcher whose VM has a region.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use dsa_host_core::handshake::ControlHandshake;
use dsa_host_core::session::{GuestLauncher, Session, SessionConfig};
use dsa_host_core::video_slots::{DEFAULT_REGION_BYTES, REGION_NAME};
use dsa_protocol::auth::BootSecret;
use dsa_protocol::control::{self, envelope::Body, Role};
use dsa_protocol::handshake::{guest_connect, local_hello};
use dsa_transport::memory::{self, MemoryConnector};
use dsa_transport::{Connection, Connector};

const WAIT: Duration = Duration::from_secs(10);

/// A VM whose region of shared memory sits at `address`, as HCS reports it once the VM runs.
#[derive(Clone)]
struct SharingLauncher {
    secret: Arc<Mutex<Option<BootSecret>>>,
    address: u64,
}

impl GuestLauncher for SharingLauncher {
    fn start(&mut self, secret: &BootSecret) -> Result<(), String> {
        *self.secret.lock().unwrap() = Some(secret.clone());
        Ok(())
    }
    fn stop(&mut self, _timeout: Duration) -> Result<(), String> {
        *self.secret.lock().unwrap() = None;
        Ok(())
    }
    fn shared_memory(&self) -> Vec<control::GuestSharedMemory> {
        vec![control::GuestSharedMemory {
            name: REGION_NAME.into(),
            guest_physical_address: self.address,
            length: DEFAULT_REGION_BYTES,
        }]
    }
}

fn secret_of(launcher: &SharingLauncher) -> BootSecret {
    let deadline = Instant::now() + WAIT;
    loop {
        if let Some(secret) = launcher.secret.lock().unwrap().clone() {
            return secret;
        }
        assert!(Instant::now() < deadline, "the guest never started");
        thread::sleep(Duration::from_millis(5));
    }
}

fn session_with(launcher: SharingLauncher) -> (Session, MemoryConnector) {
    let handshake = ControlHandshake::new("dsa-host test", WAIT);
    let (listener, connector) = memory::pair();
    let config = SessionConfig {
        connect_timeout: WAIT,
        stop_timeout: WAIT,
        ..SessionConfig::default()
    };
    (
        Session::new(launcher, Box::new(listener), handshake, config),
        connector,
    )
}

/// The next GuestSharedMemory the host sends, skipping the other messages.
fn next_region(guest: &mut Box<dyn Connection>) -> control::GuestSharedMemory {
    let deadline = Instant::now() + WAIT;
    loop {
        assert!(Instant::now() < deadline, "the host never said where");
        let frame = guest.recv(Some(WAIT)).expect("a message for the guest");
        let body = dsa_protocol::decode(&frame)
            .expect("well formed")
            .body
            .expect("a body");
        if let Body::GuestSharedMemory(region) = body {
            return region;
        }
    }
}

#[test]
fn pr7_the_daemon_is_told_where_the_shared_picture_region_is_after_its_handshake() {
    let launcher = SharingLauncher {
        secret: Arc::default(),
        address: 0x1_0800_0000,
    };
    let (session, mut connector) = session_with(launcher.clone());
    let session = Arc::new(session);
    session.start();
    let secret = secret_of(&launcher);
    let mut guest = connector.connect(WAIT).expect("the daemon connects");
    guest_connect(
        guest.as_mut(),
        &local_hello(Role::Guest, "dsa-channeld test"),
        &secret,
        WAIT,
    )
    .expect("the host accepts the daemon");
    let region = next_region(&mut guest);
    assert_eq!(region.name, REGION_NAME);
    assert_eq!(region.guest_physical_address, 0x1_0800_0000);
    assert_eq!(region.length, DEFAULT_REGION_BYTES);
    session.stop();
}
