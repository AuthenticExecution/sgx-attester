use std::fs;
use std::env;
use std::io::Result;

mod data;
use data::Data;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <input_yaml>", args[0]);
        panic!("Wrong arguments");
    }

    let file_data = read_from_file(&args[1]).unwrap();
    let config : Data = serde_yaml::from_slice(&file_data).expect("Failed to read input file");

    let mut sp_privkey = read_from_file(&config.sp_privkey)
        .expect("Failed to read SP private key file");
    sp_privkey.push(0); // fix bug with PEM keys

    let mut ias_cert = read_from_file(&config.ias_cert)
        .expect("Failed to read IAS root certificate file");
    ias_cert.push(0); // fix bug with PEM keys

    let enclave_settings = read_from_file(&config.enclave_settings)
        .expect("Failed to read enclave settings file");

    let enclave_sig = read_from_file(&config.enclave_sig)
        .expect("Failed to read enclave signature file");

    let key = sgx_attestation::attest_enclave(
        &config.enclave_host, 
        config.enclave_port, 
        &config.aesm_host,
        config.aesm_port, 
        &sp_privkey, 
        &ias_cert, 
        &enclave_settings, 
        &enclave_sig).unwrap();

    println!("{:?}", key);
}

fn read_from_file(file : &str) -> Result<Vec<u8>> {
    let res = fs::read(file)?;
    Ok(res)
}
