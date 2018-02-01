// Object-oriented state design pattern

pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }    

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { // extract state
            self.state = Some(s.request_review()) // create new state
        }
    } 

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }    
}

trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str { // Will return part of post, thus the 'a lifetime
        "" // Default implementation (only Published will override it)
    }    
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }    
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }    
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }    
}

#[cfg(test)] // tells Rust to compile and run the test code only when we run 'cargo test'
mod tests; // Units test go into a submodule, thus having access to private code if needed

/* We moved the submodule to an external file although the convention would be to put it here:

#[cfg(test)]
mod tests {

    #[test]
    fn all_states() {
        let mut post = super::Post::new();

        let text = "I ate a salad for lunch today";

        post.add_text(text);
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!(text, post.content());        
    }
}
*/