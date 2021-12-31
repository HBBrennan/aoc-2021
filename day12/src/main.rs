use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

impl FromStr for Node {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Node::Start,
            "end" => Node::End,
            _ => {
                if s.chars().all(|c| {
                    [
                        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
                        'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                    ]
                    .contains(&c)
                }) {
                    Node::Small(s.to_string())
                } else {
                    Node::Big(s.to_string())
                }
            }
        })
    }
}

struct CaveGraph {
    edgelists: HashMap<Node, Vec<Node>>,
}

impl CaveGraph {
    fn get_neighbors(&self, node: Node) -> &Vec<Node> {
        &self.edgelists[&node]
    }

    fn find_all_paths(&self, small_cave_visit_twice: usize) -> Vec<Vec<Node>> {
        self.find_all_paths_helper(
            Vec::from([Node::Start]),
            HashSet::from([Node::Start]),
            small_cave_visit_twice,
        )
    }

    fn find_all_paths_helper(
        &self,
        path_so_far: Vec<Node>,
        visited_smol: HashSet<Node>,
        small_cave_twice: usize,
    ) -> Vec<Vec<Node>> {
        let mut ret: Vec<Vec<Node>> = Vec::new();
        for neighbor in self.get_neighbors(path_so_far.last().unwrap().clone()) {
            if let Node::Big(_) = neighbor {
                let mut new_path = path_so_far.clone();
                new_path.push(neighbor.clone());
                ret.extend(self.find_all_paths_helper(
                    new_path,
                    visited_smol.clone(),
                    small_cave_twice,
                ));
            } else if let Node::Small(_) = neighbor {
                if !visited_smol.contains(neighbor) || small_cave_twice > 0 {
                    let mut new_path = path_so_far.clone();
                    new_path.push(neighbor.clone());
                    let mut new_visited_smol = visited_smol.clone();
                    new_visited_smol.insert(neighbor.clone());
                    ret.extend(self.find_all_paths_helper(
                        new_path,
                        new_visited_smol,
                        if visited_smol.contains(neighbor) {
                            small_cave_twice - 1
                        } else {
                            small_cave_twice
                        },
                    ));
                }
            } else if let Node::End = neighbor {
                let mut new_path = path_so_far.clone();
                new_path.push(neighbor.clone());
                ret.push(new_path);
            }
        }
        ret
    }
}

impl FromStr for CaveGraph {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edgelists: HashMap<Node, Vec<Node>> = HashMap::new();
        for line in s.split('\n') {
            let nodes = line
                .split_once('-')
                .map(|e| {
                    (
                        Node::from_str(e.0).expect("Failed to parse node"),
                        Node::from_str(e.1).expect("Failed to parse node"),
                    )
                })
                .expect("failed to parse line");
            edgelists
                .entry(nodes.0.clone())
                .or_insert(Vec::new())
                .push(nodes.1.clone());
            edgelists.entry(nodes.1).or_insert(Vec::new()).push(nodes.0);
        }

        Ok(CaveGraph { edgelists })
    }
}

fn part1(cave: &CaveGraph) -> usize {
    cave.find_all_paths(0).len()
}

fn part2(cave: &CaveGraph) -> usize {
    cave.find_all_paths(1).len()
}

fn main() {
    let input = CaveGraph::from_str(
        read_to_string("input")
            .expect("failed to read input")
            .trim(),
    )
    .expect("Failed to parse input into graph");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let input = CaveGraph::from_str(input).expect("failed to parse cave graph");
        assert_eq!(part1(&input), 226);
        assert_eq!(part2(&input), 3509);
    }
}