    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    struct GameState{
	board: Vec<Vec<char>>,
	y: i32,
	x: i32,
	dx: i32,
	dy: i32,
    }
    impl GameState{
	fn print_board(&mut self){
		for line in self.board.clone(){
			for cha in line{
				print!("{}",cha);
			}
			println!()	
		}
	}
	fn move_char(&mut self)-> bool{	
		let ny=self.y+self.dy;
		let nx=self.x+self.dx;
		if self.board.clone()[ny as usize ][nx as usize] != '#'{
			self.board[ny as usize][nx as usize] = '^';
			self.board[self.y as usize ][self.x as usize] = 'X';
			self.y=ny;
			self.x=nx;
			return true
		}	
		return false
	}
	fn rotate(&mut self){
		let tmp =self.dy;
		self.dy=self.dx;
		self.dx=tmp*-1
	}
	fn simulate(&mut self){
		let y_max: i32 = self.board.len().try_into().unwrap();
		let x_max: i32 = self.board[0].len().try_into().unwrap();
		while self.x < x_max-1 && self.y < y_max-1{
			//println!("pos:{},{}  dir:{},{}",self.y,self.x,self.dy,self.dx);
			//self.print_board();
			//println!();
			if !self.move_char(){
				println!("bump");
				self.rotate();
			}else{

			}
		}
	}	
    	fn find_start(&mut self){//in yx  form
		for (y,line) in self.board.clone().into_iter().enumerate(){
			for (x,cha) in line.into_iter().enumerate(){
				match cha {
				'^' => {
					self.y=y.try_into().unwrap();
					self.x=x.try_into().unwrap();
					return;
				       },
				_ =>()
				}
			}
		}
	}
    }
    fn main() {   
        let f = File::open("input.txt").unwrap();
        let file = BufReader::new(&f);
	let mut state = GameState{y: -1,x:-1,board: vec![],dx: 0,dy: -1}; 
	state.board  = file.lines().map(|a| a.unwrap().chars().collect::<Vec<char>>()).collect();
	state.find_start();
	state.simulate();
	state.print_board();
	let out: usize = state.board.clone().into_iter().map(|a| a.into_iter().filter(|a| *a=='X' || *a=='^').count()).sum();
	println!("out:{}",out)
    }

