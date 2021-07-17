struct Dog{

}
struct Cat{

}

trait Animal{
    fn make_noise(&self)->&'static str;
}

impl Animal for Dog {
    fn make_noise(&self)->&'static str {
        return "woof";
    }
}

impl Animal for Cat {
    fn make_noise(&self)->&'static str {
        return "meow";
    }
}

fn get_animal(rand_num:f64)->Box <dyn Animal>{
    if rand_num<1.0{
         Box::new(Dog{})
    }else{
         Box::new(Cat{})
    }
}

fn main(){
  println!("The aimal says {}",get_animal(0.5).make_noise()) ;
  println!("The aimal says {}",get_animal(1.5).make_noise()) 

}