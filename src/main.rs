use wayland_client::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Connection::connect_to_env().unwrap();
    println!("connected to wayland client!");
    Ok(())
}
