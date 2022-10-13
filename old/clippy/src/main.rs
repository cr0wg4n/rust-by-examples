extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

fn capture_clipboard() -> Result<String,  Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    return ctx.get_contents();
}

fn main() {
    let demo = capture_clipboard().unwrap_or(String::new());
    println!("{:?}", demo);
}
