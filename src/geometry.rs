//! Geometric primitives for tensegrity and structural layouts

/// Represents an equilateral triangle with computed nodes based on edge length
#[derive(Debug, Clone)]
pub struct EquilateralTriangle {
    /// Edge length of the triangle
    pub edge_length: f64,
    /// The three vertices of the triangle
    pub nodes: [nalgebra::Point2<f64>; 3],
}

impl EquilateralTriangle {
    /// Creates a new equilateral triangle with the given edge length
    ///
    /// The triangle is oriented with one vertex at the origin, one on the positive x-axis,
    /// and the third in the first quadrant.
    ///
    /// # Arguments
    /// * `edge_length` - The length of each side of the triangle
    ///
    /// # Example
    /// ```
    /// use rustysrota::geometry::EquilateralTriangle;
    /// let tri = EquilateralTriangle::new(2.0);
    /// assert_eq!(tri.edge_length, 2.0);
    /// assert_eq!(tri.nodes.len(), 3);
    /// ```
    pub fn new(edge_length: f64) -> Self {
        // Standard equilateral triangle with vertices at:
        // A = (0, 0)
        // B = (edge_length, 0)
        // C = (edge_length/2, height)
        let height = edge_length * (3.0_f64.sqrt() / 2.0);
        
        use nalgebra::Point2;
        
        let nodes = [
            Point2::new(0.0, 0.0),
            Point2::new(edge_length, 0.0),
            Point2::new(edge_length / 2.0, height),
        ];
        
        EquilateralTriangle { edge_length, nodes }
    }
    
    /// Computes the area of the triangle
    pub fn area(&self) -> f64 {
        (self.edge_length * self.edge_length * 3.0_f64.sqrt()) / 4.0
    }
    
    /// Computes the perimeter of the triangle
    pub fn perimeter(&self) -> f64 {
        self.edge_length * 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_creation() {
        let tri = EquilateralTriangle::new(2.0);
        assert_eq!(tri.edge_length, 2.0);
        assert_eq!(tri.nodes.len(), 3);
    }

    #[test]
    fn test_triangle_area() {
        let tri = EquilateralTriangle::new(2.0);
        let expected_area = (2.0 * 2.0 * 3.0_f64.sqrt()) / 4.0;
        assert!((tri.area() - expected_area).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_perimeter() {
        let tri = EquilateralTriangle::new(2.0);
        assert_eq!(tri.perimeter(), 6.0);
    }
}
