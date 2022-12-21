use std::collections::{HashMap, VecDeque};
use std::hash::Hash;


pub trait Graph {
    type Position: Clone + Eq + Hash;
    type Property;

    fn get_property(&self, pos: &Self::Position) -> &Self::Property;
    fn get_surroundings(&self, pos: &Self::Position) -> Vec<Self::Position>;
}

pub struct Bfs<'a, F, G>
    where F: Fn(&G::Property, &G::Property) -> bool,
          G: Graph,
{
    graph: &'a G,
    filter: F,
}

impl<'a, F, G> Bfs<'a, F, G>
    where F: Fn(&G::Property, &G::Property) -> bool,
        G: Graph,
{

    pub fn new(graph: &'a G, filter: F) -> Bfs<'a, F, G> {
        Bfs {
            graph,
            filter,
        }
    }

    pub fn run(&self, start_pos: &'a G::Position) -> HashMap<G::Position, u64> {
        let mut frontier: VecDeque<(G::Position, u64)> = VecDeque::new();
        frontier.push_back((start_pos.clone(), 0));

        let mut dists: HashMap<G::Position, u64> = HashMap::new();

        while let Some((cur, dist)) = frontier.pop_front() {

            if dists.contains_key(&cur) {
                continue;
            }

            dists.insert(cur.clone(), dist);

            let surroundings = self.graph.get_surroundings(&cur);

            for next in surroundings {
                if dists.contains_key(&next) {
                    continue;
                }

                let cur_height = self.graph.get_property(&cur);
                let next_height = self.graph.get_property(&next);

                if !(self.filter)(cur_height, next_height) {
                    continue;
                }

                frontier.push_back((next, dist+1));
            }

        }

        dists
    }
}
