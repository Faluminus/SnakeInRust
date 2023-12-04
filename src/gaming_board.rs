pub mod gaming_board{
    use std::collections::HashMap;
    use rand::Rng;
    use crate::objects;


    pub struct Board{
        map:HashMap<i32,objects::objects::Objects>,
        no_walls: HashMap<i32,i32>,
        apple:i32,
        width:i32,
        height:i32
    }
    impl Board{
        pub fn new() -> Board{
            Board{
                map:HashMap::new(),
                no_walls:HashMap::new(),
                apple:0,
                width:0,
                height:0
            }
        }
        pub fn create(&mut self, width:i32,height:i32,apple_preset:char,wall_preset:char,snake_preset:char ,empty_preset:char){
            self.width = width;
            self.height = height;

            let mut height = 1;

            let mut map:HashMap<i32,objects::objects::Objects> = HashMap::new();
            for i in 1..self.width*self.height{
                let mut object = objects::objects::Objects::new(apple_preset,snake_preset,wall_preset,empty_preset);
                if i == 1 || self.width==i {
                    object.wall();
                    map.insert(i,object);
                    if self.width== i{
                        height += 1;
                    }
                }else{
                    if height== 1 || height == self.height {
                        object.wall();
                        map.insert(i, object);
                    }else{
                        object.empty();
                        map.insert(i,object);
                    }
                }
            }

            let mut counter = 1;
            for i in 0..self.width*self.height {
                if i > self.width && i < self.width{
                    if i != self.width +1 && i != self.width -1{
                        self.no_walls.insert(counter,i);
                        counter += 1;
                    }
                }
            }
        }
        pub fn apple_spawn(&mut self){
            if self.apple != 0{
                self.map.get_mut(&self.apple).unwrap().empty();
            }

            let mut random = rand::thread_rng();

            let location:i32= random.gen_range(1..(self.width-2) * (self.height-2));
            self.apple = *self.no_walls.get_mut(&location).unwrap();
            self.map.get_mut(&self.apple).unwrap().apple();
        }
        pub fn snake_location_calculation(){

        }
    }
}