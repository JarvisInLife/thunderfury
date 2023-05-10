use actix_web::web;

pub mod tv;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(tv::list_tv).service(tv::new_tv);
}
