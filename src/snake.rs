pub mod snake{
    pub struct Snake{
        length:i32,
        head_y:i32,
        head_x:i32,
        tail_y:i32,
        tail_x:i32
    }
    impl Snake{
        pub fn new() -> Snake{
            Snake{
                length:1,
                head_y:0,
                head_x:0,
                tail_y:0,
                tail_x:0
            }
        }
        pub fn increase_size(&mut self,sizeup:bool){
            if sizeup == true {
                self.length += 1;
            }
        }
        pub fn snake_location_memorised(self) -> Snake{
            self
        }
    }
}