use std::ffi::CStr;

use pam::{pam_hooks, pam_try};
use pam::constants::{PAM_TEXT_INFO, PamFlag, PamResultCode};
use pam::constants::PamResultCode::{PAM_AUTH_ERR, PAM_IGNORE, PAM_SUCCESS};
use pam::conv::Conv;
use pam::module::{PamHandle, PamHooks};
use rand::random;

struct PamRandom;
pam_hooks!(PamRandom);

impl PamHooks for PamRandom {
    fn sm_authenticate(pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        let conv = match pamh.get_item::<Conv>() {
            Ok(Some(conv)) => conv,
            Ok(None) => todo!(),
            Err(err) => {
                println!("Couldn't get pam_conv");
                return err;
            }
        };
        if random::<bool>() {
            pam_try!(conv.send(PAM_TEXT_INFO, "Randomly succeeded"));
            PAM_SUCCESS
        } else {
            pam_try!(conv.send(PAM_TEXT_INFO, "Randomly failed"));
            PAM_AUTH_ERR
        }
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        PAM_SUCCESS
    }
}