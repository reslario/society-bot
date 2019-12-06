mod wikipedia;
mod qa;
mod get;
mod out;

fn main() {
    get::get_statement()
        .map(out::format)
        .map(out::print);
}
