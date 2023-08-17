struct Type1;
struct Type2;
struct Type3;

impl Type1{
    fn method1(&self) {
        println!("Type1");
    }
}
impl Type2{
    fn method2(&self) {
        println!("Type2");
    }
}
impl Type3{
    fn method3(&self) {
        println!("Type3");
    }
}

//Vec - enum可以调用不同类型上的不同方法，method之间不相关
fn enum(){
    let vec: Vec<WrappedType> = vec![
        WrappedType::One(Type1),
        WrappedType::Two(Type2),
        WrappedType::Three(Type3),
    ];
    for type in vec.iter(){
        match type{
            WrappedType::One(type1) => type1.method1(),
            WrappedType::Two(type2) => type2.method2(),
            WrappedType::Three(type3) => type3.method3(),
        }
    }
}

//Trait Object - trait只能调用trait里面定义的方法，methodxiangtong但impl可以有差异
trait TraitMethod {
    fn method(&self);
}

impl TraitMethod for Type1 {
    fn method(&self){
        println!("Type1");
    }
}

impl TraitMethod for Type2 {
    fn method(&self){
        println!("Type2");
    }
}

impl TraitMethod for Type3 {
    fn method(&self){
        println!("Type3");
    }
}

fn trait_obj(){
    let vec: Vec<Box<dyn TraitMethod>> = vec! [
        Box::new(Type1),
        Box::new(Type2),
        Box::new(Type3),
    ];

    for trait_type in vec.iter(){
        trait_method.method();
    }
}

fn main(){
    enum();
    trait_method();
}