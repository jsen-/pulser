use pulser::{ReadAdapter, dynamic_array, Digits, string, object, dynamic_object};

struct DbRow {
    id: isize,
    name: String,
    age: u8,
}

impl IntoIterator for DbRow {
    type Item = u8;
    type IntoIter = Box<Iterator<Item = u8>>;

    fn into_iter(self) -> Self::IntoIter {
        let id = self.id.digits();
        let name = string(self.name);
        let age = self.age.digits();
        Box::new(
            object()
                .prop("age", age)
                .prop("name", name)
                .prop("id", id)
                .into_iter(),
        )
    }
}

//

//

//

fn log<T: IntoIterator<Item = u8>>(t: T) {
    let mut r = ReadAdapter::new(t);
    let stdout = std::io::stdout();
    let mut l = stdout.lock();
    std::io::copy(&mut r, &mut l).unwrap();
    println!();
}


fn main() {
    log(std::f32::NAN.digits());
    log((-14987_isize).digits());
    log(DbRow {
        id: 1,
        name: "Jozo".to_string(),
        age: 70,
    });
    let row2 = DbRow {
        id: 2,
        name: "Milan".to_string(),
        age: 52,
    };
    let row3 = DbRow {
        id: 3,
        name: "Cecilka".to_string(),
        age: 92,
    };
    log(dynamic_array(vec![row2, row3]));

    let props = vec![("a".bytes(), 1u8.digits())].into_iter();

    log(dynamic_object(props));

}