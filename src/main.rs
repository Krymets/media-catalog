#[derive(Debug)]
enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    // Podcast {episode_number: u32},
    Podcast (u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book {} by {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie {} by {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook {}", title)
        // } else {
        //     String::from("Media Description")
        // }

        match self {
            Media::Book{title, author} =>
                format!("Book {} by {}", title, author),
            // Media::Book{title, author} => {
            //     format!("Book {} by {}", title, author)
            // },
            Media::Movie{title, director} => {
                format!("Movie {} by {}", title, director)
            },
            Media::Audiobook{title} => {
                format!("Audiobook {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast episode {}", episode_number)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog{items: vec![]}
    }

    fn add_item(&mut self, media: Media) {
        self.items.push(media);
    }

    // fn get_by_index(&self, index: usize) -> MightHaveAValue {
    //     if self.items.len() > index {
    //         MightHaveAValue::ThereIsAValue(&self.items[index])
    //     } else {
    //         MightHaveAValue::NoValue
    //     }
    // }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}

// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValue
// }

// fn print_book(book: &Media) {}
// fn print_movie(book: &Media) {}
fn print_media (media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My book"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("https://www.google.com"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("May Re")
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());
    //
    // print_media(good_movie);
    // print_media(bad_book);
    // print_media(audiobook);

    let mut catalog = Catalog::new();

    catalog.add_item(audiobook);
    catalog.add_item(good_movie);
    catalog.add_item(bad_book);
    catalog.add_item(podcast);
    catalog.add_item(placeholder);

    // println!("{:#?}", catalog.items.get(0));
    //
    // match catalog.items.get(100) {
    //     Some(value) => println!("{:#?}", value),
    //     None => println!("Nothing found")
    // }

    // let items = catalog.get_by_index(4);
    //
    // println!("{:#?}", item);

    // match catalog.get_by_index(0) {
    //     MightHaveAValue::ThereIsAValue(value) => println!("{:#?}", value),
    //     MightHaveAValue::NoValue => println!("No value"),
    // }

    // if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(1) {
    //     println!("There is a value {:#?}", value);
    // } else {
    //     println!("There is not value");
    // }
    if let Some(value) = catalog.get_by_index(1) {
        println!("There is a value {:#?}", value);
    } else {
        println!("There is not value");
    }

    let item = catalog.get_by_index(40);
    let placeholder = Media::Placeholder;
    // print!("{:#?}", item.unwrap());
    //
    // print!("{:#?}", item.expect("Must be some value"));

    print!("{:#?}", item.unwrap_or(&placeholder));

}


// #[derive(Debug)]
// struct Account {
//     balance: i32
// }
//
// fn main() {
//     let mut accounts: Vec<Account> = vec![
//         Account { balance: 0 },
//         Account { balance: 10 }
//     ];
//
//     // Add code here:
//     match accounts.get_mut(10) {
//         Some(value) => {
//             value.balance = 30;
//             println!("{:#?}", value);
//         }
//         None => println!("No account found"),
//     }
// }


// #[derive(Debug)]
// struct Account {
//     balance: i32
// }
//
// fn main() {
//     let mut accounts: Vec<Account> = vec![
//         Account { balance: 0 },
//         Account { balance: 10 }
//     ];
//
//     // Add code here:
//     match accounts.first_mut() {
//         Some(value) => {
//             value.balance = 30;
//             println!("{:#?}", value);
//         }
//         None => println!("No account found"),
//     }
// }