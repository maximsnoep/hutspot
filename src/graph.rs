use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::HashMap;

// Find the shortest path from element `a` to `b` using Dijkstra's algorithm.
// Neighborhood of a vertex is defined by `neighbor_function`, and the weight a pair elements is defined by `weight_function
pub fn find_shortest_path<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + Copy>(
    a: T,
    b: T,
    neighbor_function: impl Fn(T) -> Vec<T>,
    weight_function: impl Fn(T, T) -> OrderedFloat<f32>,
    cache: &mut HashMap<T, Vec<(T, OrderedFloat<f32>)>>,
) -> Option<(Vec<T>, OrderedFloat<f32>)> {
    pathfinding::prelude::dijkstra(
        &a,
        |&elem| {
            if cache.contains_key(&elem) {
                cache[&elem].clone()
            } else {
                let neighbors = neighbor_function(elem)
                    .iter()
                    .map(|&neighbor| (neighbor, weight_function(elem, neighbor)))
                    .collect_vec();
                cache.insert(elem, neighbors.clone());
                neighbors
            }
        },
        |&elem| elem == b,
    )
}

// Find the shortest cycle through element `a`, using the `find_shortest_path` function.
pub fn find_shortest_cycle<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + Copy>(
    a: T,
    neighbor_function: impl Fn(T) -> Vec<T>,
    weight_function: impl Fn(T, T) -> OrderedFloat<f32>,
    cache: &mut HashMap<T, Vec<(T, OrderedFloat<f32>)>>,
) -> Option<(Vec<T>, OrderedFloat<f32>)> {
    neighbor_function(a)
        .iter()
        .filter_map(|&neighbor| {
            find_shortest_path(neighbor, a, &neighbor_function, &weight_function, cache)
        })
        .sorted_by(|(_, cost1), (_, cost2)| cost1.cmp(cost2))
        .next()
        .map(|(path, score)| ([vec![a], path].concat(), score))
}
