

pub struct Book {
    pub title: String,
    pub pages: i32,
}

impl Book {
    pub fn new(title: String, pages: i32) -> Self {
        Book {
            title,
            pages,
        }
    }

    pub fn print(&self) {
        println!("{} {}", self.title, self.pages);
    }

    pub fn is_big(&self) -> bool {
        self.pages > 300
    }
}


