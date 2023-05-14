use actix_web::{ guard::{ Guard, GuardContext } };

use crate::utils::app_config::AppConfig;

pub struct AuthGuard;

impl Guard for AuthGuard {
    fn check(&self, req: &GuardContext) -> bool {
        let app_config = AppConfig::from_env();

        req.head()
            .headers()
            .get("RM-Password")
            .map(|password| {
                let opt_password = password.to_str();
                if opt_password.is_err() {
                    return false;
                }

                let str_password = opt_password.unwrap();

                str_password == app_config.admin_password
            })
            .unwrap_or(false)
    }
}