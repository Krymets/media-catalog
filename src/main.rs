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
}

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

    println!("{:#?}", catalog)

}
