pub mod objects{
    pub struct Objects{
        length:i32,
        snake:bool,
        wall:bool
    }
    impl Objects{
        pub fn new(length:i32) -> Objects{
            Objects {
                length,
                snake: false,
                wall: false,
            }
        }
        pub fn colision_check(&mut self,colide:char,apple_preset:char,wall_preset:char,snake_preset:char){
            if colide==apple_preset{
                self.eat_apple();
            }else if colide==wall_preset{
                self.kiss_wall();
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