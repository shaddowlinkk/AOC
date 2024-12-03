    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    fn check()->(i32,i32){
    }
    fn main() {   
        let f = File::open("test.txt").unwrap();
        let file = BufReader::new(&f);
	//let mut first: Vec<usize> = Vec::new();
        for (_num, line) in file.lines().enumerate() {
            let cha:Vec<char> = line.unwrap().chars().collect();
	    let mut key:Vec<char>=vec![];
	    println!("{:?}",cha);
	    for  i in 0..cha.len(){
		match key.iter().collect::<String>().as_str(){
		"mul" => println!("found"),
		_ => {
			println!("{:?}",key);
			if key.len()>=3{
				key.remove(0);
			}
			key.push(cha[i]);
		     }
		}
		println!("{}",key.iter().collect::<String>());		
	    }
        }
	//first.sort();
	//sec.sort();
	//let out = first.into_iter().map(|a| a * sec.iter().filter(|&n|*n==a).count() ).sum::<usize>();
	//println!("out:{}",out)
    }

