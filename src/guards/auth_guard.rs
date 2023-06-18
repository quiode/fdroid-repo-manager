use actix_web::{ guard::{ Guard, GuardContext } };

use crate::utils::app_config::AppConfig;

pub struct AuthGuard;

impl Guard for AuthGuard {
    fn check(&self, req: &GuardContext) -> bool {
        let app_config = AppConfig::from_env();

        // check that password in header (RM-Password) equals password set in environment variables
        req.head()
            .headers()
            .get("RM-Password")
            .map(|password| {
                let opt_password = password.to_str();
                if opt_password.is_err() {
                    return false;
                }

                let str_password = opt_password.unwrap();

                str_password == app_config.admin_password.to_string()
            })
            .unwrap_or(false)
    }
}