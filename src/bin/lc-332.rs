use std::collections::{BTreeMap, BTreeSet, HashMap};

struct Solution;

impl Solution {
    // I didn't account for cases where there are multiple trips
    // glad I do the adjacency matrix approach! Likely not efficient tnough though.j
    pub fn find_itinerary_1(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut adj_list: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for ticket in tickets {
            let i = ticket[0].clone();
            let j = ticket[1].clone();
            adj_list
                .entry(i)
                .and_modify(|e| e.push(j.clone()))
                .or_insert(vec![j.clone()]);
            adj_list.entry(j.clone()).or_insert(vec![]);
        }
        let n = adj_list.len();
        let idx: HashMap<usize, &String> = adj_list.keys().enumerate().collect();
        let rev_idx: HashMap<String, usize> = idx.iter().map(|(i, x)| ((*x).clone(), *i)).collect();

        let mut adj_matrix = vec![vec![0; n]; n];
        let mut n_trips = 0;
        for (k, vs) in adj_list.iter() {
            let i = *rev_idx.get(k).unwrap();
            for v in vs {
                let j = *rev_idx.get(v).unwrap();
                adj_matrix[i][j] += 1;
                n_trips += 1;
            }
        }

        // DFS
        fn dfs(
            adj_matrix: &Vec<Vec<usize>>,
            f: &mut Vec<(usize, usize)>,
            counter: &mut HashMap<(usize, usize), usize>,
            n_trips: usize,
            i: usize,
            j: usize,
        ) -> Option<Vec<usize>> {
            let used = match counter.get(&(i, j)) {
                Some(v) => *v,
                None => 0,
            };
            if adj_matrix[i][j] == 0 || used >= adj_matrix[i][j] {
                None
            } else {
                f.push((i, j));
                counter.entry((i, j)).and_modify(|e| *e += 1).or_insert(1);
                if f.len() == n_trips {
                    let mut route = vec![f[0].0];
                    for _f in f.iter() {
                        route.push(_f.1);
                    }
                    return Some(route);
                }
                for k in 0..adj_matrix.len() {
                    let route = dfs(adj_matrix, f, counter, n_trips, j, k);
                    if route.is_some() {
                        return route;
                    }
                }
                f.pop();
                counter.entry((i, j)).and_modify(|e| *e -= 1);
                None
            }
        }

        let mut f = Vec::new();
        // NOTE: After reading Eulerian path (discussion @ leetcode):
        // I realize if I choose greedily by lowest lex ordering, I don't need
        // to find every possible path!
        // Lesson: read the problem properly; what being asked and what consequence of the chosen path
        let i = *rev_idx.get("JFK").unwrap();
        for j in 0..n {
            if adj_matrix[i][j] > 0 {
                let yea = dfs(&adj_matrix, &mut f, &mut HashMap::new(), n_trips, i, j);
                if let Some(y) = yea {
                    return y
                        .iter()
                        .map(|x| idx.get(x).unwrap().clone().clone())
                        .collect();
                }
            }
        }
        unreachable!()
    }

    // neetcode (inspired): technically it is possible to make this much simpler really
    // unfortunately doesn't work!
    pub fn find_itinerary_2(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut adj_list: HashMap<String, BTreeSet<String>> = HashMap::new();
        for ticket in tickets {
            adj_list
                .entry(ticket[0].clone())
                .or_insert_with(BTreeSet::new)
                .insert(ticket[1].clone());
        }
        println!("========");
        println!("{adj_list:?}");
        println!("========");

        let mut f = vec!["JFK".to_string()];
        let mut fr = vec![];
        while let Some(source) = f.last() {
            if let Some(dests) = adj_list.get_mut(source) {
                // Pop and no put back: after all it is already used!
                // And since we are following lex ordering, the first found path
                // is the correct one.
                if !dests.is_empty() {
                    f.push(dests.pop_first().unwrap());
                    continue;
                }
            }
            // how to backtrack tho? clever: if no dests then it is definitely last point!
            // Then if everything is used it will fallback all the way
            if let Some(e) = f.pop() {
                fr.push(e);
            }
        }
        fr.reverse();
        fr
    }
}

