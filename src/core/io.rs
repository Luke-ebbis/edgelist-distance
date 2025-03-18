//! # Input and output of files
use petgraph::graph::{Graph, NodeIndex};
use petgraph::{Directed, Undirected};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Function to read edge lists
///
/// From ChatGPT
///
///
pub(crate) fn read_edge_list<P: AsRef<Path>>(
    filename: P,
) -> io::Result<Graph<String, (), Directed>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut graph = Graph::<String, (), Directed>::new();
    let mut node_indices = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 2 {
            continue; // Skip malformed lines
        }
        let (node1, node2) = (parts[0].to_string(), parts[1].to_string());

        let index1 = *node_indices
            .entry(node1.clone())
            .or_insert_with(|| graph.add_node(node1));
        let index2 = *node_indices
            .entry(node2.clone())
            .or_insert_with(|| graph.add_node(node2));

        graph.add_edge(index1, index2, ());
    }
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use bio::io::fasta::Record;
    use kodama::{linkage, Method};

    use crate::core::io::read_edge_list;
    use rayon::iter::IntoParallelRefIterator;

    #[test]
    fn test_read_edgelist() {
        let data = read_edge_list("resources/test/data/fseek_0.0_cluster.tsv").unwrap();
        let result = data.node_count();
    }
}
