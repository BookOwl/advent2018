extern crate regex;

static POLYMER: &str = include_str!("../../inputs/day5.txt");

fn react_polymer(poly: &str) -> String {
    let pat = regex::Regex::new(r"aA|bB|cC|dD|eE|fF|gG|hH|iI|jJ|kK|lL|mM|nN|oO|pP|qQ|rR|sS|tT|uU|vV|wW|xX|yY|zZ|Aa|Bb|Cc|Dd|Ee|Ff|Gg|Hh|Ii|Jj|Kk|Ll|Mm|Nn|Oo|Pp|Qq|Rr|Ss|Tt|Uu|Vv|Ww|Xx|Yy|Zz").unwrap();
    let mut poly = String::from(poly);
    let mut after_reaction = pat.replace_all(&poly, "").into_owned();
    while poly != after_reaction {
        poly = after_reaction;
        after_reaction = pat.replace_all(&poly, "").into_owned();
    }
    after_reaction
}

fn part1() {
    let poly = react_polymer(POLYMER);
    println!("Part 1: {}", poly.len());
}

fn part2() {
    let mut best_len = POLYMER.len() + 1;
    for (unitp, unitn) in [('a', 'A'), ('b', 'B'), ('c', 'C'), ('d', 'D'), ('e', 'E'), ('f', 'F'), 
                           ('g', 'G'), ('h', 'H'), ('i', 'I'), ('j', 'J'), ('k', 'K'), ('l', 'L'), 
                           ('m', 'M'), ('n', 'N'), ('o', 'O'), ('p', 'P'), ('q', 'Q'), ('r', 'R'), 
                           ('s', 'S'), ('t', 'T'), ('u', 'U'), ('v', 'V'), ('w', 'W'), ('x', 'X'), 
                           ('y', 'Y'), ('z', 'Z')].iter() {
        let re = regex::Regex::new(&format!("{}|{}", unitp, unitn)).unwrap();

        let poly = re.replace_all(POLYMER, "").into_owned();
        let reacted = react_polymer(&poly);
        if reacted.len() < best_len {
            best_len = reacted.len();
        }
    }
    println!("Part 2: {}", best_len);

}

fn main() {
    part1();
    part2();
}