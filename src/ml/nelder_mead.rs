use crate::ml::vector::Vector;

/// # n-dimensional nelder mead
///
//holy fuck the performance is gonna be ass
pub fn nelder_mead(cost: fn(Vector) -> f64, points: &[Vec<f64>]) -> Vec<f64> {
    assert!(
        points.len() > 0,
        "💀 nelder_mead: simplex(n) where n =  dim + 1"
    );

    let n = points.len();
    let simplex = Simplex::new(n);

    if n == 0 {
        panic!("💀 nelder_mead: n > 0");
    }

    let mut best = (simplex.n(0).clone(), cost(simplex.n(0).clone()));
    //this is so much better than reevaulating cost 😭
    let mut second_worst = best.clone();
    let mut worst = best.clone();

    let mut m = Vector::origin(n);

    todo!();
    // for x in simplex.iter() {
    //     m += x;

    //     let cost_x = cost(x.clone());

    //     //update best
    //     if cost_x < best.1 {
    //         //minimize
    //         best = (x.clone(), cost_x);
    //     }

    //     //update worst (and shift previous worst to second worst
    //     if cost_x > worst.1 {
    //         second_worst = worst;

    //         worst = (x.clone(), cost_x);
    //     }
    // }
    // m -= worst; //reflect through centroid

    // m = m.iter().map(|&x| x / ((n - 1) as f64)).collect();

    // let m = m; //remove mutability

    // let r = vec_add(lhs, rhs);

    // vec![]
}

struct Simplex {
    points: Vec<Vector>, /* (x,y,z) -> ah fuck its a matrix
                          * (1,0,0)
                          * (0,1,0)
                          * (0,0,1)
                          */
}

impl Simplex {
    pub fn new(n: usize) -> Self {
        let points = Self::simplex(n);

        Simplex { points }
    }

    fn simplex(n: usize) -> Vec<Vector> {
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
        &self.points[index]
    }

    //replace iterator trait with iter method
    pub fn iter(&self) -> impl Iterator<Item = &Vector> {
        self.points.iter()
    }
}

#[cfg(test)]
mod nelder_mead {
    use super::*;

    #[test]
    fn test_simplex() {
        let simplex = Simplex::new(2);
        assert_eq!(simplex.points.len(), 3);
        assert_eq!(simplex.points[0], Vector::from(vec![0.0, 0.0]));
        assert_eq!(simplex.points[1], Vector::from(vec![1.0, 0.0]));
        assert_eq!(simplex.points[2], Vector::from(vec![0.0, 1.0]));
    }
}
