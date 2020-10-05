use html_parse;
use html_parse::parse;

fn main() {
    let mut stack: Vec<String> = Vec::new();
    let tree = parse("<div>
        <img src=\"xxx\" alt=\"yyy\" />
        <input type=\"text\" />
        <p>This is a p tag.</p>
    </div>", &mut stack);

    println!("{:#?}", tree);
}
