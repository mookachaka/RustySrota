use rustysrota::geometry::EquilateralTriangle;

fn main() {
    let tri = EquilateralTriangle::new(2.0);
    println!("Nodes: {:?}", tri.nodes);
}