pub mod objects{
    struct Objects{
        length:i32,
        snake:bool,
        wall:bool
    }
    impl Objects{
        pub fn new(&mut self , length:i32){
            self.length = length;
            self.snake = false;
            self.wall = false;
        }
        pub fn colision_check(&mut self,colide:char,apple_preset:char,wall_preset:char,snake_preset:char){
            if colide==apple_preset{
                self.apple_fn();
            }else if colide==wall_preset{
                self.wall_fn();
            }else if colide==snake_preset{
                self.twist_your_neck();
            }
        }
        fn eat_apple(&mut self){
            self.length +=1;
        }
        fn kiss_wall(&mut self){
            self.wall = true;
        }
        fn twist_your_neck(&mut self){
            self.snake = true;
        }
    }
}