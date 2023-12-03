
mod gaming_board{
    use std::collections::HashMap;

    struct Board{
        map:HashMap<i32,char>,
        width:i32,
        height:i32
    }
    impl Board{
        pub fn new(&mut self, width:i32,height:i32,wall_preset:char,space_preset:char){
            self.width = width;
            self.height = height;

            height.drop();
            let mut height = 0;

            let mut map:HashMap<i32,char> = HashMap::new();
            for i in 0..self.width*self.height{
                if i == 0 || width==i {
                    map.insert(i,wall_preset);
                    if width== i{
                        height += 1;
                    }
                }else{
                    if height== 0 || height == self.height {
                        map.insert(i, wall_preset);
                    }else{
                        map.insert(i,space_preset);
                    }
                }
            }
        }
    }
}