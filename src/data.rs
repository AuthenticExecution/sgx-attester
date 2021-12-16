use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub sp_privkey : String,
    pub ias_cert : String,
    pub enclave_settings : String,
    pub enclave_sig : String,
    pub enclave_host : String,
    pub enclave_port : u16,
    pub aesm_host : String,
    pub aesm_port : u16
}