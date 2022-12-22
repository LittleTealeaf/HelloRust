fn main() {
    {
        pub trait Draw {
            fn draw(&self);
        }

        pub struct Screen<T: Draw> {
            pub components: Vec<T>,
        }

        impl<T> Screen<T>
        where
            T: Draw,
        {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }

        pub struct Button {
            pub width: u32,
            pub height: u32,
            pub label: String,
        }

        impl Draw for Button {
            fn draw(&self) {
                // code goes here
            }
        }
    }

    // Larger example
    {
        pub struct Post {
            state: Option<Box<dyn State>>,
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
                self.state.as_ref().unwrap().content(self)
            }

            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }

            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }
        }

        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }
        }

        struct Draft {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }

        struct PendingReview {}

        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                Box::new(Published {})
            }
        }

        struct Published {}

        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }
        }

        {
            let mut post = Post::new();

            post.add_text("I ate a salad for lunch today");

            println!("Draft Content: {}", post.content());

            post.request_review();

            println!("Pending Content: {}", post.content());

            post.approve();

            println!("Approved Post Content: {}", post.content());
        }
    }
}
