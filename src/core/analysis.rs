use std::collections::HashSet;

fn jaccard_index<N>(set1: &HashSet<(N, N)>, set2: &HashSet<(N, N)>) -> f64
where
    N: Eq + std::hash::Hash,
{
    let intersection_size = set1.intersection(set2).count();
    let union_size = set1.union(set2).count();
    intersection_size as f64 / union_size as f64
}
#[cfg(test)]
mod tests {

    use crate::core::io::read_edge_list;

    use crate::core::utils::graph_to_edge_set;

    use super::jaccard_index;

    #[test]
    fn test_read_edgelist() {
        let data1 = read_edge_list("resources/test/data/fseek_0.0_cluster.tsv").unwrap();
        let set1 = graph_to_edge_set(&data1);

        let data2 = read_edge_list("resources/test/data/mmseqs_0.8_cluster.tsv").unwrap();
        let set2 = graph_to_edge_set(&data2);
        let jaccard = jaccard_index(&set1, &set2);
        dbg!(jaccard);
    }
    #[test]
    fn test_same_file() {
        let data1 = read_edge_list("resources/test/data/mmseqs_0.8_cluster.tsv").unwrap();
        let set1 = graph_to_edge_set(&data1);

        let data2 = read_edge_list("resources/test/data/mmseqs_0.8_cluster.tsv").unwrap();
        let set2 = graph_to_edge_set(&data2);
        let jaccard = jaccard_index(&set1, &set2);
        assert_eq!(jaccard, 1.0);
    }
}
