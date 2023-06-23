use sea_orm::EntityTrait;

use crate::{common::AppState, entity::subscription, utils::filename_parser::EpisodeInfo};

pub async fn do_subscription(state: &AppState) {
    let subs: Vec<subscription::Model> = subscription::Entity::find().all(&state.db).await.unwrap();
    for sub in &subs {
        run(&state, sub).await;
    }
}

async fn run(state: &AppState, sub: &subscription::Model) {
    let content = reqwest::get(sub.rss_url.as_str())
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let channel = rss::Channel::read_from(&content[..]).unwrap();
    println!("{}", channel.title());
    for item in channel.items() {
        let info = EpisodeInfo::from(item.title().unwrap());
        if info.is_valid() && info.resolution == Some("1080p".to_string()) {
            println!(
                "matched, file: {}, season {:?}, episode {:?}, subtitles {:?}",
                item.title().unwrap(),
                info.season_number,
                info.episode_number,
                info.subtitles
            );
        }
    }
}
