use crate::error::{WolgramError, WolgramResult};
use wakey::WolPacket;

pub fn perform_wol(mac: &str) -> WolgramResult {
    let wol = WolPacket::from_string(mac, ':');
    if let Err(err) = wol.send_magic() {
        Err(WolgramError::WakeyError(err))
    } else {
        Ok(())
    }
}
