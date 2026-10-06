use crate::error::Result;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

fn unwrap_response(response: &serde_json::Value) -> &serde_json::Value {
    if response.get("results").is_some() || response.get("cursor").is_some() {
        return response;
    }
    response.get("data").unwrap_or(response)
}

pub struct Paginator<F, T> {
    method: F,
    limit: Option<u32>,
    cursor: Option<String>,
    buffer: Vec<T>,
    exhausted: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<F, T> Paginator<F, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Result<serde_json::Value>,
    T: serde::de::DeserializeOwned,
{
    pub fn new(method: F, limit: Option<u32>) -> Self {
        Self {
            method,
            limit,
            cursor: None,
            buffer: Vec::new(),
            exhausted: false,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, T> Iterator for Paginator<F, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Result<serde_json::Value>,
    T: serde::de::DeserializeOwned,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        loop {
            if let Some(item) = self.buffer.pop() {
                return Some(Ok(item));
            }

            let resp = match (self.method)(self.limit, self.cursor.clone()) {
                Ok(r) => r,
                Err(e) => return Some(Err(e)),
            };

            let page = unwrap_response(&resp);
            let results = page.get("results").and_then(|v| v.as_array())?;

            if results.is_empty() {
                self.exhausted = true;
                return None;
            }

            self.cursor = page
                .get("cursor")
                .and_then(|c| c.as_str())
                .map(|s| s.to_string());

            self.buffer = results
                .iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .rev()
                .collect();
        }
    }
}

pub struct AsyncPaginator<F, Fut, T> {
    method: F,
    limit: Option<u32>,
    cursor: Option<String>,
    buffer: Vec<T>,
    pending: Option<Pin<Box<Fut>>>,
    exhausted: bool,
}

impl<F, Fut, T> AsyncPaginator<F, Fut, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Fut,
    Fut: std::future::Future<Output = Result<serde_json::Value>>,
    T: serde::de::DeserializeOwned,
{
    pub fn new(method: F, limit: Option<u32>) -> Self {
        Self {
            method,
            limit,
            cursor: None,
            buffer: Vec::new(),
            pending: None,
            exhausted: false,
        }
    }
}

impl<F, Fut, T> Stream for AsyncPaginator<F, Fut, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Fut,
    Fut: std::future::Future<Output = Result<serde_json::Value>>,
    T: serde::de::DeserializeOwned,
{
    type Item = Result<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // SAFETY: the struct is never moved out of, contains no
        // self-referential data, and all access below is through `&mut`,
        // so an unpinned mutable reference is sound here. This avoids
        // requiring `F: Unpin` / `Fut: Unpin`, which natural async closures
        // (e.g. `async move` blocks awaiting HTTP calls) do not satisfy.
        let this = unsafe { self.get_unchecked_mut() };

        if this.exhausted {
            return Poll::Ready(None);
        }

        loop {
            if let Some(item) = this.buffer.pop() {
                return Poll::Ready(Some(Ok(item)));
            }

            // Reuse the in-flight request across polls instead of starting
            // a new one on every wakeup.
            if this.pending.is_none() {
                let limit = this.limit;
                let cursor = this.cursor.clone();
                this.pending = Some(Box::pin((this.method)(limit, cursor)));
            }

            let pending = this.pending.as_mut().expect("pending future just stored");
            match pending.as_mut().poll(cx) {
                Poll::Ready(Ok(resp)) => {
                    this.pending = None;
                    let page = unwrap_response(&resp);
                    let results = match page.get("results").and_then(|v| v.as_array()) {
                        Some(r) => r,
                        None => return Poll::Ready(None),
                    };

                    if results.is_empty() {
                        this.exhausted = true;
                        return Poll::Ready(None);
                    }

                    this.cursor = page
                        .get("cursor")
                        .and_then(|c| c.as_str())
                        .map(|s| s.to_string());

                    this.buffer = results
                        .iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .rev()
                        .collect();
                }
                Poll::Ready(Err(e)) => {
                    this.pending = None;
                    return Poll::Ready(Some(Err(e)));
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

/// Iterates cursor pages for you. `method` receives `(limit, cursor)` and
/// must return the raw page as a [`serde_json::Value`]; items are
/// deserialized from the page's `results` array.
///
/// ```no_run
/// use nomba_rs::{Nomba, paginate};
/// use std::collections::HashMap;
///
/// let nomba = Nomba::new("client_id", "client_secret", "account_id")?;
///
/// let accounts = paginate(
///     |limit, cursor| {
///         nomba
///             .virtual_accounts
///             .filter_virtual_accounts(
///                 limit.map(|n| n.to_string()),
///                 cursor,
///                 None, None, None, None, None, None, None, None,
///             )
///             .and_then(|resp| serde_json::to_value(resp.data).map_err(nomba_rs::NombaError::from))
///     },
///     Some(50),
/// );
///
/// for account in accounts {
///     let account: HashMap<String, serde_json::Value> = account?;
///     println!("{:?}", account.get("accountRef"));
/// }
/// # Ok::<(), nomba_rs::NombaError>(())
/// ```
pub fn paginate<F, T>(method: F, limit: Option<u32>) -> Paginator<F, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Result<serde_json::Value>,
    T: serde::de::DeserializeOwned,
{
    Paginator::new(method, limit)
}

/// Async variant of [`paginate`]: drives cursor pages as a [`futures::Stream`].
///
/// ```no_run
/// use nomba_rs::{AsyncNomba, apaginate};
/// use futures::StreamExt;
/// use std::collections::HashMap;
///
/// #[tokio::main]
/// async fn main() -> nomba_rs::Result<()> {
///     let nomba = AsyncNomba::new("client_id", "client_secret", "account_id").await?;
///
///     let mut stream = apaginate(
///         |limit, cursor| {
///             let accounts = nomba.virtual_accounts.clone();
///             async move {
///                 accounts
///                     .filter_virtual_accounts(
///                         limit.map(|n| n.to_string()),
///                         cursor,
///                         None, None, None, None, None, None, None, None,
///                     )
///                     .await
///                     .and_then(|resp| serde_json::to_value(resp.data).map_err(nomba_rs::NombaError::from))
///             }
///         },
///         Some(50),
///     );
///
///     while let Some(account) = stream.next().await {
///         let account: HashMap<String, serde_json::Value> = account?;
///         println!("{:?}", account.get("accountRef"));
///     }
///     Ok(())
/// }
/// ```
pub fn apaginate<F, Fut, T>(method: F, limit: Option<u32>) -> AsyncPaginator<F, Fut, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Fut,
    Fut: std::future::Future<Output = Result<serde_json::Value>>,
    T: serde::de::DeserializeOwned,
{
    AsyncPaginator::new(method, limit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_unwrap_response() {
        let wrapped = json!({"code": "00", "description": "Success", "data": {"results": [], "cursor": null}});
        assert!(unwrap_response(&wrapped).get("results").is_some());

        let unwrapped = json!({"results": [], "cursor": null});
        assert!(unwrap_response(&unwrapped).get("results").is_some());
    }
}
