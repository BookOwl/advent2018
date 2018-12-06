static POINTS: &str = include_str!("../../inputs/day6.txt");
use std::collections::{HashMap, HashSet};

// Calculates Taxicab distance https://en.wikipedia.org/wiki/Taxicab_geometry
fn dist(x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
    (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()
}

fn part1() {
    let points: Vec<_> = POINTS.lines()
                                .map(|l| l.split(", ")
                                           .map(|x| x.parse::<usize>().unwrap())
                                           .collect::<Vec<_>>())
                                .enumerate()
                                .map(|(i, p)| (i+1, p[0], p[1]))
                                .collect();
    // CAUTION! Horrible hacks ahead
    let mut grid = vec![vec![0; 400]; 400];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // This code is 💩
            let mut best_dist = 9999;
            let mut closest = 0;
            let mut equidist = false;
            for (id, x, y) in points.iter() {
                let d = dist(i, j, *x, *y);
                if d < best_dist {
                    best_dist = d;
                    closest = *id;
                    equidist = false;
                } else if d == best_dist {
                    equidist = true;
                }
            }
            if !equidist {
                grid[i][j] = closest;
            }
        }
    }
    let mut closest_counts: HashMap<usize, _> = HashMap::new();
    let mut bad_ids: HashSet<_> = [0].iter().collect();
    for (i, row) in grid.iter().enumerate() {
        for (j, id) in row.iter().enumerate() {
            //println!("{}, {}", i, j);
            if i == 0 || i == grid.len()-1 || j == 0 || j == grid[0].len()-1 {
                bad_ids.insert(id);
            }
            *closest_counts.entry(*id).or_insert(0) += 1;
        }
    }
    for bad_id in bad_ids.iter() {
        let bad_id = *bad_id;
        closest_counts.remove(&bad_id);
    }
    println!("Part 1: {}", closest_counts.values().max().unwrap());
}

fn part2() {
    const MAX_DIST: i32= 10000;
    let points: Vec<_> = POINTS.lines()
                                .map(|l| l.split(", ")
                                           .map(|x| x.parse::<usize>().unwrap())
                                           .collect::<Vec<_>>())
                                .enumerate()
                                .map(|(i, p)| (i+1, p[0], p[1]))
                                .collect();
    let mut grid = vec![vec![0; 400]; 400];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let dist_sum: i32 = points.iter().map(|(_, x, y)| dist(i, j, *x, *y)).sum();
            if dist_sum < MAX_DIST {
                grid[i][j] = 1;
            }
        }
    }
    let s: i32 = grid.iter().map(|row| row.iter().sum()).collect::<Vec<i32>>().iter().sum();
    println!("Part 2: {}", s)
}

fn main() {
    part1();
    part2();
}