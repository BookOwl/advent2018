extern crate regex;
use std::collections::HashSet;
static CLAIMS: &str = include_str!("../../inputs/day3.txt");


#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}
impl Claim {
    fn from_str(claim: &str, re: &regex::Regex) -> Claim {
        let caps = re.captures(claim).unwrap();
        let id = caps.get(1).unwrap().as_str().parse().unwrap();
        let left = caps.get(2).unwrap().as_str().parse().unwrap();
        let top = caps.get(3).unwrap().as_str().parse().unwrap();
        let width = caps.get(4).unwrap().as_str().parse().unwrap();
        let height = caps.get(5).unwrap().as_str().parse().unwrap();
        Claim {
            id,
            left,
            top,
            width,
            height,
        }
    }
    fn mark_on_fabric(&self, fabric: &mut [[usize; 1000]; 1000]) -> HashSet<usize> {
        let mut overlaps = HashSet::new();
        let mut i_overlapped = false;
        for x in self.left..(self.left+self.width) {
            for y in self.top..(self.top+self.height) {
                if fabric[x][y] != 0 {
                    overlaps.insert(fabric[x][y]);
                    i_overlapped = true;
                }
                fabric[x][y] = self.id;
            }
        }
        if i_overlapped {
            overlaps.insert(self.id);
        }
        overlaps
    }
}

fn main() {
    let re = regex::Regex::new(r#"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"#).unwrap();
    let mut fabric = [[0; 1000]; 1000];
    let mut overlaps = HashSet::new();
    let claims: Vec<_> = CLAIMS.split("\n").map(|c| Claim::from_str(c, &re)).collect();
    let all_ids: HashSet<_> = claims.iter().map(|c| c.id).collect();
    for claim in claims {
        let claim_overlapped = claim.mark_on_fabric(&mut fabric);
        overlaps = overlaps.union(&claim_overlapped).cloned().collect();
    }
    let did_not_overlap: Vec<_> = all_ids.difference(&overlaps).collect();
    println!("{:?}", did_not_overlap);
    
}