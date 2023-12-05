pub mod objects{
    pub struct Objects{
        apple:bool,
        snake:bool,
        wall:bool,
        empty:bool,
        direction:i32,
        apple_preset:char,
        snake_preset:char,
        wall_preset:char,
        empty_preset:char
    }
    impl Objects{
        pub fn new(apple_preset:char,snake_preset:char,wall_preset:char,empty_preset:char) -> Objects{
            Objects {
                apple: false,
                snake: false,
                wall: false,
                empty: false,
                direction: 2,
                apple_preset,
                snake_preset,
                wall_preset,
                empty_preset
            }
        }
        pub fn apple(&mut self){
            self.wall = false;
            self.apple = true;
            self.snake = false;
            self.empty = false;
        }
        pub fn wall(&mut self){
            self.wall = true;
            self.apple = false;
            self.snake = false;
            self.empty = false;
        }
        pub fn empty(&mut self){
            self.wall = false;
            self.apple = false;
            self.snake = false;
            self.empty = true;
        }
        pub fn snake_body(&mut self){
            self.wall = false;
            self.apple = false;
            self.snake = true;
            self.empty = false;
        }
        pub fn direction(&mut self,direction_code:i32){
            self.direction = direction_code;
        }
        pub fn return_direction(&self) -> i32{
            self.direction
        }
        pub fn colision(&self) -> i32{
            let death_code:i32 = 101;
            let growth_code:i32 = 100;
            let continue_code:i32 = 0;
            if self.snake ==true || self.wall == true{
                death_code
            }
            else if self.apple {
                growth_code
            }else{
                continue_code
            }
        }
        pub fn write_on_console(&self){
            if self.empty == true{
                print!("{}",self.empty_preset);
            }else if self.apple == true{
                print!("{}",self.apple_preset);
            }else if self.snake == true{
                print!("{}",self.snake_preset);
            }else{
                print!("{}",self.wall_preset);
            }
        }
    }
}