

fn main() {
    let v = (1..)
                .filter(
    |&x| (1..11)
                .map(|y| x % y)
                .all(|z| z == 0))
                .take(1000)
                .collect::<Vec<u32>>();
                
    for item in &v {
        println!("{}", item);
    }
}
