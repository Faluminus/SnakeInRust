pub mod objects{
    pub struct Objects{
        apple:bool,
        snake:bool,
        wall:bool
    }
    impl Objects{
        pub fn new(apple_preset:char,snake_preset:char,wall_preset:char) -> Objects{
            Objects {
                apple: false,
                snake: false,
                wall: false,
            }
        }
        fn apple(&mut self){
            self.apple = true;
        }
        fn wall(&mut self){
            self.wall = true;
        }
        fn snake_body(&mut self){
            self.snake = true;
        }
        pub fn write_on_console(self){
            if self.apple == true{
                print!("{}",);
            }else if self.snake == true{

            }else{

            }
        }
    }
}