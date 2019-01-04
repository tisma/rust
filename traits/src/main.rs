trait Animal {
    fn make_sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn make_sound(&self) { println!("Ruff"); }
}

struct Cat;
impl Animal for Cat {
    fn make_sound(&self) { println!("Meow"); }
}

struct Chinchilla;
impl Animal for Chinchilla {
    fn make_sound(&self) { println!("..."); }
}

fn main() {
    let mut animals = Vec::<Box<dyn Animal>>::new();
    animals.push(Box::new(Dog));
    animals.push(Box::new(Cat));
    animals.push(Box::new(Chinchilla));

    for animal in animals {
        animal.make_sound();
    }
}

