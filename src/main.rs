use html_parse;
use html_parse::parse;

fn main() {
    let tree = parse("<div>
        <img src=\"xxx\" alt=\"yyy\" />
        <input type=\"text\" />
        <p>This is a p tag.</p>
    </div>");

    println!("{:#?}", tree);
}
