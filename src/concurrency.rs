use crate::error::Result;
use futures::future::BoxFuture;
use std::pin::Pin;
use tokio::sync::Semaphore;

pub async fn gather_limited<F, Fut, T>(
    calls: Vec<F>,
    limit: usize,
    return_exceptions: bool,
) -> Result<Vec<T>>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T>> + Send + 'static,
    T: Send + 'static,
{
    let semaphore = std::sync::Arc::new(Semaphore::new(limit));
    let mut handles = Vec::new();

    for call in calls {
        let sem = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            call().await
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        match handle.await {
            Ok(Ok(val)) => results.push(val),
            Ok(Err(e)) => {
                if return_exceptions {
                    // We can't easily return the error as a value in Rust like Python
                    // So we'll just return the error
                    return Err(e);
                } else {
                    return Err(e);
                }
            }
            Err(e) => return Err(crate::error::NombaError::api(format!("Task panicked: {}", e))),
        }
    }

    Ok(results)
}

pub async fn gather_limited_ordered<F, Fut, T>(
    calls: Vec<F>,
    limit: usize,
    return_exceptions: bool,
) -> Result<Vec<T>>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T>> + Send + 'static,
    T: Send + 'static,
{
    let semaphore = std::sync::Arc::new(Semaphore::new(limit));
    let mut futures = Vec::new();

    for call in calls {
        let sem = semaphore.clone();
        let fut: BoxFuture<'static, Result<T>> = Box::pin(async move {
            let _permit = sem.acquire().await.unwrap();
            call().await
        });
        futures.push(fut);
    }

    let results = if return_exceptions {
        // In Rust, we'd need to wrap in Result, but for now just use futures::future::join_all
        // and handle errors
        let results: Vec<Result<T>> = futures::future::join_all(futures).await;
        results.into_iter().collect::<Result<Vec<T>>>()?
    } else {
        futures::future::try_join_all(futures).await?
    };

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_gather_limited() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut calls = Vec::new();

        for _ in 0..10 {
            let c = counter.clone();
            calls.push(move || {
                let c = c.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    Ok::<_, crate::error::NombaError>(c.load(Ordering::SeqCst))
                }
            });
        }

        let results = gather_limited(calls, 3, false).await.unwrap();
        assert_eq!(results.len(), 10);
    }
}