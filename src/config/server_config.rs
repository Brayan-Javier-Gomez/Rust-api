use dotenv::dotenv;
use std::env;
pub fn host_config() -> (String,u16) {
    dotenv().ok();
    let server_ip = env::var("SERVER_IP").expect("No se ha configurado una direccion ip");
    let server_port = env::var("SERVER_PORT").expect("No se ha configurado un puerto");
    let server_port: u16 = server_port.parse().expect("Numero de puerto invalido");
    return (server_ip,server_port);
}
