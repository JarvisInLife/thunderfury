use sea_orm::EntityTrait;

use crate::{common::AppState, entity::subscription, utils::filename_parser::EpisodeInfo};

pub async fn do_subscription(state: &AppState) {
    let subs: Vec<subscription::Model> = subscription::Entity::find().all(&state.db).await.unwrap();
    for sub in &subs {
        match sub.resource_provider.as_str() {
            "mikan" => run_mikan(state, sub).await,
            "alist" => run_alist(state, sub).await,
            &_ => {
                println!("{} is not supported", sub.resource_provider);
            }
        }
    }
}

async fn run_mikan(state: &AppState, sub: &subscription::Model) {
    let content = reqwest::get(sub.resource_url.as_str())
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

async fn run_alist(state: &AppState, sub: &subscription::Model) {
    let files = state.alist.list(&sub.resource_url).await.unwrap();
    for f in files.as_slice() {
        if f.is_dir {
            continue;
        }

        let info = EpisodeInfo::from(f.name.as_str());
        if !info.is_valid() {
            continue;
        }

        println!(
            "matched, file: {}, season {:?}, episode {:?}",
            f.name, info.season_number, info.episode_number
        );

        state.alist.download(&f.path, &f.name).await.unwrap();
    }
}
