use html_parse;
use html_parse::parse;
use html_parse::parse_props;

fn main() {
    let mut stack: Vec<String> = Vec::new();
    let tree = parse("<div>
        <img src=\"xxx\" alt=\"yyy\" />
        <input type=\"text\" />
        <p>This is a p tag.</p>
    </div>", &mut stack);

    println!("{:#?}", tree);

    let (tag, prop_map) = parse_props("img src=\"xxx\" alt=\"yyy\"");
    println!("tag is {}", tag);
    println!("prop_map is {:#?}", prop_map);
}
