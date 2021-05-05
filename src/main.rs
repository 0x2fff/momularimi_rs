extern crate getopts;
use getopts::Options;

fn main() {
    let moption2momutput = [
        ["A", "Ancientry", "いにしえの図書館"],
        ["F", "Forest", "森を汚さないでね"],
        ["L", "Legend", "伝説のきりかぶ"],
        ["M", "Master", "きのこマスター"],
        ["a", "army", "もみゅ部隊"],
        ["c", "cabbage", "きゃべつウィーク"],
        ["d", "dark", "闇のお茶会"],
        ["f", "forest", "森のシャワー"],
        ["h", "hello", "こんにちは！"],
        ["l", "light", "光るどんぐり"],
        ["m", "momu", "もみゅ"],
        ["u", "ultimate", "究極のまつぼっくり"],
        ["w", "world", "もみゅだけの世界"]];

    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    for text in moption2momutput.iter() {
        opts.optflag(text[0], text[1], text[2]);
    }

    let matches = opts.parse(&args[1..]).unwrap();
    for text in moption2momutput.iter() {
        if matches.opt_present(text[0]) {
            println!("{}", text[2]);
        }
    }
}
