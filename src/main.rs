use html_parse;
use html_parse::parse;

fn main() {
    let tree = parse(
        "<div>
        <!-- <div>comment</div> -->
        <img src=\"xxx\" alt=\"yyy\" />
        <input type=\"text\" />
        <!-- <div>comment</div> -->
        <p>
            This is a p tag.
            <span>lalala</span>
        </p>
    </div>",
    );

    println!("{:#?}", tree);
}
