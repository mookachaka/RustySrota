use rustysrota::geometry::EquilateralTriangle;

fn main() {
    println!("🔺 RustySrota Triangle Demo\n");
    
    let tri = EquilateralTriangle::new(2.0);
    
    println!("Triangle created with edge length: {}", tri.edge_length);
    println!("Vertices:");
    for (i, (x, y)) in tri.nodes.iter().enumerate() {
        println!("  Node {}: ({:.4}, {:.4})", i, x, y);
    }
    println!("\nProperties:");
    println!("  Area: {:.4}", tri.area());
    println!("  Perimeter: {:.4}", tri.perimeter());
}
