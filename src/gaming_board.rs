
pub mod gaming_board{
    use std::collections::HashMap;
    use rand::Rng;

    pub struct Board{
        map:HashMap<i32,char>,
        apple:i32,
        width:i32,
        height:i32
    }
    impl Board{
        pub fn new() -> Board{
            Board{
                map:HashMap::new(),
                apple:0,
                width:0,
                height:0
            }
        }
        pub fn create(&mut self, width:i32,height:i32,wall_preset:char,space_preset:char){
            self.width = width;
            self.height = height;


            let mut height = 1;

            let mut map:HashMap<i32,char> = HashMap::new();
            for i in 1..self.width*self.height{
                if i == 1 || self.width==i {
                    map.insert(i,wall_preset);
                    if self.width== i{
                        height += 1;
                    }
                }else{
                    if height== 1 || height == self.height {
                        map.insert(i, wall_preset);
                    }else{
                        map.insert(i,space_preset);
                    }
                }
            }
        }
        pub fn apple_spawn(&mut self,apple_preset:char,space_preset:char){

            self.map.remove(&self.apple);
            self.map.insert(self.apple,space_preset);

            let mut random = rand::thread_rng();
            loop{
                let location:i32= random.gen_range(1..self.width * self.height - self.width);
                if self.width % location != 1 && self.width % location != 0 {
                    self.map.remove(&location);
                    self.map.insert(location,apple_preset);
                    self.apple = location;
                    break;
                }
            }
        }
    }
}