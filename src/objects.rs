pub mod objects{
    pub struct Objects{
        apple:bool,
        snake:bool,
        wall:bool,
        empty:bool,
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