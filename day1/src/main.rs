    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;

    fn main() {   
        let f = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
	let mut first: Vec<usize> = Vec::new();
	let mut sec: Vec<usize> = Vec::new();
        for (_num, line) in file.lines().enumerate() {
            let l = line.unwrap();
	    let test: Vec<&str> = l.split(" ").filter(|&s| !s.is_empty()).collect();
	    if test.len()>1{
		first.push(test[0].parse().unwrap());
		sec.push(test[1].parse().unwrap());
            }
        }
	first.sort();
	sec.sort();
	let out = first.into_iter().map(|a| a * sec.iter().filter(|&n|*n==a).count() ).sum::<usize>();
	println!("out:{}",out)
    }

