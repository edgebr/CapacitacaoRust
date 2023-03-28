mod wifi_sync {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread::sleep;
    use std::time::Duration;
    use wifi::WifiSync;

    mod wifi {
        use std::thread;
        use std::thread::sleep;
        use std::time::Duration;

        pub struct WifiSync {
            ssid: String,
            password: String,
        }

        impl WifiSync {
            pub fn new(ssid: &str, password: &str) -> Self {
                Self {
                    ssid: ssid.to_owned(),
                    password: password.to_owned(),
                }
            }

            pub fn start(&self, conn_callback: fn(bool)) {
                thread::spawn(move || {
                    sim_wifi_connection(conn_callback);
                });
            }
        }

        fn sim_wifi_connection(conn_callback: fn(bool)) {
            loop {
                sleep(Duration::from_secs(2));
                conn_callback(true);
                sleep(Duration::from_secs(5));
                conn_callback(false);
            }
        }
    }

    pub fn main() {
        let wifi_sync = WifiSync::new("network", "123456");
        wifi_sync.start(wifi_status_change);
        while !IS_CONNETED.load(Ordering::SeqCst) {
            sleep(Duration::from_millis(100));
        }
        println!("Connected!");
    }

    fn wifi_status_change(is_connected: bool) {
        IS_CONNETED.store(is_connected, Ordering::SeqCst);
    }

    static IS_CONNETED: AtomicBool = AtomicBool::new(false);
}

mod wifi_async {
    use wifi::WifiAsync;

    mod wifi {
        use std::sync::Arc;
        use tokio::sync::Notify;
        use tokio::time::sleep;
        use tokio::time::Duration;

        pub struct WifiAsync {
            ssid: String,
            password: String,
            conn_notify: Arc<Notify>,
            disconn_notify: Arc<Notify>,
        }

        impl WifiAsync {
            pub fn new(ssid: &str, password: &str) -> Self {
                Self {
                    ssid: ssid.to_owned(),
                    password: password.to_owned(),
                    conn_notify: Arc::new(Notify::new()),
                    disconn_notify: Arc::new(Notify::new()),
                }
            }

            pub fn start(&self) -> (Arc<Notify>, Arc<Notify>) {
                let conn_notify = self.conn_notify.clone();
                let disconn_notify = self.disconn_notify.clone();

                tokio::spawn(async move {
                    sleep(Duration::from_secs(2)).await;
                    conn_notify.notify_one();
                    sleep(Duration::from_secs(5)).await;
                    disconn_notify.notify_one();
                });

                (self.conn_notify.clone(), self.disconn_notify.clone())
            }
        }
    }

    #[tokio::main]
    pub async fn main() {
        let wifi_async = WifiAsync::new("network", "123456");
        let (conn, disconn) = wifi_async.start();
        conn.notified().await;
        println!("Connected!");
        disconn.notified().await;
        println!("Disconnected!");
    }
}

fn main() {
    // wifi_sync::main();
    wifi_async::main();
}
