mod bus_a;
mod bus_b;
mod def;

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(tokio::spawn(bus_a::main()), tokio::spawn(bus_b::main()));
    a.unwrap();
    b.unwrap();
}
