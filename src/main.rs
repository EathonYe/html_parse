use html_parse;
use html_parse::parse;

fn main() {
    let tree = parse("<div>
        <img src=\"xxx\" />
        <input type=\"text\" />
    </div>");

    println!("{:?}", tree);
}
