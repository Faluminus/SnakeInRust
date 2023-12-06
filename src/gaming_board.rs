pub mod gaming_board{
    use std::collections::HashMap;
    use std::io::{self, Write};
    use rand::Rng;
    use crate::objects;


    pub struct Board{
        map:HashMap<i32,objects::objects::Objects>,
        no_walls: HashMap<i32,i32>,
        width:i32,
        height:i32,
        apple:i32,
        snake_direction:i32,
        snake_length:i32,
        snake_head:i32,
        snake_tail:i32,
        action_code:i32
    }
    impl Board{
        pub fn new() -> Board{
            Board{
                map:HashMap::new(),
                no_walls:HashMap::new(),
                apple:0,
                snake_direction:2,
                snake_length:1,
                snake_head:1,
                snake_tail:0,
                width:0,
                height:0,
                action_code:0
            }
        }
        pub fn create_board(&mut self, width:i32,height:i32,apple_preset:char,wall_preset:char,snake_preset:char ,empty_preset:char){
            self.width = width;
            self.height = height;

            //snake spawnpoint
            let spawn_point:i32 = (self.width*self.height)/2 -self.width/2;
            self.snake_head = spawn_point;
            self.snake_tail = spawn_point + self.width;

            let mut height = 1;
            let mut counter = 1;
            //map with walls
            for i in 1..self.width*self.height {
                let mut object = objects::objects::Objects::new(apple_preset, snake_preset, wall_preset, empty_preset);
                if i % self.width == 1 || i % self.width == 0{
                    object.wall();
                    self.map.insert(i, object);
                    if i % self.width == 0 {
                        height += 1;
                    }
                } else {
                    if height == 1 || height == self.height {
                        object.wall();
                        self.map.insert(i, object);
                    } else {
                        object.empty();
                        //map with no walls
                        self.no_walls.insert(counter,i);
                        counter += 1;
                        self.map.insert(i, object);
                    }
                }
            }
        }
        pub fn apple_spawn(&mut self){
            if self.apple != 0 {
                self.map.get_mut(&self.apple).unwrap().snake_body();
            }

            let mut random = rand::thread_rng();

            let location:i32= random.gen_range(1..(self.width-2) * (self.height-2));
            self.apple = *self.no_walls.get_mut(&location).unwrap();
            self.map.get_mut(&self.apple).unwrap().apple();
        }
        pub fn snake_direction(&mut self,direction_change:i32){
            if self.snake_direction != direction_change{
                self.snake_direction = direction_change;
            }
        }
        pub fn snake_move_on(&mut self){
            match self.snake_direction{
                0 => {
                    //down
                    self.map.get_mut(&self.snake_head).unwrap().direction(self.snake_direction);
                    self.snake_head += self.width;
                    self.action_code = self.map.get(&self.snake_head).unwrap().colision();
                    self.map.get_mut(&self.snake_head).unwrap().snake_body();
                }
                1 =>{
                    //left
                    self.map.get_mut(&self.snake_head).unwrap().direction(self.snake_direction);
                    self.snake_head -= 1;
                    self.action_code = self.map.get(&self.snake_head).unwrap().colision();
                    self.map.get_mut(&self.snake_head).unwrap().snake_body();
                }
                2 => {
                    //up
                    self.map.get_mut(&self.snake_head).unwrap().direction(self.snake_direction);
                    self.snake_head -= self.width;
                    self.action_code = self.map.get(&self.snake_head).unwrap().colision();
                    self.map.get_mut(&self.snake_head).unwrap().snake_body();
                }
                3 => {
                    //right
                    self.map.get_mut(&self.snake_head).unwrap().direction(self.snake_direction);
                    self.snake_head += 1;
                    self.action_code = self.map.get(&self.snake_head).unwrap().colision();
                    self.map.get_mut(&self.snake_head).unwrap().snake_body();
                }
                //nonsense check
                _ => panic!()
            }
        }
        pub fn snake_cleanup(&mut self){
            self.map.get_mut(&self.snake_tail).unwrap().empty();
            if self.action_code != 100
            {
                match self.map.get_mut(&self.snake_tail).unwrap().return_direction() {
                    0 => {
                        self.snake_tail += self.width;
                    }
                    1 => {
                        self.snake_tail -= 1;
                    }
                    2 => {
                        self.snake_tail -= self.width;
                    }
                    3 => {
                        self.snake_tail += 1;
                    }
                    _ => panic!()
                }
            }
        }
        pub fn asses_action(&mut self) -> i32{
            match self.action_code{
                101 => {
                    io::stdout().flush().unwrap();
                    println!("You died");
                    101
                }
                100 =>{
                    self.snake_length += 1;
                    100
                }
                0 => {
                    0
                }
                _ => {
                    2000
                }
            }
        }
        pub fn reset(&self){
            print!("\x1B[2J");
            io::stdout().flush().unwrap();
            for i in 1..self.width * self.height{
                self.map.get(&i).unwrap().write_on_console();
                if i % self.width == 0{
                    println!()
                }
            }
        }
    }
}