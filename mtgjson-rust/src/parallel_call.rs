/// Wrapper around creating a parallel function call
// matches the Python parallel_call signature exactly
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

/// Configuration for parallel execution
pub struct ParallelConfig {
    pub fold_list: bool,
    pub fold_dict: bool,
    pub force_starmap: bool,
    pub pool_size: usize,
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            fold_list: false,
            fold_dict: false,
            force_starmap: false,
            pool_size: 32,
        }
    }
}

/// Execute a function in parallel with basic arguments
pub fn parallel_call_simple<T, R, F>(
    function: F,
    args: Vec<T>,
    config: Option<ParallelConfig>,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(T) -> R + Send + Sync,
{
    let config = config.unwrap_or_default();
    
    // Configure rayon thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.pool_size)
        .build()
        .unwrap_or_else(|_| rayon::ThreadPool::current());
    
    pool.install(|| {
        args.into_par_iter()
            .map(function)
            .collect()
    })
}

/// Execute a function in parallel with repeatable arguments
pub fn parallel_call_with_repeatable<T, R, F, A>(
    function: F,
    args: Vec<T>,
    repeatable_args: Vec<A>,
    config: Option<ParallelConfig>,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    A: Clone + Send + Sync,
    F: Fn(T, &[A]) -> R + Send + Sync,
{
    let config = config.unwrap_or_default();
    let repeatable_args = Arc::new(repeatable_args);
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.pool_size)
        .build()
        .unwrap_or_else(|_| rayon::ThreadPool::current());
    
    pool.install(|| {
        args.into_par_iter()
            .map(|arg| {
                let repeated = Arc::clone(&repeatable_args);
                function(arg, &repeated)
            })
            .collect()
    })
}

/// Execute a function in parallel with tuple arguments (starmap equivalent)
pub fn parallel_call_starmap<T, R, F>(
    function: F,
    args: Vec<T>,
    config: Option<ParallelConfig>,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(T) -> R + Send + Sync,
{
    let config = config.unwrap_or_default();
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.pool_size)
        .build()
        .unwrap_or_else(|_| rayon::ThreadPool::current());
    
    pool.install(|| {
        args.into_par_iter()
            .map(function)
            .collect()
    })
}

/// Execute a function in parallel and fold results into a single vector
pub fn parallel_call_fold_list<T, R, F>(
    function: F,
    args: Vec<T>,
    config: Option<ParallelConfig>,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(T) -> Vec<R> + Send + Sync,
{
    let config = config.unwrap_or_default();
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.pool_size)
        .build()
        .unwrap_or_else(|_| rayon::ThreadPool::current());
    
    pool.install(|| {
        args.into_par_iter()
            .map(function)
            .flatten()
            .collect()
    })
}

/// Execute a function in parallel and fold results into a single HashMap
pub fn parallel_call_fold_dict<T, K, V, F>(
    function: F,
    args: Vec<T>,
    config: Option<ParallelConfig>,
) -> HashMap<K, V>
where
    T: Send + Sync,
    K: Send + Eq + Hash,
    V: Send,
    F: Fn(T) -> HashMap<K, V> + Send + Sync,
{
    let config = config.unwrap_or_default();
    
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.pool_size)
        .build()
        .unwrap_or_else(|_| rayon::ThreadPool::current());
    
    pool.install(|| {
        args.into_par_iter()
            .map(function)
            .reduce(HashMap::new, |mut acc, map| {
                acc.extend(map);
                acc
            })
    })
}

/// Main parallel call function that mimics the Python version
pub fn parallel_call<T, R, F>(
    function: F,
    args: Vec<T>,
    config: ParallelConfig,
) -> ParallelResult<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(T) -> R + Send + Sync,
{
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.pool_size)
        .build()
        .unwrap_or_else(|_| rayon::ThreadPool::current());
    
    let results: Vec<R> = pool.install(|| {
        if config.force_starmap {
            args.into_par_iter().map(function).collect()
        } else {
            args.into_par_iter().map(function).collect()
        }
    });
    
    ParallelResult::new(results, config)
}

/// Wrapper for parallel execution results with folding options
pub struct ParallelResult<R> {
    results: Vec<R>,
    config: ParallelConfig,
}

impl<R> ParallelResult<R> {
    fn new(results: Vec<R>, config: ParallelConfig) -> Self {
        Self { results, config }
    }
    
    /// Get the raw results
    pub fn into_results(self) -> Vec<R> {
        self.results
    }
    
    /// Fold results into a single vector (for when R is Vec<T>)
    pub fn fold_list<T>(self) -> Vec<T>
    where
        R: IntoIterator<Item = T>,
    {
        self.results.into_iter().flatten().collect()
    }
    
    /// Fold results into a single HashMap (for when R is HashMap<K, V>)
    pub fn fold_dict<K, V>(self) -> HashMap<K, V>
    where
        R: IntoIterator<Item = (K, V)>,
        K: Eq + Hash,
    {
        self.results.into_iter().flatten().collect()
    }
}

// Example usage and utility functions for MTGJSON

use crate::classes::MtgjsonCardObject;
use serde_json::Value;

/// Parallel card processing function
pub fn parallel_process_cards<F>(
    card_data: Vec<Value>,
    processor: F,
    pool_size: Option<usize>,
) -> Vec<MtgjsonCardObject>
where
    F: Fn(Value) -> MtgjsonCardObject + Send + Sync,
{
    let config = ParallelConfig {
        pool_size: pool_size.unwrap_or(32),
        ..Default::default()
    };
    
    parallel_call_simple(processor, card_data, Some(config))
}

/// Parallel batch processing with custom function
pub fn parallel_batch_process<T, R, F>(
    items: Vec<T>,
    batch_size: usize,
    processor: F,
    pool_size: Option<usize>,
) -> Vec<R>
where
    T: Send + Sync,
    R: Send,
    F: Fn(Vec<T>) -> Vec<R> + Send + Sync,
{
    let config = ParallelConfig {
        fold_list: true,
        pool_size: pool_size.unwrap_or(32),
        ..Default::default()
    };
    
    // Split items into batches
    let batches: Vec<Vec<T>> = items
        .chunks(batch_size)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    parallel_call_fold_list(processor, batches, Some(config))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parallel_call_simple() {
        let numbers = vec![1, 2, 3, 4, 5];
        let results = parallel_call_simple(|x| x * 2, numbers, None);
        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }
    
    #[test]
    fn test_parallel_call_fold_list() {
        let numbers = vec![1, 2, 3];
        let results = parallel_call_fold_list(
            |x| vec![x, x * 2], 
            numbers, 
            None
        );
        assert_eq!(results, vec![1, 2, 2, 4, 3, 6]);
    }
    
    #[test]
    fn test_parallel_call_fold_dict() {
        let numbers = vec![1, 2, 3];
        let results = parallel_call_fold_dict(
            |x| {
                let mut map = HashMap::new();
                map.insert(x, x * 2);
                map
            }, 
            numbers, 
            None
        );
        
        assert_eq!(results.get(&1), Some(&2));
        assert_eq!(results.get(&2), Some(&4));
        assert_eq!(results.get(&3), Some(&6));
    }
} 