pub mod objects{
    pub struct Objects{
        apple:bool,
        snake:bool,
        wall:bool,
        apple_preset:char,
        snake_preset:char,
        wall_preset:char
    }
    impl Objects{
        pub fn new(apple_preset:char,snake_preset:char,wall_preset:char) -> Objects{
            Objects {
                apple: false,
                snake: false,
                wall: false,
                apple_preset,
                snake_preset,
                wall_preset
            }
        }
        pub fn apple(&mut self){
            self.apple = true;
        }
        pub fn wall(&mut self){
            self.wall = true;
        }
        pub fn snake_body(&mut self){
            self.snake = true;
        }
        pub fn write_on_console(self){
            if self.apple == true{
                print!("{}",self.apple_preset);
            }else if self.snake == true{
                print!("{}",self.snake_preset);
            }else{
                print!("{}",self.wall_preset);
            }
        }
    }
}