pub mod objects{
    struct Objects{
        length:i32,
        wall:bool
    }
    impl Objects{
        pub fn new(&mut self , length:i32){
            self.length = length;
            self.wall = false;
        }
        pub fn colision_check(&mut self,colide:char,apple_preset:char,wall_preset:char){
            if colide==apple_preset{
                self.apple_fn();
            }else if colide==wall_preset{
                self.wall_fn();
            }
        }
        fn apple_fn(&mut self){
            self.length +=1;
        }
        fn wall_fn(&mut self){
            self.wall = true;
        }
    }
}