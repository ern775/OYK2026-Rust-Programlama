use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct LapTime {
    minutes: u32,
    seconds: u32,
}

impl fmt::Display for LapTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{:02}", self.minutes, self.seconds)
    }
}

// builder icin oyun karakteri
#[derive(Debug)]
struct Character {
    name: String,
    health: u32,
    attack: u32,
    can_fly: bool,
}

impl Character {
    fn new(name: &str) -> Character {
        Character {
            name: name.to_string(),
            health: 100,
            attack: 10,
            can_fly: false,
        }
    }

    // her halka: mut self alir, bir alani degistirir, self'i geri dondurur
    fn health(mut self, x: u32) -> Self {
        self.health = x;
        self
    }

    fn attack(mut self, x: u32) -> Self {
        self.attack = x;
        self
    }

    fn can_fly(mut self) -> Self {
        self.can_fly = true;
        self
    }

    fn build(self) -> Character {
        self
    }
}

fn main() {
    let t1 = LapTime {
        minutes: 3,
        seconds: 45,
    };
    let t2 = LapTime {
        minutes: 3,
        seconds: 5,
    };

    // Debug gelistirici icin, Display kullanici icin
    println!("{:?}", t1);
    println!("{:#?}", t2);
    println!("{} ve {}", t1, t2); // Display: 3:45 ve 3:05
    // PartialEq -> ==
    println!("esit mi: {}", t1 == t2);

    // BUILDER - alan sirasi onemsiz, yazmadiginiz alan varsayilan kalir
    let ejder = Character::new("Ejderha").health(120).attack(15).can_fly();
    println!("{:?}", ejder);

    let kopek = Character::new("Kopek").build(); // hepsi varsayilan
    println!("{:?}", kopek);
    println!(
        "{} can={} / {} can={}",
        ejder.name, ejder.health, kopek.name, kopek.health
    );
}
