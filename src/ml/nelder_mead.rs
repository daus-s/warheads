use crate::ml::simplex::Simplex;
use crate::ml::vector::Vector;

/// # n-dimensional nelder mead
///
//holy fuck the performance is gonna be ass
pub fn nelder_mead(cost: impl Fn(&Vector) -> f64, simplex: &mut Simplex) {
    let n = simplex.dim() + 1;

    assert!(n > 0, "💀 nelder_mead: n > 0");
    assert!(
        simplex.dim() == n - 1,
        "💀 nelder_mead: simplex(n) where n =  dim + 1"
    );

    //declarations
    let (best, second_worst, worst) = simplex.rank_vertices(&cost);

    let centroid = simplex.centroid();

    let midpoint = &(&centroid * (n as f64 / (n - 1) as f64)) - &(&simplex[worst] / (n - 1) as f64);

    let reflection = &(&midpoint * 2.0) - &simplex[worst];

    let best_cost = cost(&simplex[best]);
    let second_worst_cost = cost(&simplex[second_worst]);
    let worst_cost = cost(&simplex[worst]);
    let reflection_cost = cost(&reflection);
    //algorithm
    //remove these cost evaluations or memoize?
    if reflection_cost < second_worst_cost {
        //reflection is pretty good, how good?
        if best_cost < reflection_cost {
            //replace w with r
            simplex.replace(worst, &reflection);
        } else {
            let expansion = &(&reflection * 2.0) - &midpoint;

            if cost(&expansion) < best_cost {
                simplex.replace(worst, &expansion);
            } else {
                simplex.replace(worst, &reflection);
            }
        }
    } else {
        //cost(reflection) >= cost(second_worst) SHRINK
        //
        if cost(&reflection) < worst_cost {
            let contraction = &(&midpoint + &reflection) / 2.0;

            if cost(&contraction) < cost(&reflection) {
                simplex.replace(worst, &contraction);
            } else {
                // Shrink all points toward best
                simplex.shrink_toward(best);
            }
        } else {
            // Inside contraction: reflect was worse than worst
            let contraction = &(&midpoint + &simplex[worst]) / 2.0;

            if cost(&contraction) < worst_cost {
                simplex.replace(worst, &contraction);
            } else {
                // Shrink all points toward best
                simplex.shrink_toward(best);
            }
        }
    }
}

#[cfg(test)]
mod test_nelder_mead {
    use super::*;

    #[test]
    fn test_nelder_mead() {
        let cost = |v: &Vector| (v.x() - 1.) * v.x() + (v.y() + 1.) * v.y(); //analytically shown the minimum to be at (-.75, -.5)

        let mut simplex = Simplex::from(&vec![
            Vector::from(vec![0.0, 0.0]),
            Vector::from(vec![1.0, 2.0]),
            Vector::from(vec![3.0, 4.0]),
        ]);

        nelder_mead(&cost, &mut simplex);

        let (b_prime, g_prime, w_prime) = simplex.rank_vertices(&cost);

        // dbg!(&b_prime, &g_prime, &w_prime);

        assert_eq!(simplex[b_prime], Vector::from(vec![0.0, 0.0]));
        assert_eq!(simplex[g_prime], Vector::from(vec![-0.75, -0.5]));
        assert_eq!(simplex[w_prime], Vector::from(vec![1.0, 2.0]));
    }
}
