
pub mod gaming_board{
    use std::collections::HashMap;

    pub struct Board{
        map:HashMap<i32,char>,
        width:i32,
        height:i32
    }
    impl Board{
        pub fn new() -> Board{
            Board{
                map:HashMap::new(),
                width:0,
                height:0
            }
        }
        pub fn create(&mut self, width:i32,height:i32,wall_preset:char,space_preset:char){
            self.width = width;
            self.height = height;

            width.drop();
            height.drop();

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
    }
}