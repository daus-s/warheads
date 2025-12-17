use crate::ml::vector::{self, Vector};

#[derive(Debug)]
pub struct Simplex {
    vertices: Vec<Vector>, /* (x,y,z) -> ah fuck its a matrix
                            * (1,0,0)
                            * (0,1,0)
                            * (0,0,1)
                            */
}

impl Simplex {
    pub fn new(n: usize) -> Self {
        let vertices = Self::default(n);

        Simplex { vertices }
    }

    pub fn from(points: &[Vector]) -> Self {
        //run asserts at construction
        for point in points {
            assert_eq!(
                point.dim(),
                points[0].dim(),
                "💀 all vertices must have the same dimension. {}≠{}",
                point.dim(),
                points[0].dim()
            );
        }
        assert_eq!(
            points.len(),
            points[0].dim() + 1,
            "💀 simplex must have n + 1 vertices where n is the dimension."
        );

        let vertices = points.to_vec();

        Simplex { vertices }
    }

    fn default(n: usize) -> Vec<Vector> {
        let mut simplex = Vec::with_capacity(n + 1);

        simplex.push(Vector::from(vec![0.0; n]));

        for i in 0..n {
            let mut point = vec![0.0; n];
            point[i] = 1.0;
            simplex.push(Vector::from(point));
        }
        simplex
    }

    pub fn n(&self, index: usize) -> &Vector {
        &self.vertices[index]
    }

    //replace iterator trait with iter method
    pub fn iter(&self) -> impl Iterator<Item = &Vector> {
        self.vertices.iter()
    }

    pub fn rank_vertices(&self, cost: &impl Fn(&Vector) -> f64) -> (Vector, Vector, Vector) {
        let mut best = (self.n(0).clone(), cost(self.n(0)));
        //this is so much better than reevaulating cost 😭
        let mut second_worst = best.clone();
        let mut worst = best.clone();

        for x in self.iter() {
            let cost_x = cost(x);

            //update best
            if cost_x < best.1 {
                //minimize
                best = (x.clone(), cost_x);
            }

            //update worst (and shift previous worst to second worst
            if cost_x > worst.1 {
                second_worst = worst;

                worst = (x.clone(), cost_x);
            } else if cost_x > second_worst.1 {
                second_worst = (x.clone(), cost_x);
            }
        }

        (best.0, second_worst.0, worst.0)
    }

    pub fn centroid(&self) -> Vector {
        let len = self.vertices.len();

        let mut sum = Vector::origin(len - 1);
        for vertex in self.vertices.iter() {
            sum += vertex;
        }

        sum /= len as f64;

        sum
    }

    pub fn dim(&self) -> usize {
        self.vertices[0].dim()
    }

    pub fn replace(&mut self, old: &Vector, new: &Vector) {
        println!("Replacing {} with {}", old, new);
        for vertex in self.vertices.iter_mut() {
            if vertex == old {
                *vertex = Vector::from(new);
                println!("replaced {} with {}", old, new);
            }
        }

        dbg!(&self);
    }

    pub fn shrink_toward(&mut self, target: &Vector) {
        for vertex in self.vertices.iter_mut() {
            *vertex = vector::midpoint(vertex, target);
        }
    }
}

#[cfg(test)]
mod test_simplex {
    use super::*;

    #[test]
    fn test_simplex() {
        let simplex = Simplex::new(2);
        assert_eq!(simplex.vertices.len(), 3);
        assert_eq!(simplex.vertices[0], Vector::from(vec![0.0, 0.0]));
        assert_eq!(simplex.vertices[1], Vector::from(vec![1.0, 0.0]));
        assert_eq!(simplex.vertices[2], Vector::from(vec![0.0, 1.0]));
    }

    #[test]
    fn test_from_vec() {
        let points = vec![
            Vector::from(vec![0.0, 0.0]),
            Vector::from(vec![1.0, 0.0]),
            Vector::from(vec![0.0, 1.0]),
        ];
        let simplex = Simplex::from(&points);
        assert_eq!(simplex.vertices.len(), 3);
        assert_eq!(simplex.vertices[0], Vector::from(vec![0.0, 0.0]));
        assert_eq!(simplex.vertices[1], Vector::from(vec![1.0, 0.0]));
        assert_eq!(simplex.vertices[2], Vector::from(vec![0.0, 1.0]));
    }

    #[test]
    fn test_centroid() {
        let points = vec![
            Vector::from(vec![0.0, 0.0]),
            Vector::from(vec![1.0, 0.0]),
            Vector::from(vec![0.0, 1.0]),
        ];
        let simplex = Simplex::from(&points);
        assert_eq!(
            simplex.centroid(),
            Vector::from(vec![1f64 / 3f64, 1f64 / 3f64])
        );
    }
}