fn main() {
    for f in [Solution::find_itinerary_1, Solution::find_itinerary_2] {
        assert_eq!(
            f(vec![
                vec!["MUC".to_string(), "LHR".to_string()],
                vec!["JFK".to_string(), "MUC".to_string()],
                vec!["SFO".to_string(), "SJC".to_string()],
                vec!["LHR".to_string(), "SFO".to_string()]
            ]),
            vec![
                "JFK".to_string(),
                "MUC".to_string(),
                "LHR".to_string(),
                "SFO".to_string(),
                "SJC".to_string()
            ]
        );
        assert_eq!(
            f(vec![
                vec!["JFK".to_string(), "SFO".to_string()],
                vec!["JFK".to_string(), "ATL".to_string()],
                vec!["SFO".to_string(), "ATL".to_string()],
                vec!["ATL".to_string(), "JFK".to_string()],
                vec!["ATL".to_string(), "SFO".to_string()]
            ]),
            vec![
                "JFK".to_string(),
                "ATL".to_string(),
                "JFK".to_string(),
                "SFO".to_string(),
                "ATL".to_string(),
                "SFO".to_string()
            ]
        );
        assert_eq!(
            f(vec![vec!["JFK".to_string(), "ATL".to_string()]]),
            vec!["JFK".to_string(), "ATL".to_string()]
        );
        assert_eq!(
            f(vec![
                vec!["JFK".to_string(), "AAA".to_string()],
                vec!["AAA".to_string(), "JFK".to_string()],
                vec!["JFK".to_string(), "BBB".to_string()],
                vec!["JFK".to_string(), "CCC".to_string()],
                vec!["CCC".to_string(), "JFK".to_string()]
            ]),
            vec!["JFK", "AAA", "JFK", "CCC", "JFK", "BBB"]
        );
        println!(
            "{:?}",
            f(vec![
                vec!["EZE".to_string(), "AXA".to_string()],
                vec!["TIA".to_string(), "ANU".to_string()],
                vec!["ANU".to_string(), "JFK".to_string()],
                vec!["JFK".to_string(), "ANU".to_string()],
                vec!["ANU".to_string(), "EZE".to_string()],
                vec!["TIA".to_string(), "ANU".to_string()],
                vec!["AXA".to_string(), "TIA".to_string()],
                vec!["TIA".to_string(), "JFK".to_string()],
                vec!["ANU".to_string(), "TIA".to_string()],
                vec!["JFK".to_string(), "TIA".to_string()]
            ])
        );
        println!(
            "{:?}",
            f(vec![
                vec!["AXA".to_string(), "EZE".to_string()],
                vec!["EZE".to_string(), "AUA".to_string()],
                vec!["ADL".to_string(), "JFK".to_string()],
                vec!["ADL".to_string(), "TIA".to_string()],
                vec!["AUA".to_string(), "AXA".to_string()],
                vec!["EZE".to_string(), "TIA".to_string()],
                vec!["EZE".to_string(), "TIA".to_string()],
                vec!["AXA".to_string(), "EZE".to_string()],
                vec!["EZE".to_string(), "ADL".to_string()],
                vec!["ANU".to_string(), "EZE".to_string()],
                vec!["TIA".to_string(), "EZE".to_string()],
                vec!["JFK".to_string(), "ADL".to_string()],
                vec!["AUA".to_string(), "JFK".to_string()],
                vec!["JFK".to_string(), "EZE".to_string()],
                vec!["EZE".to_string(), "ANU".to_string()],
                vec!["ADL".to_string(), "AUA".to_string()],
                vec!["ANU".to_string(), "AXA".to_string()],
                vec!["AXA".to_string(), "ADL".to_string()],
                vec!["AUA".to_string(), "JFK".to_string()],
                vec!["EZE".to_string(), "ADL".to_string()],
                vec!["ANU".to_string(), "TIA".to_string()],
                vec!["AUA".to_string(), "JFK".to_string()],
                vec!["TIA".to_string(), "JFK".to_string()],
                vec!["EZE".to_string(), "AUA".to_string()],
                vec!["AXA".to_string(), "EZE".to_string()],
                vec!["AUA".to_string(), "ANU".to_string()],
                vec!["ADL".to_string(), "AXA".to_string()],
                vec!["EZE".to_string(), "ADL".to_string()],
                vec!["AUA".to_string(), "ANU".to_string()],
                vec!["AXA".to_string(), "EZE".to_string()],
                vec!["TIA".to_string(), "AUA".to_string()],
                vec!["AXA".to_string(), "EZE".to_string()],
                vec!["AUA".to_string(), "SYD".to_string()],
                vec!["ADL".to_string(), "JFK".to_string()],
                vec!["EZE".to_string(), "AUA".to_string()],
                vec!["ADL".to_string(), "ANU".to_string()],
                vec!["AUA".to_string(), "TIA".to_string()],
                vec!["ADL".to_string(), "EZE".to_string()],
                vec!["TIA".to_string(), "JFK".to_string()],
                vec!["AXA".to_string(), "ANU".to_string()],
                vec!["JFK".to_string(), "AXA".to_string()],
                vec!["JFK".to_string(), "ADL".to_string()],
                vec!["ADL".to_string(), "EZE".to_string()],
                vec!["AXA".to_string(), "TIA".to_string()],
                vec!["JFK".to_string(), "AUA".to_string()],
                vec!["ADL".to_string(), "EZE".to_string()],
                vec!["JFK".to_string(), "ADL".to_string()],
                vec!["ADL".to_string(), "AXA".to_string()],
                vec!["TIA".to_string(), "AUA".to_string()],
                vec!["AXA".to_string(), "JFK".to_string()],
                vec!["ADL".to_string(), "AUA".to_string()],
                vec!["TIA".to_string(), "JFK".to_string()],
                vec!["JFK".to_string(), "ADL".to_string()],
                vec!["JFK".to_string(), "ADL".to_string()],
                vec!["ANU".to_string(), "AXA".to_string()],
                vec!["TIA".to_string(), "AXA".to_string()],
                vec!["EZE".to_string(), "JFK".to_string()],
                vec!["EZE".to_string(), "AXA".to_string()],
                vec!["ADL".to_string(), "TIA".to_string()],
                vec!["JFK".to_string(), "AUA".to_string()],
                vec!["TIA".to_string(), "EZE".to_string()],
                vec!["EZE".to_string(), "ADL".to_string()],
                vec!["JFK".to_string(), "ANU".to_string()],
                vec!["TIA".to_string(), "AUA".to_string()],
                vec!["EZE".to_string(), "ADL".to_string()],
                vec!["ADL".to_string(), "JFK".to_string()],
                vec!["ANU".to_string(), "AXA".to_string()],
                vec!["AUA".to_string(), "AXA".to_string()],
                vec!["ANU".to_string(), "EZE".to_string()],
                vec!["ADL".to_string(), "AXA".to_string()],
                vec!["ANU".to_string(), "AXA".to_string()],
                vec!["TIA".to_string(), "ADL".to_string()],
                vec!["JFK".to_string(), "ADL".to_string()],
                vec!["JFK".to_string(), "TIA".to_string()],
                vec!["AUA".to_string(), "ADL".to_string()],
                vec!["AUA".to_string(), "TIA".to_string()],
                vec!["TIA".to_string(), "JFK".to_string()],
                vec!["EZE".to_string(), "JFK".to_string()],
                vec!["AUA".to_string(), "ADL".to_string()],
                vec!["ADL".to_string(), "AUA".to_string()],
                vec!["EZE".to_string(), "ANU".to_string()],
                vec!["ADL".to_string(), "ANU".to_string()],
                vec!["AUA".to_string(), "AXA".to_string()],
                vec!["AXA".to_string(), "TIA".to_string()],
                vec!["AXA".to_string(), "TIA".to_string()],
                vec!["ADL".to_string(), "AXA".to_string()],
                vec!["EZE".to_string(), "AXA".to_string()],
                vec!["AXA".to_string(), "JFK".to_string()],
                vec!["JFK".to_string(), "AUA".to_string()],
                vec!["ANU".to_string(), "ADL".to_string()],
                vec!["AXA".to_string(), "TIA".to_string()],
                vec!["ANU".to_string(), "AUA".to_string()],
                vec!["JFK".to_string(), "EZE".to_string()],
                vec!["AXA".to_string(), "ADL".to_string()],
                vec!["TIA".to_string(), "EZE".to_string()],
                vec!["JFK".to_string(), "AXA".to_string()],
                vec!["AXA".to_string(), "ADL".to_string()],
                vec!["EZE".to_string(), "AUA".to_string()],
                vec!["AXA".to_string(), "ANU".to_string()],
                vec!["ADL".to_string(), "EZE".to_string()],
                vec!["AUA".to_string(), "EZE".to_string()]
            ])
        )
    }
}
