use crate::errors::ProtocolError;
use crate::errors::ProtocolError::NetParams;

// AddressParser parser node block to model block

pub struct Address {
    script: String,
    address: String,
}

impl Address {
    pub fn script(&mut self) -> &str {
        self.script.as_ref()
    }

    pub fn address(&self) -> &str {
        self.address.as_ref()
    }
}
struct AddressParser {
    net_params: String,
}

#[allow(dead_code)]
impl AddressParser {
    // new_address_parser new object
    fn new_address_parser(net_params: String) -> AddressParser {
        AddressParser { net_params }
    }

    fn parse(&self, cp: &str) -> Result<Address, ProtocolError> {
        let cps = cp;
        if !is_p2wpkh_script(cps) && !is_p2wsh_script(cps) {
            return Err(ProtocolError::NetParams("script invalid".to_string()));
        }

        let address = script_to_address(cps, self.net_params.clone())?;
        let script = hex::encode(cps);
        Ok(Address {
            script: script.clone(),
            address,
        })
    }
}

pub fn script_to_address(_cps: &str, net_str: String) -> Result<String, ProtocolError> {
    if net_str.is_empty() {
        return Result::Err(NetParams("not find the network".to_string()));
    }

    Ok("".to_string())
}

pub fn is_p2wpkh_script(_cp: &str) -> bool {
    false
}

pub fn is_p2wsh_script(_cp: &str) -> bool {
    false
}
