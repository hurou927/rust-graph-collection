use std::io::Read;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge{
    from: usize,
    to: usize,
    weight: i64,
}
impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn dijkstra_heap(graph: &Vec<Vec<Edge>>, start: usize) -> Vec<(usize, i64)> {
    let n = graph.len();
    // let mut stack: Vec<Edge>  = Vec::new();
    let mut heap = BinaryHeap::new();
    let mut dist : Vec<(usize, i64)> = vec![(std::usize::MAX, std::i64::MAX); n];
    
    heap.push((start,0));
    dist[start] = (start, 0);
    while let Some((position, cost)) = heap.pop(){

        let cost_s2f = dist[position].1;
        if cost_s2f < cost {
            continue;
        }
        for &next in graph[position].iter() {
            let cost_f2n = next.weight;
            let cost_s2n = dist[next.to].1;
            let new_edge = (next.to, cost_s2f + cost_f2n);
            // println!("!!! {}=>{} / {} {} {}",position, i.to, cost_s2n , cost_s2f , cost_f2n  );
            if cost_s2n > new_edge.1 {
                dist[next.to] = (position, cost_s2f + cost_f2n);
                heap.push(new_edge);
            }
        }
    }
    dist
}

fn dijkstra_bruteforce(graph: &Vec<Vec<Edge>>, start: usize) -> Vec<(usize, i64)> {
    let n = graph.len();
    // let mut stack: Vec<Edge>  = Vec::new();
    // let mut heap = BinaryHeap::new();
    let mut dist : Vec<(usize, i64)> = vec![(std::usize::MAX, std::i64::MAX); n];
    let mut visited : Vec<bool> = vec![false; n];

    // heap.push((start,0));
    dist[start] = (start, 0);
    loop {
        let mut position = n;
        let mut cost = std::i64::MAX;
        for i in 0..n {
            if !visited[i] && dist[i].1 < cost{
                position = i;
                cost = dist[i].1;
            }
        }
        if position == n {
            break;
        }

        let cost_s2f = dist[position].1;
        visited[position] = true;
        // if cost_s2f < cost {
        //     continue;
        // }
        for &next in graph[position].iter() {
            let cost_f2n = next.weight;
            let cost_s2n = dist[next.to].1;
            let new_edge = (next.to, cost_s2f + cost_f2n);
            // println!("!!! {}=>{} / {} {} {}",position, i.to, cost_s2n , cost_s2f , cost_f2n  );
            if cost_s2n > new_edge.1 {
                dist[next.to] = (position, cost_s2f + cost_f2n);
                // heap.push(new_edge);
            }
        }
    }
    dist
}


fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    // get stdio    
    let n: usize = iter.next().unwrap().parse::<usize>().unwrap();
    let m: usize = iter.next().unwrap().parse::<usize>().unwrap();
    let s: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    let t: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
    
    let edges: Vec<(usize, usize, i64, i64)> = (0..m)
        .map(|_| {
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<i64>().unwrap(),
                iter.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();

    // construct graph
    let mut graph_a: Vec<Vec<Edge>> = (0..n)
        .map(|_| {
            Vec::new()
        }).collect();
    
    let mut graph_b: Vec<Vec<Edge>> = (0..n)
        .map(|_| {
            Vec::new()
        }).collect();

    for e in edges.iter() {
        graph_a[e.0].push( Edge{from:e.0, to:e.1, weight:e.2});
        graph_a[e.1].push( Edge{from:e.1, to:e.0, weight:e.2});
        graph_b[e.0].push( Edge{from:e.0, to:e.1, weight:e.3});
        graph_b[e.1].push( Edge{from:e.1, to:e.0, weight:e.3});
    }



    let dist_a = dijkstra_bruteforce(&graph_a, s);
    let dist_b = dijkstra_bruteforce(&graph_b, t);
    // for i in (0..n) {
    //     println!("node {} <- {} : {} / {} <- {} : {}", i, dist_a[i].0, dist_a[i].1, i, dist_b[i].0, dist_b[i].1);
    // }

    let mut dist : Vec<(i64, usize)> = dist_a.iter()
        .zip(dist_b.iter())
        .enumerate()
        .map(|(i,(a, b))| (100000_00000_00000 - (a.1 + b.1), i))
        .collect::<Vec<(i64, usize)>>();

    dist.sort_by(|a, b|b.cmp(a));

    
    let mut index:usize = 0;
    for i in 0..n {
        while dist[index].1 < i {
            index = index + 1
        }
        println!("{}", dist[index].0);
    }


    // for i in (0..n) {
    //     println!("{} : {}", dist[i].0, dist[i].1);
    // }
    // println!("{}",0);

}

