mod content;

use content::media::Media;
use content::catalog::Catalog;

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