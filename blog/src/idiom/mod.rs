// Encoding state into the type system

pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
       &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // takes ownership of self, thus consuming DrafPost and transforming it into a PendingReviewPost:
    // no DraftPost instance lingering after this method is called
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }    
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn complete_workflow() {
        let mut post = super::Post::new();

        let text = "I ate a salad for lunch today";

        post.add_text(text);

        let post = post.request_review();

        let post = post.approve();

        assert_eq!(text, post.content());
    }
}