#[derive(Debug)]
pub enum Media {
    Book {title: String, author: String},
    Movie {title: String, director: String},
    Audiobook {title: String},
    // Podcast {episode_number: u32},
    Podcast (u32),
    Placeholder
}

impl Media {
    pub fn description(&self) -> String {
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
