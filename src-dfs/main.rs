use std::io::Read;


#[derive(Copy, Clone)]
struct Edge{
    from: usize,
    to: usize,
    weight: i64,
}

fn is_partite (graph: Vec<Vec<Edge>>) -> (bool,usize,usize) {
    let nodes = graph.len();
    let mut stack: Vec<Edge> = Vec::new();
    let mut visited: Vec<bool> = vec![false; nodes];
    let mut color: Vec<i32> = vec![-1; nodes];

    stack.push(Edge{from:0, to:0, weight:0});
    color[0] = 2;
    let mut is_partite : bool = true;
    
    // dfs
    while let Some(e) = stack.pop()  {
        if visited[e.to] {
            continue;
        }
        visited[e.to] = true;
        color[e.to] = 3 - color[e.from];
        for i in graph[e.to].iter() {
            if ! visited[i.to] {
                stack.push(*i);
            } else if color[i.from] == color[i.to] {
                is_partite = false; // if adjacent node's color is different, given graph is not bipartite.
            }
        }
    }
    let whites = color.iter().filter(|&x| *x == 1).collect::<Vec<&i32>>().len();

    (is_partite, whites, nodes - whites)
}


fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    // get stdio    
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    
    let edges: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect();


    // construct graph
    let mut graph: Vec<Vec<Edge>> = (0..n)
        .map(|_| {
            Vec::new()
        }).collect();

    for e in edges.iter() {
        graph[e.0].push( Edge{from:e.0, to:e.1, weight:1});
        graph[e.1].push( Edge{from:e.1, to:e.0, weight:1})
    }

    let re = is_partite(graph);

    if re.0 {
        println!("{}",  re.1 * re.2 - m);
    } else {
        println!("{}", n * ( n - 1 ) / 2 - m);
    }

}
