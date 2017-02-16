use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use std::collections::vec_deque::VecDeque;


const MAX_SIZE : usize = 100000;

fn bfs(graph: &Vec<Vec<usize>>, v: usize, visited: &mut [bool; MAX_SIZE], result: &mut Vec<Vec<usize>>) {
	let mut cres = Vec::new();
	visited[v] = true;
	let mut queue: VecDeque<usize> = VecDeque::new();
	queue.push_back(v);
	cres.push(v);
	while !queue.is_empty() {
		let cv = queue.pop_front().unwrap();
		let ref t = graph[cv];
		for u in t {
			if !visited[*u] {
				cres.push(*u);
				queue.push_back(*u);
				visited[*u] = true;
			}
		}
	}
	cres.sort();
	result.push(cres);
}

fn main() {
    let mut file = File::open("input.txt").expect("opening file");
    let mut input_text = String::new();
    file.read_to_string(&mut input_text).expect("reading file");
   
    let mut lines = input_text.lines();
    let line = lines.next().unwrap();
    let graph_size = line.parse::<usize>().unwrap();
    let mut graph = Vec::new();

    for line in lines {
		let mut node = Vec::<usize>::new();
		for num in line.split(' ') {
			let cval = num.parse::<usize>().unwrap();
			if cval > 0 {
				node.push(cval - 1);
			}
		}
		graph.push(node);
	}
	
	let mut visited = [false; MAX_SIZE];
	let mut result = Vec::new();
	
	for i in 0..graph_size {
		if !visited[i as usize] {
			bfs(&graph, i as usize, &mut visited, &mut result);
		}
	}
	println!("{}", result.len());
	for res_line in result {
		for item in res_line {
			print!("{} ", item + 1);
		}
		println!("0");
	}
}
