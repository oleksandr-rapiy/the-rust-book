fn main() {
    let mut post = state_pattern::Post::new();
    let content = "This is some text for the post";

    post.add_text(content);
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(content, post.content());

    let mut post = state_pattern::blog::Post::new();
    post.add_text(content);

    let post = post.request_review().approve();

    assert_eq!(content, post.content());
}
