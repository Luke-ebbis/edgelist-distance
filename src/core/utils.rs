use std::collections::HashSet;

use petgraph::{visit::EdgeRef, Graph};

pub(crate) fn graph_to_edge_set<N, E>(graph: &Graph<N, E>) -> HashSet<(N, N)>
where
    N: Clone + Eq + std::hash::Hash,
{
    let mut edge_set = HashSet::new();
    for edge in graph.edge_references() {
        let source = graph[edge.source()].clone();
        let target = graph[edge.target()].clone();
        edge_set.insert((source, target));
    }
    edge_set
}

#[cfg(test)]
mod tests {
    use bio::io::fasta::Record;
    use kodama::{linkage, Method};

    use crate::core::io::read_edge_list;
    use rayon::iter::IntoParallelRefIterator;

    use super::graph_to_edge_set;

    #[test]
    fn test_read_edgelist() {
        let data = read_edge_list("resources/test/data/fseek_0.0_cluster.tsv").unwrap();
        let result = graph_to_edge_set(&data);
    }
}
