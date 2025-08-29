use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Storage<T> {
    Iagon(Url),
    Local(T),
}

#[derive(Debug, Error)]
pub enum StorageFetchError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),
    
    #[error("HTTP error: {status}")]
    Http { status: u16 },
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl<T> Storage<T> 
where 
    T: for<'de> Deserialize<'de> + Clone,
{
    /// Fetch the data from storage.
    /// 
    /// For Local storage, returns the contained data immediately.
    /// For Iagon storage, fetches the data from the remote URL.
    pub async fn fetch(&self) -> Result<T, StorageFetchError> {
        match self {
            Storage::Local(data) => Ok(data.clone()),
            Storage::Iagon(url) => {
                // For now, we'll use a simple HTTP client approach
                // In the future, this could be replaced with Iagon-specific API calls
                self.fetch_from_url(url).await
            }
        }
    }
    
    /// Fetch data from a URL (HTTP/HTTPS)
    async fn fetch_from_url(&self, url: &Url) -> Result<T, StorageFetchError> {
        // Simple HTTP fetch implementation using curl
        // In a production environment, you might want to use reqwest or similar
        use std::process::Stdio;
        use tokio::process::Command;
        
        let output = Command::new("curl")
            .arg("-s") // silent
            .arg("-L") // follow redirects
            .arg("-f") // fail on HTTP errors
            .arg("--max-time")
            .arg("30") // 30 second timeout
            .arg(url.as_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
            
        if !output.status.success() {
            let status_code = output.status.code().unwrap_or(0) as u16;
            return Err(StorageFetchError::Http { status: status_code });
        }
        
        let response_text = String::from_utf8(output.stdout)?;
        let data: T = serde_json::from_str(&response_text)?;
        
        Ok(data)
    }
    
    /// Check if this storage is local
    pub fn is_local(&self) -> bool {
        matches!(self, Storage::Local(_))
    }
    
    /// Check if this storage is remote (Iagon)
    pub fn is_remote(&self) -> bool {
        matches!(self, Storage::Iagon(_))
    }
    
    /// Get the URL if this is Iagon storage
    pub fn url(&self) -> Option<&Url> {
        match self {
            Storage::Iagon(url) => Some(url),
            Storage::Local(_) => None,
        }
    }
    
    /// Get the local data if this is Local storage
    pub fn local_data(&self) -> Option<&T> {
        match self {
            Storage::Local(data) => Some(data),
            Storage::Iagon(_) => None,
        }
    }
}

impl<T> Storage<T> {
    /// Create a new Local storage
    pub fn local(data: T) -> Self {
        Storage::Local(data)
    }
    
    /// Create a new Iagon storage with a URL
    pub fn iagon(url: Url) -> Self {
        Storage::Iagon(url)
    }
    
    /// Create a new Iagon storage from a URL string
    pub fn iagon_from_str(url_str: &str) -> Result<Self, url::ParseError> {
        let url = Url::parse(url_str)?;
        Ok(Storage::Iagon(url))
    }
}