use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog{items: vec![]}
    }

    pub fn add_item(&mut self, media: Media) {
        self.items.push(media);
    }

    // fn get_by_index(&self, index: usize) -> MightHaveAValue {
    //     if self.items.len() > index {
    //         MightHaveAValue::ThereIsAValue(&self.items[index])
    //     } else {
    //         MightHaveAValue::NoValue
    //     }
    // }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
