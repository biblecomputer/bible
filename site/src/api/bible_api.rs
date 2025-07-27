use crate::core::{Bible, BIBLE, init_bible_signal};
use leptos::prelude::Set;
use gloo_net::http::Request;
use rexie::{ObjectStore, Rexie, TransactionMode};

pub async fn init_bible() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if BIBLE.get().is_some() {
        return Ok(());
    }

    let bible = load_or_fetch_bible().await?;

    BIBLE
        .set(bible.clone())
        .map_err(|_| "Failed to set Bible data")?;
    let bible_signal = init_bible_signal();
    bible_signal.set(Some(bible));

    Ok(())
}

async fn load_or_fetch_bible() -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    use crate::storage::{get_selected_translation, is_translation_downloaded, load_downloaded_translation};
    
    if let Some(selected_translation) = get_selected_translation() {
        if is_translation_downloaded(&selected_translation) {
            if let Ok(bible) = load_downloaded_translation(&selected_translation).await {
                return Ok(bible);
            }
        }
    }

    match load_bible_from_cache().await {
        Ok(cached_bible) => return Ok(cached_bible),
        Err(_) => {
            let bible = fetch_bible_from_api().await?;

            let _ = save_bible_to_cache(&bible).await;

            Ok(bible)
        }
    }
}

async fn load_bible_from_cache() -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    const CACHE_VERSION: &str = "v1";

    let rexie = Rexie::builder("BibleCache")
        .version(1)
        .add_object_store(ObjectStore::new("bible_data"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["bible_data"], TransactionMode::ReadOnly)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("bible_data")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    let version_result = store.get("cache_version".into()).await;
    match version_result {
        Ok(Some(version_value)) => {
            if let Some(version_str) = version_value.as_string() {
                if version_str != CACHE_VERSION {
                    drop(transaction);
                    clear_bible_cache()
                        .await
                        .map_err(|e| format!("Failed to clear cache: {:?}", e))?;
                    return Err("Cache version mismatch".into());
                }
            } else {
                return Err("Invalid cache version format".into());
            }
        }
        Ok(None) => return Err("No cache version found".into()),
        Err(_) => return Err("Failed to read cache version".into()),
    }

    let data_result = store.get("bible_json".into()).await;
    match data_result {
        Ok(Some(data_value)) => {
            if let Some(json_str) = data_value.as_string() {
                let bible: Bible = serde_json::from_str(&json_str)
                    .map_err(|e| format!("Failed to parse cached data: {:?}", e))?;
                Ok(bible)
            } else {
                Err("Invalid cached data format".into())
            }
        }
        Ok(None) => Err("No cached Bible data found".into()),
        Err(_) => Err("Failed to read cached data".into()),
    }
}

async fn save_bible_to_cache(bible: &Bible) -> std::result::Result<(), Box<dyn std::error::Error>> {
    const CACHE_VERSION: &str = "v1";

    let rexie = Rexie::builder("BibleCache")
        .version(1)
        .add_object_store(ObjectStore::new("bible_data"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["bible_data"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("bible_data")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    let json_data = serde_json::to_string(bible)
        .map_err(|e| format!("Failed to serialize Bible data: {:?}", e))?;

    store
        .put(&json_data.into(), Some(&"bible_json".into()))
        .await
        .map_err(|e| format!("Failed to save Bible data: {:?}", e))?;
    store
        .put(&CACHE_VERSION.into(), Some(&"cache_version".into()))
        .await
        .map_err(|e| format!("Failed to save cache version: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}

#[allow(dead_code)]
pub async fn clear_bible_cache() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let rexie = Rexie::builder("BibleCache")
        .version(1)
        .add_object_store(ObjectStore::new("bible_data"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["bible_data"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("bible_data")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    store
        .delete("bible_json".into())
        .await
        .map_err(|e| format!("Failed to delete Bible data: {:?}", e))?;
    store
        .delete("cache_version".into())
        .await
        .map_err(|e| format!("Failed to delete cache version: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}

async fn fetch_bible_from_api() -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    let proxy_urls = [
        "https://corsproxy.io/?https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA",
        "https://api.allorigins.win/get?url=https://gw.iagon.com/api/v2/storage/shareable/link/Njg2ZDFjNDgwOGQ0M2UzNTUyNTdhYmRh:MTJjOTRlYTBmNzM2YWZiZDE2NzdkMzU3NzA3MjBmMTRmZGZkMWYzNWVkYWVlNTU1Y2RjYTA1NzYzZmE1YmEzNA",
    ];

    let mut last_error = None;

    for proxy_url in &proxy_urls {
        match try_fetch_bible(proxy_url).await {
            Ok(bible) => return Ok(bible),
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "All proxy attempts failed".into()))
}

pub async fn try_fetch_bible(url: &str) -> std::result::Result<Bible, Box<dyn std::error::Error>> {
    let response = Request::get(url).send().await?;

    let json_string = if url.contains("allorigins.win") {
        let wrapped: serde_json::Value = response.json().await?;
        wrapped["contents"]
            .as_str()
            .ok_or("Failed to extract contents from allorigins response")?
            .to_string()
    } else {
        response.text().await?
    };

    let bible: Bible = serde_json::from_str(&json_string)?;
    Ok(bible)
}