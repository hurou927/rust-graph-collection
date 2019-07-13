use std::io::{Read, stdout, Write};
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;


macro_rules! simple_print {
    ( $( $x:expr ),* ) => {{
        let out = stdout();
        let mut out = out.lock();
        $( write!(out, "{},", $x).unwrap(); )*
        write!(out, "\n").unwrap();
    }};
}

// #[derive(Copy, Clone, Eq, PartialEq)]
// struct Edge {
//     from: usize,
//     to: usize,
//     cost: i64,
// }

// struct Graph {
//     node_count: usize,
//     edge_count: usize,
//     adj_list: Vec<Vec<Edge>>,
// }

// impl Graph {
//     fn new(node_count: usize) -> Graph {
//         Graph {
//             node_count: node_count, 
//             edge_count: 0, 
//             adj_list: (0..node_count).map(|_| {Vec::new()}).collect(),
//         }
//     }
//     fn add_edge(&mut self, edge: Edge) {
//         self.edge_count += 1;
//         self.adj_list[edge.from].push(edge);
//     }
//     fn iter(&self, node: usize) -> GraphIterator {
//         GraphIterator{
//             graph: self,
//             node: node,
//             index: 0,
//         }
//     } 
// }


// struct GraphIterator <'a> {
//     graph: &'a Graph,
//     node: usize,
//     index: usize,
// }

// impl<'a> Iterator for GraphIterator<'a> {
//     type Item = Edge;
//     fn next(&mut self) -> Option<Edge> {
//         if self.index < self.graph.adj_list[self.node].len(){ 
//             let e = self.graph.adj_list[self.node][self.index];
//             self.index += 1;
//             Some(e)
//         }else{
//             None
//         }
//     }
// }


struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n :usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![1;n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let par = self.parent[x];
            let res = self.find(par);
            self.parent[x] = res;
            res
        }
    }
    fn union(&mut self, a: usize, b:usize) {
        let a_parent = self.find(a);
        let b_parent = self.find(b);
        if self.rank[a_parent] > self.rank[b_parent] {
            self.parent[b_parent] = a_parent;

        } else if self.rank[b_parent] < self.rank[a_parent] {
            self.parent[a_parent] = b_parent

        } else if a_parent != b_parent {
            self.parent[b_parent] = a_parent;
            self.rank[a_parent] += 1;

        } 
    }
    fn same(&mut self, a:usize, b:usize) -> bool {
        self.find(a) == self.find(b)
    }
}


fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    // get stdio    
    let v: usize = iter.next().unwrap().parse::<usize>().unwrap();
    let e: usize = iter.next().unwrap().parse::<usize>().unwrap();
    
    let mut edges: Vec<(usize, usize, i64)> = (0..e)
        .map(|_| {
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();


    // let mut graph = Graph::new(v);

    // for edge in edges.iter() {
    //     graph.add_edge(Edge{from: edge.0, to: edge.1, cost: edge.2});
    // }
    // simple_print!(v, e);

    let mut uf = UnionFind::new(v);
    edges.sort_by(|a, b| a.2.cmp(&b.2));
    
    let mut sum_cost = 0;
    let mut sum_edge = 0;
    for edge in edges.iter() {
        // simple_print!(edge.2);
        if !uf.same(edge.0, edge.1) {
            uf.union(edge.0, edge.1);
            // simple_print!(edge.0, edge.1, edge.2);
            sum_cost += edge.2;
            sum_edge += 1;
            if sum_edge == v-1{
                break;
            }
        } 
    }

    println!("{}", sum_cost);

}


