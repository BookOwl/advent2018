extern crate regex;
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
    fn mark_on_fabric(&self, fabric: &mut [[usize; 1000]; 1000]) {
        for x in self.left..(self.left+self.width) {
            for y in self.top..(self.top+self.height) {
                fabric[x][y] += 1;
            }
        }
    }
}

fn count_overlaps(fabric: &[[usize; 1000]; 1000]) -> usize {
    let mut count = 0;
    for row in fabric.iter() {
        for item in row.iter() {
            if *item > 1 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let re = regex::Regex::new(r#"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"#).unwrap();
    let mut fabric = [[0; 1000]; 1000];
    let claims: Vec<_> = CLAIMS.split("\n").map(|c| Claim::from_str(c, &re)).collect();
    for claim in claims {
        claim.mark_on_fabric(&mut fabric);
    }
    let count = count_overlaps(&fabric);
    println!("{}", count)
    
}