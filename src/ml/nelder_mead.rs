use crate::ml::simplex::Simplex;
use crate::ml::vector::Vector;

/// # n-dimensional nelder mead
///
//holy fuck the performance is gonna be ass
pub fn nelder_mead(cost: impl Fn(&Vector) -> f64, simplex: &mut Simplex) {
    println!("Starting nelder_mead");
    let n = simplex.dim() + 1;

    assert!(n > 0, "💀 nelder_mead: n > 0");
    assert!(
        simplex.dim() == n - 1,
        "💀 nelder_mead: simplex(n) where n =  dim + 1"
    );

    //declarations
    let (best, second_worst, worst) = simplex.rank_vertices(&cost);

    let centroid = simplex.centroid();

    let midpoint = &(&centroid * (n as f64 / (n - 1) as f64)) - &(&worst / (n - 1) as f64);

    let reflection = &(&midpoint * 2.0) - &worst;

    //algorithm
    //remove these cost evaluations or memoize?
    if cost(&reflection) < cost(&second_worst) {
        //reflection is pretty good, how good?
        if cost(&best) < cost(&reflection) {
            //replace w with r
            simplex.replace(&worst, &reflection);
        } else {
            let expansion = &(&reflection * 2.0) - &midpoint;

            if cost(&expansion) < cost(&best) {
                simplex.replace(&worst, &expansion);
            } else {
                simplex.replace(&worst, &reflection);
            }
        }
    } else {
        //cost(reflection) >= cost(second_worst) SHRINK
        //
        if cost(&reflection) < cost(&worst) {
            let contraction = &(&midpoint + &reflection) / 2.0;

            if cost(&contraction) < cost(&reflection) {
                simplex.replace(&worst, &contraction);
            } else {
                // Shrink all points toward best
                simplex.shrink_toward(&best);
            }
        } else {
            // Inside contraction: reflect was worse than worst
            let contraction = &(&midpoint + &worst) / 2.0;

            if cost(&contraction) < cost(&worst) {
                simplex.replace(&worst, &contraction);
            } else {
                // Shrink all points toward best
                simplex.shrink_toward(&best);
            }
        }
        // let mut worst = worst; //if i shadow here is it accessible in all following subbranchee?
        //                        //
        // let contract_point = if cost(&reflection) < cost(&worst) {
        //     println!("replacing {worst} with {reflection}");
        //     simplex.replace(&worst, &reflection);
        //     worst = reflection.clone();
        //     &reflection
        // } else {
        //     &worst
        // };
        // let contraction = &(&midpoint + &contract_point) / 2.0; //worst has been replaced already if necesary thus our case 2 is accounted for
        // dbg!(&contraction);

        // if cost(&contraction) < cost(&worst) {
        //     println!("replacing {worst} with {contraction}");
        //     simplex.replace(&worst, &contraction);
        // } else {
        //     let shrink = &(&best + &worst) / 2.0;
        //     simplex.replace(&worst, &shrink);
        //     simplex.replace(&second_worst, &midpoint);
        // }
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

        dbg!(&b_prime, &g_prime, &w_prime);

        // dbg!(&b_prime, &g_prime, &w_prime);

        assert_eq!(b_prime, Vector::from(vec![0.0, 0.0]));
        assert_eq!(cost(&b_prime), 0.0);

        assert_eq!(g_prime, Vector::from(vec![-0.75, -0.5]));
        assert_eq!(cost(&g_prime), 1.0625);

        assert_eq!(w_prime, Vector::from(vec![1.0, 2.0]));
        assert_eq!(cost(&w_prime), 6.0);
    }
}
