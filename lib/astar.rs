extern crate std;

use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
struct FNode<N: Eq>(i32, N);

impl<N: Eq> Ord for FNode<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl<N: Eq> PartialOrd for FNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Graph {
    type Node: Clone + Eq + std::hash::Hash;
    type Move: Copy + Clone;

    fn neighbors(&self, &Self::Node) -> Vec<(Self::Move, Self::Node)>;
    fn is_goal(&self, &Self::Node) -> bool;
}

fn build_path<G>(come_from: &HashMap<G::Node, (G::Move, G::Node)>, end: G::Node) -> Vec<(G::Move, G::Node)>
    where G: Graph
{
    let mut ret = Vec::new();
    let mut prev = &end;

    while let Some(&(ref mov, ref curr)) = come_from.get(&prev) {
        ret.push((*mov, (*curr).clone()));
        prev = curr;
    }
    ret.reverse();
    ret
}

pub fn a_star<G, H>(graph: &G, start: G::Node, h: H) -> (usize, Option<Vec<(G::Move, G::Node)>>)
    where G: Graph,
          H: Fn(&G, &G::Node) -> i32
{
    let mut frontier: BinaryHeap<FNode<G::Node>> = BinaryHeap::new();
    let mut come_from: HashMap<G::Node, (G::Move, G::Node)> = HashMap::new();
    let mut cost: HashMap<G::Node, i32> = HashMap::new();
    let mut counter = 0;
    let max = std::i32::MAX;

    frontier.push(FNode(0, start.clone()));
    cost.insert(start.clone(), 0);

    while let Some(FNode(_, curr)) = frontier.pop() {
        counter += 1;
        if graph.is_goal(&curr) {
            let path = build_path::<G>(&come_from, curr);
            return (counter, Some(path));
        }

        for (mov, next) in graph.neighbors(&curr) {
            let new_cost = cost[&curr] + 1;
            let old_cost = *cost.get(&next).unwrap_or(&max);
            if new_cost < old_cost {
                cost.insert(next.clone(), new_cost);
                come_from.insert(next.clone(), (mov, curr.clone()));
                frontier.push(FNode(new_cost + h(graph, &next), next.clone()));
            }
        }
    }
    (counter, None)
}
