    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
 fn swap(data: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<i32>{
        let mut out = data.clone();
	let mut i = 0;
        while i<rules.len(){
                let (p1,p2):(usize,usize);
		//println!("{}",i);
                match out.iter().position(|&p| p == rules[i][0]){
                Some(x)=>{
                        p1=x
                }
                None => {i+=1;continue}
                }
                match out.iter().position(|&p| p == rules[i][1]){
                Some(x)=>{
                        p2=x
                }
                None => {i+=1;continue}
                }
		//println!("p2:{},p1:{}",p2,p1);
                if !(p1<p2){
		    //println!("p2:{},p1:{}",p2,p1);
		    //println!("rule:{:?},o2:{},o1:{}",rules[i],out[p2],out[p1]);
                    let temp = out[p2];
                    out[p2]=out[p1];
                    out[p1]=temp;
		    i=0;
		    //println!("swaped:{:?}",out)
                }else{
		i=i+1;
		}
        }
        return out;
    } 
    
    fn rulecheck(data: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool{
	for rule in rules{
		if !data.iter().any(|&r| r== rule[0]){
			continue
		}
		match data.iter().position(|&r| r == rule[1]){
		Some(x)=>{
			let mut found=false;
			for num in &data[0..x] {
				if *num == rule[0]{
					found=true
				}
			}
			if !found {
				println!("{:?},{:?}",rule,data);	
				return false;
			}
		},
		None => continue
		}
	}
	return true;
    }
    fn main() {   
        let f = File::open("rules.txt").unwrap();
        let f2 = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
        let file2 = BufReader::new(&f2);
	let rules= file.lines().map(|a| a.unwrap().split("|").map(|a| a.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
	let data = file2.lines().map(|a| a.unwrap().split(",").map(|a| a.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
	let data2 = data.into_iter().filter(|a| !rulecheck(a,&rules)).collect::<Vec<Vec<i32>>>();
	let out =  data2.into_iter().map(|a| swap(&a,&rules)).fold(0,|acc,ve| ve[ve.len()/2]+acc);
	println!("data:{:?}",out)
    }

        
