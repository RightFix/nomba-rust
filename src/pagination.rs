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
            let results = match page.get("results").and_then(|v| v.as_array()) {
                Some(r) => r,
                None => return None,
            };

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
    exhausted: bool,
    _phantom: std::marker::PhantomData<(Fut, T)>,
}

impl<F, Fut, T> AsyncPaginator<F, Fut, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Fut + Unpin,
    Fut: std::future::Future<Output = Result<serde_json::Value>>,
    T: serde::de::DeserializeOwned + Unpin,
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

impl<F, Fut, T> Stream for AsyncPaginator<F, Fut, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Fut + Unpin,
    Fut: std::future::Future<Output = Result<serde_json::Value>> + Unpin,
    T: serde::de::DeserializeOwned + Unpin,
{
    type Item = Result<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Get mutable access to self
        let this = self.get_mut();

        if this.exhausted {
            return Poll::Ready(None);
        }

        loop {
            if let Some(item) = this.buffer.pop() {
                return Poll::Ready(Some(Ok(item)));
            }

            // Call the method to get a future
            let limit = this.limit;
            let cursor = this.cursor.clone();
            let future = (this.method)(limit, cursor);

            // Pin the future and poll it
            let mut future = Box::pin(future);
            match future.as_mut().poll(cx) {
                Poll::Ready(Ok(resp)) => {
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
                Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

pub fn paginate<F, T>(method: F, limit: Option<u32>) -> Paginator<F, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Result<serde_json::Value>,
    T: serde::de::DeserializeOwned,
{
    Paginator::new(method, limit)
}

pub fn apaginate<F, Fut, T>(method: F, limit: Option<u32>) -> AsyncPaginator<F, Fut, T>
where
    F: FnMut(Option<u32>, Option<String>) -> Fut + Unpin,
    Fut: std::future::Future<Output = Result<serde_json::Value>>,
    T: serde::de::DeserializeOwned + Unpin,
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
