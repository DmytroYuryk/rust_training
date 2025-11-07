use std::collections::HashMap;

// replace with much longer text for testing
const LARGE_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut quis augue posuere, ultricies odio a, luctus justo. Donec varius, elit ac ornare porttitor, eros felis consequat velit, id tristique lacus mi et augue. Praesent nec lobortis elit, nec iaculis neque. Cras sed lorem vel ipsum porttitor fringilla quis ut neque. Vivamus tincidunt mollis turpis, vitae venenatis enim. Nam enim nunc, efficitur et laoreet nec, laoreet id turpis. Fusce vel placerat enim. Maecenas eu tortor scelerisque, varius sem sit amet, semper est. Morbi nisl dolor, blandit ultricies interdum vitae, mattis vel nulla. Praesent sodales nulla in leo tristique, tincidunt facilisis nunc tempus. Ut quis mauris id nunc blandit elementum. Maecenas mattis non justo quis scelerisque. Cras porta, ante vitae fringilla tincidunt, enim metus sodales tellus, ut sollicitudin mi justo nec dolor. Phasellus suscipit metus et nisi pretium interdum. Morbi dignissim turpis orci, molestie pharetra lacus eleifend non. Sed et posuere dui, tempor placerat ligula. Quisque aliquet lacus sem, non efficitur leo tincidunt eu. Curabitur pulvinar, mi in dapibus porta, mi lorem venenatis justo, quis auctor nisl nisi vestibulum odio. Nunc nulla quam, volutpat ut posuere vitae, condimentum in sem. Phasellus ac tortor risus. Quisque vel nulla commodo, mattis elit non, interdum.";

fn main() {
    let map = index_words(LARGE_TEXT);
    println!("{map:#?}");
}

fn index_words(text: &str) -> HashMap<&str, Vec<usize>> {
    let mut hash_map: HashMap<&str, Vec<usize>> = HashMap::new();

    let text_addr = LARGE_TEXT.as_ptr().addr();
    for word in text.split_whitespace() {
        hash_map.entry(word).or_default().push(word.as_ptr().addr() - text_addr);
    }

    return hash_map;
}