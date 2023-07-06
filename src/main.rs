fn main() {
    if let Err(err) = artisan::template() {
        println!("{}", err);
    }
}
