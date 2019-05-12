use rand::Rng;

/// The `ConditionalDistribution` trait provides an interface for sampling
/// elements conditionally to priors.
pub trait ConditionalDistribution<T, K>
{
    /// Draw a sample conditionally to the previous state `y`.
    fn conditional_sample<R: Rng + ?Sized>(&self, rng: &mut R, y: T) -> K;
}

/// The `ConditionalPDF` trait provides an interface for conditional
/// probability densities.
pub trait ConditionalPDF<T> {
    /// Returns the conditional probability density function calculated at `y`
    /// conditionally on `x`.
    fn conditional_pdf(&self, x: T, y: T) -> f64;

    /// Calculates the log-probability density at point `y` conditionally on `x`.
    fn ln_conditional_pdf(&self, x: T, y: T) -> f64 {
        self.conditional_pdf(x, y).ln()
    }
}

/// Trait for MCMC transition kernels, fusing the conditional
/// distribution and density traits.
pub trait TransitionKernel<T, K>: ConditionalDistribution<T, K> + ConditionalPDF<T> {
    /// Indicate whether or not the transition kernel is symmetrical.
    fn is_symmetrical(&self) -> bool;
}

default impl<T, K, E> TransitionKernel<T, K> for E
where
    E: ConditionalDistribution<T, K> + ConditionalPDF<T>,
{
    default fn is_symmetrical(&self) -> bool {
        false
    }
}