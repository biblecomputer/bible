use crate::api::{try_fetch_bible, try_fetch_bible_with_progress};
use crate::core::{Bible, init_bible_signal};
use crate::components::custom_translation_import::_remove_custom_translation;
use leptos::prelude::Set;
use gloo_storage::{LocalStorage, Storage};
use rexie::{ObjectStore, Rexie, TransactionMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BibleTranslation {
    pub name: String,
    pub short_name: String,
    pub release_year: u16,
    pub iagon: String,
    pub languages: Vec<Language>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    Dutch,
    English,
}

const SELECTED_TRANSLATION_KEY: &str = "selected_translation";
const DOWNLOADED_TRANSLATIONS_KEY: &str = "downloaded_translations";

pub fn get_selected_translation() -> Option<String> {
    LocalStorage::get(SELECTED_TRANSLATION_KEY).ok()
}

pub fn set_selected_translation(
    translation_short_name: &str,
) -> Result<(), gloo_storage::errors::StorageError> {
    LocalStorage::set(SELECTED_TRANSLATION_KEY, translation_short_name)
}

pub fn get_downloaded_translations() -> Vec<String> {
    LocalStorage::get::<Vec<String>>(DOWNLOADED_TRANSLATIONS_KEY).unwrap_or_default()
}

pub fn add_downloaded_translation(
    translation_short_name: &str,
) -> Result<(), gloo_storage::errors::StorageError> {
    let mut downloaded = get_downloaded_translations();
    if !downloaded.contains(&translation_short_name.to_string()) {
        downloaded.push(translation_short_name.to_string());
        LocalStorage::set(DOWNLOADED_TRANSLATIONS_KEY, &downloaded)?;
    }
    Ok(())
}

pub fn is_translation_downloaded(translation_short_name: &str) -> bool {
    get_downloaded_translations().contains(&translation_short_name.to_string())
}

pub fn remove_downloaded_translation(
    translation_short_name: &str,
) -> Result<(), gloo_storage::errors::StorageError> {
    let mut downloaded = get_downloaded_translations();
    downloaded.retain(|name| name != translation_short_name);
    LocalStorage::set(DOWNLOADED_TRANSLATIONS_KEY, &downloaded)
}

pub async fn switch_bible_translation(
    translation_short_name: &str,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let bible = if is_translation_downloaded(translation_short_name) {
        load_downloaded_translation(translation_short_name).await?
    } else {
        return Err("Translation not downloaded".into());
    };

    let bible_signal = init_bible_signal();
    bible_signal.set(Some(bible));

    Ok(())
}

pub async fn uninstall_translation(
    translation_short_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    remove_downloaded_translation(translation_short_name)?;

    // Also remove from custom translations if it's a custom translation
    if translation_short_name.starts_with("custom_") {
        _remove_custom_translation(translation_short_name)?;
    }

    let translation_cache_key = format!("translation_{}", translation_short_name);
    remove_translation_from_cache(&translation_cache_key).await?;

    if let Some(selected) = get_selected_translation() {
        if selected == translation_short_name {
            let _ = set_selected_translation("sv");
        }
    }

    Ok(())
}

pub async fn download_translation(
    translation: &BibleTranslation,
) -> Result<Bible, Box<dyn std::error::Error>> {
    let bible = fetch_translation_from_url(&translation.iagon).await?;

    let translation_cache_key = format!("translation_{}", translation.short_name);
    save_translation_to_cache_internal(&translation_cache_key, &bible).await?;

    add_downloaded_translation(&translation.short_name)?;

    Ok(bible)
}

pub async fn download_translation_with_progress<F>(
    translation: &BibleTranslation,
    progress_callback: F,
) -> Result<Bible, Box<dyn std::error::Error>>
where
    F: Fn(f32, String) + Clone + 'static,
{
    progress_callback(0.1, "Starting download...".to_string());
    
    let bible = fetch_translation_from_url_with_progress(&translation.iagon, progress_callback.clone()).await?;

    progress_callback(0.8, "Saving to storage...".to_string());
    
    let translation_cache_key = format!("translation_{}", translation.short_name);
    save_translation_to_cache_internal(&translation_cache_key, &bible).await?;

    progress_callback(0.95, "Updating translation list...".to_string());
    
    add_downloaded_translation(&translation.short_name)?;

    progress_callback(1.0, "Download complete!".to_string());

    Ok(bible)
}

pub async fn load_downloaded_translation(
    translation_short_name: &str,
) -> Result<Bible, Box<dyn std::error::Error>> {
    let translation_cache_key = format!("translation_{}", translation_short_name);
    load_translation_from_cache(&translation_cache_key).await
}

async fn fetch_translation_from_url(url: &str) -> Result<Bible, Box<dyn std::error::Error>> {
    let proxy_urls = [
        format!("https://corsproxy.io/?{}", url),
        format!("https://api.allorigins.win/get?url={}", url),
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

async fn fetch_translation_from_url_with_progress<F>(
    url: &str, 
    progress_callback: F
) -> Result<Bible, Box<dyn std::error::Error>>
where
    F: Fn(f32, String) + Clone + 'static,
{
    let proxy_urls = [
        format!("https://corsproxy.io/?{}", url),
        format!("https://api.allorigins.win/get?url={}", url),
    ];

    let mut last_error = None;

    for (i, proxy_url) in proxy_urls.iter().enumerate() {
        progress_callback(0.2 + (i as f32 * 0.1), format!("Trying download server {}...", i + 1));
        
        match try_fetch_bible_with_progress(proxy_url, progress_callback.clone()).await {
            Ok(bible) => return Ok(bible),
            Err(e) => {
                last_error = Some(e);
                continue;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| "All proxy attempts failed".into()))
}


async fn load_translation_from_cache(cache_key: &str) -> Result<Bible, Box<dyn std::error::Error>> {
    let rexie = Rexie::builder("TranslationCache")
        .version(1)
        .add_object_store(ObjectStore::new("translations"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["translations"], TransactionMode::ReadOnly)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("translations")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    let data_result = store.get(cache_key.into()).await;
    match data_result {
        Ok(Some(data_value)) => {
            if let Some(json_str) = data_value.as_string() {
                let bible: Bible = serde_json::from_str(&json_str)
                    .map_err(|e| format!("Failed to parse cached translation: {:?}", e))?;
                Ok(bible)
            } else {
                Err("Invalid cached translation format".into())
            }
        }
        Ok(None) => Err("Translation not found in cache".into()),
        Err(_) => Err("Failed to read cached translation".into()),
    }
}

pub async fn save_translation_to_cache(
    cache_key: &str,
    bible: &Bible,
) -> Result<(), Box<dyn std::error::Error>> {
    save_translation_to_cache_internal(cache_key, bible).await
}

async fn save_translation_to_cache_internal(
    cache_key: &str,
    bible: &Bible,
) -> Result<(), Box<dyn std::error::Error>> {
    let rexie = Rexie::builder("TranslationCache")
        .version(1)
        .add_object_store(ObjectStore::new("translations"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["translations"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("translations")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    let json_data = serde_json::to_string(bible)
        .map_err(|e| format!("Failed to serialize Bible data: {:?}", e))?;

    store
        .put(&json_data.into(), Some(&cache_key.into()))
        .await
        .map_err(|e| format!("Failed to save translation data: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}

async fn remove_translation_from_cache(cache_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rexie = Rexie::builder("TranslationCache")
        .version(1)
        .add_object_store(ObjectStore::new("translations"))
        .build()
        .await
        .map_err(|e| format!("Failed to open IndexedDB: {:?}", e))?;

    let transaction = rexie
        .transaction(&["translations"], TransactionMode::ReadWrite)
        .map_err(|e| format!("Failed to create transaction: {:?}", e))?;
    let store = transaction
        .store("translations")
        .map_err(|e| format!("Failed to get store: {:?}", e))?;

    store
        .delete(cache_key.into())
        .await
        .map_err(|e| format!("Failed to delete translation from cache: {:?}", e))?;

    transaction
        .commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {:?}", e))?;
    Ok(())
}