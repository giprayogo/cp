use std::collections::{BTreeMap, HashMap};

struct Solution;

impl Solution {
    // I didn't account for cases where there are multiple trips
    // glad I do the adjacency matrix approach! Likely not efficient tnough though.j
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
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
            itineraries: &mut Vec<Vec<usize>>,
            counter: &mut HashMap<(usize, usize), usize>,
            n_trips: usize,
            i: usize,
            j: usize,
        ) {
            let used = match counter.get(&(i, j)) {
                Some(v) => *v,
                None => 0,
            };
            if adj_matrix[i][j] == 0 || used >= adj_matrix[i][j] {
            } else {
                // println!("{i},{j}");
                f.push((i, j));
                counter.entry((i, j)).and_modify(|e| *e += 1).or_insert(1);
                if f.len() == n_trips {
                    let mut it = vec![f[0].0];
                    for _f in f.iter() {
                        it.push(_f.1);
                    }
                    itineraries.push(it);
                }
                for k in 0..adj_matrix.len() {
                    dfs(adj_matrix, f, itineraries, counter, n_trips, j, k);
                }
                f.pop();
                counter.entry((i, j)).and_modify(|e| *e -= 1);
            }
        }

        let mut f = Vec::new();
        let mut itineraries = Vec::new();
        // TODO: small opt: can just use the number index
        for v in adj_list.get("JFK").unwrap() {
            let i = *rev_idx.get("JFK").unwrap();
            let j = *rev_idx.get(v).unwrap();
            dfs(
                &adj_matrix,
                &mut f,
                &mut itineraries,
                &mut HashMap::new(),
                n_trips,
                i,
                j,
            );
        }

        itineraries.sort();
        itineraries[0]
            .iter()
            .map(|x| idx.get(x).unwrap().clone().clone())
            .collect()
    }
}

fn main() {
    // assert_eq!(
    //     Solution::find_itinerary(vec![
    //         vec!["MUC".to_string(), "LHR".to_string()],
    //         vec!["JFK".to_string(), "MUC".to_string()],
    //         vec!["SFO".to_string(), "SJC".to_string()],
    //         vec!["LHR".to_string(), "SFO".to_string()]
    //     ]),
    //     vec![
    //         "JFK".to_string(),
    //         "MUC".to_string(),
    //         "LHR".to_string(),
    //         "SFO".to_string(),
    //         "SJC".to_string()
    //     ]
    // );
    // assert_eq!(
    //     Solution::find_itinerary(vec![
    //         vec!["JFK".to_string(), "SFO".to_string()],
    //         vec!["JFK".to_string(), "ATL".to_string()],
    //         vec!["SFO".to_string(), "ATL".to_string()],
    //         vec!["ATL".to_string(), "JFK".to_string()],
    //         vec!["ATL".to_string(), "SFO".to_string()]
    //     ]),
    //     vec![
    //         "JFK".to_string(),
    //         "ATL".to_string(),
    //         "JFK".to_string(),
    //         "SFO".to_string(),
    //         "ATL".to_string(),
    //         "SFO".to_string()
    //     ]
    // );
    // assert_eq!(
    //     Solution::find_itinerary(vec![vec!["JFK".to_string(), "ATL".to_string()]]),
    //     vec!["JFK".to_string(), "ATL".to_string()]
    // );
    // assert_eq!(
    //     Solution::find_itinerary(vec![
    //         vec!["JFK".to_string(), "AAA".to_string()],
    //         vec!["AAA".to_string(), "JFK".to_string()],
    //         vec!["JFK".to_string(), "BBB".to_string()],
    //         vec!["JFK".to_string(), "CCC".to_string()],
    //         vec!["CCC".to_string(), "JFK".to_string()]
    //     ]),
    //     vec!["JFK", "AAA", "JFK", "CCC", "JFK", "BBB"]
    // );
    // println!(
    //     "{:?}",
    //     Solution::find_itinerary(vec![
    //         vec!["EZE".to_string(), "AXA".to_string()],
    //         vec!["TIA".to_string(), "ANU".to_string()],
    //         vec!["ANU".to_string(), "JFK".to_string()],
    //         vec!["JFK".to_string(), "ANU".to_string()],
    //         vec!["ANU".to_string(), "EZE".to_string()],
    //         vec!["TIA".to_string(), "ANU".to_string()],
    //         vec!["AXA".to_string(), "TIA".to_string()],
    //         vec!["TIA".to_string(), "JFK".to_string()],
    //         vec!["ANU".to_string(), "TIA".to_string()],
    //         vec!["JFK".to_string(), "TIA".to_string()]
    //     ])
    // )q;
    println!(
        "{:?}",
        Solution::find_itinerary(vec![
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
