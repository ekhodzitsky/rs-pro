#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
    age: Option<u32>,
    email: Option<String>,
    phone: Option<String>,
}

struct PersonBuilder {
    first_name: String,
    last_name: String,
    age: Option<u32>,
    email: Option<String>,
    phone: Option<String>,
}

impl PersonBuilder {
    fn new(first_name: &str, last_name: &str) -> PersonBuilder {
        PersonBuilder {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age: None,
            email: None,
            phone: None,
        }
    }

    fn set_age(&mut self, age: u32) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn set_email(&mut self, email: &str) -> &mut Self {
        self.email = Some(email.to_string());
        self
    }

    fn set_phone(&mut self, phone: &str) -> &mut Self {
        self.phone = Some(phone.to_string());
        self
    }

    fn build(&self) -> Person {
        Person {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            age: self.age,
            email: self.email.clone(),
            phone: self.phone.clone(),
        }
    }
}

pub fn demo() {
    println!("Builder");

    let person = PersonBuilder::new("John", "Doe")
        .set_age(30)
        .set_email("johndoe@gmail.com")
        .set_phone("+1234567890")
        .build();

    println!("{:?}", person);

    println!();
}
