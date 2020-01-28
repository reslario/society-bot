mod wikipedia;
mod qa;
mod get;
mod out;
mod auth;

fn main() {
    get::get_statement()
        .map(out::format)
        .map(out::tweet);
}
