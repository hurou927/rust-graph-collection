// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A&lang=jp
use std::io::{Read, stdout, Write};
use std::cmp::min;
// use std::collections::BinaryHeap;


macro_rules! simple_print {
    ( $( $x:expr ),* ) => {{
        let out = stdout();
        let mut out = out.lock();
        $( write!(out, "{},", $x).unwrap(); )*
        write!(out, "\n").unwrap();
    }};
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
    rev_index: usize,
}

#[derive(Debug)]
struct Graph {
    node_count: usize,
    edge_count: usize,
    adj_list: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(node_count: usize) -> Graph {
        Graph {
            node_count: node_count, 
            edge_count: 0, 
            adj_list: (0..node_count).map(|_| {Vec::new()}).collect(),
        }
    }
    fn add_edge(&mut self, edge: Edge) {
        self.edge_count += 1;
        self.adj_list[edge.from].push(edge);
    }
    fn edge_size(&self, node: usize) -> usize {
        self.adj_list[node].len()
    }
    fn iter(&self, node: usize) -> GraphIterator {
        GraphIterator{
            graph: self,
            node: node,
            index: 0,
        }
    } 
}

#[derive(Debug)]
struct GraphIterator <'a> {
    graph: &'a Graph,
    node: usize,
    index: usize,
}

impl<'a> Iterator for GraphIterator<'a> {
    type Item = Edge;
    fn next(&mut self) -> Option<Edge> {
        if self.index < self.graph.adj_list[self.node].len(){ 
            let e = self.graph.adj_list[self.node][self.index];
            self.index += 1;
            Some(e)
        }else{
            None
        }
    }
}

fn search_aug_path(g: &mut Graph, start: usize, goal: usize) -> i64 {
    let mut used: Vec<Option<(usize, usize)>> = vec![None; g.node_count];
    let mut stack: Vec<(usize, usize)>  = Vec::new();
    used[start] = Some((start,0));
    for (index, edge) in g.adj_list[start].iter().enumerate() {
        // println!("{} {} {}",edge.from, edge.to, edge.cost);
        if edge.cost > 0{
            // println!("push");
            stack.push((start, index));   
        }
    }
    while let Some(e) = stack.pop() {
        let from = e.0;
        let to_index = e.1;
        let edge: Edge = g.adj_list[from][to_index];
        // println!("{} {} {}",edge.from, edge.to, edge.cost);
        if used[edge.to] != None {
            continue;
        }
        // println!("!! {}->{}, {}", edge.from, edge.to, edge.cost);
        used[edge.to] = Some((edge.from, to_index));
        if edge.to == goal  {
            break;
        }
        for (index, next) in g.adj_list[edge.to].iter().enumerate() {
            if used[next.to] == None && next.cost > 0 {
                stack.push((edge.to, index));
            }
        }
    }

    if used[goal] == None {
        return 0;
    } 

    let mut node_list: Vec<(usize, usize)>  = Vec::new();
    let mut traverse_index = goal;
    let mut flow: i64 = std::i64::MAX;
    loop {
        match used[traverse_index] {
            Some(e) => {
                let from = e.0;
                let index_to = e.1;
                flow = min(flow, g.adj_list[from][index_to].cost);
                traverse_index = e.0;
                node_list.push((from, index_to));
                if traverse_index == start {
                    break;
                } else {
                    traverse_index = from;
                }
            },
            None => {
                println!("Application error: {}", traverse_index);
                return -1;
            }
        }
    }
    for n in node_list.iter() {
        let e = g.adj_list[n.0][n.1];
        g.adj_list[e.from][n.1].cost -= flow;
        g.adj_list[e.to][e.rev_index].cost += flow;
    }
    flow
}

fn search_max_flow(g: &mut Graph, start: usize, goal: usize) -> i64 {
    let mut max_flow: i64 = 0;
    loop {
        let f = search_aug_path(g, start, goal);
        if f <= 0 {
            break;
        }
        max_flow += f;
    }
    max_flow
}


fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    // get stdio    
    let v: usize = iter.next().unwrap().parse::<usize>().unwrap();
    let e: usize = iter.next().unwrap().parse::<usize>().unwrap();
    
    let edges: Vec<(usize, usize, i64)> = (0..e)
        .map(|_| {
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();
    
    let mut graph = Graph::new(v);

    for edge in edges.iter() {
        let atob_index = graph.adj_list[edge.1].len(); // this is required for rust 1.17 in aizu compiler
        let btoa_index = graph.adj_list[edge.0].len();
        graph.add_edge(Edge{from: edge.0, to: edge.1, cost: edge.2, rev_index: atob_index});
        graph.add_edge(Edge{from: edge.1, to: edge.0, cost: 0,      rev_index: btoa_index});
    }
    

    println!("{}", search_max_flow(&mut graph, 0, v-1));

}
