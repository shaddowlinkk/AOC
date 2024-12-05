    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    fn main() {   
        let f = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
	let data:Vec<Vec<i32>> = file.lines().map(|a| a.unwrap().split(" ").map(|a| a.collect::<Vec<&str>>().map(|a| a.parse().unwrap())).collect::<Vec<i32>>()).collect();
	println!("{:?}",data);
    }

