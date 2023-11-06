#![allow(unused)]

fn main() {
    let host;
    #[cfg(target_os = "windows")]
    {
        host = cpal::host_from_id(cpal::HostId::Asio).expect("failed to initialise ASIO host");
    }
}
