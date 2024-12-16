use super::SumcheckRandomScalars;
use crate::base::{
    map::{IndexMap, IndexSet},
    polynomial::{
        compute_truncated_lagrange_basis_inner_product, compute_truncated_lagrange_basis_sum,
    },
    scalar::Scalar,
};
use core::iter::IntoIterator;

/// Evaluations for different MLEs at the random point chosen for sumcheck
#[derive(Default)]
pub struct SumcheckMleEvaluations<'a, S: Scalar> {
    /// The number of sumcheck variables.
    pub num_sumcheck_variables: usize,
    /// The evaluation (at the random point generated by sumcheck) of an MLE `{x_i}` where
    ///     `x_i = 1` if `i < length;`
    ///         = 0, otherwise
    pub one_evaluations: IndexMap<usize, S>,
    /// The evaluation (at the random point generated by sumcheck) of the MLE formed from all ones with length 1.
    pub singleton_one_evaluation: S,
    /// The evaluation (at the random point generated by sumcheck) of the MLE formed from entrywise random scalars.
    ///
    /// This is used within sumcheck to establish that a given expression
    /// is zero across all entries.
    pub random_evaluation: S,
    /// The evaluations (at the random point generated by sumcheck) of the mles that are evaluated by the inner product argument. These are batched together and checked by a single IPA.
    pub pcs_proof_evaluations: &'a [S],

    /// Evaluation (at the random point generated by sumcheck) of the function `rho_256` that is defined by rho_256(x) = x when 0 <= x < 256 and 0 otherwise.
    pub rho_256_evaluation: Option<S>,
}

#[allow(
    clippy::missing_panics_doc,
    reason = "Assertions ensure preconditions are met, eliminating the possibility of panic."
)]
impl<'a, S: Scalar> SumcheckMleEvaluations<'a, S> {
    /// Constructs the evaluations for the sumcheck MLEs.
    ///
    /// # Inputs
    /// - `evaluation_point` - the point, outputted by sumcheck, at which to evaluate the MLEs
    /// - `sumcheck_random_scalars` - the random scalars used to batch the evaluations that are proven via IPA
    /// - `pcs_proof_evaluations` - the evaluations of the MLEs that are proven via IPA
    pub fn new(
        range_length: usize,
        one_evaluation_lengths: impl IntoIterator<Item = usize>,
        evaluation_point: &[S],
        sumcheck_random_scalars: &SumcheckRandomScalars<S>,
        pcs_proof_evaluations: &'a [S],
    ) -> Self {
        let rho_256_evaluation = if evaluation_point.len() < 8 {
            None
        } else {
            let rho_256_intermediate = evaluation_point
                .iter()
                .take(8)
                .rev()
                .fold(S::ZERO, |acc, &x| acc * S::TWO + x);
            Some(
                evaluation_point
                    .iter()
                    .skip(8)
                    .fold(rho_256_intermediate, |acc, &x| acc * (S::ONE - x)),
            )
        };

        assert_eq!(
            evaluation_point.len(),
            sumcheck_random_scalars.entrywise_point.len()
        );
        assert_eq!(range_length, sumcheck_random_scalars.table_length);
        let random_evaluation = compute_truncated_lagrange_basis_inner_product(
            range_length,
            evaluation_point,
            sumcheck_random_scalars.entrywise_point,
        );
        let unique_one_evaluation_lengths: IndexSet<usize> =
            one_evaluation_lengths.into_iter().collect();
        let one_evaluations = unique_one_evaluation_lengths
            .iter()
            .map(|&length| {
                (
                    length,
                    compute_truncated_lagrange_basis_sum(length, evaluation_point),
                )
            })
            .collect();
        let singleton_one_evaluation = compute_truncated_lagrange_basis_sum(1, evaluation_point);
        Self {
            num_sumcheck_variables: evaluation_point.len(),
            one_evaluations,
            singleton_one_evaluation,
            random_evaluation,
            pcs_proof_evaluations,
            rho_256_evaluation,
        }
    }
}
