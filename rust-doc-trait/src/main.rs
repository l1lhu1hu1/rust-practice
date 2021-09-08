trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // TraitではGoやJavaのinterfaceとは異なり、
    // defaultでabstractでないメソッドを継承先に持たせることができる
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise())
    }
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name);
        } else {
            println!("{} gets a haircut", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            name: name,
            naked: false,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "babaaaaaaaa?"
        } else {
            "aaaaaaaaaa"
        }
    }
    // default trait methodはoverride可
    fn talk(&self) {
        println!("{}: {}", self.name, self.noise());
    }
}

struct Dog {
    name: &'static str,
    vaccinated: bool,
}

// Dogからのみ利用できるメソッドを設定する
impl Dog {
    fn is_vaccinated(&self) -> bool {
        self.vaccinated
    }

    fn get_vaccinated(&mut self) {
        if !self.is_vaccinated() {
            self.vaccinated = true;
            println!("getting vaccinated");
        }
        println!("already vaccinated")
    }
}

impl Animal for Dog {
    fn new(name: &'static str) -> Self {
        Dog {
            name: name,
            vaccinated: false,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.vaccinated {
            "fully vaccinated"
        } else {
            "not yet"
        }
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();

    let mut jj: Dog = Animal::new("JJ");
    jj.talk();
    jj.get_vaccinated();
    jj.talk();
}
