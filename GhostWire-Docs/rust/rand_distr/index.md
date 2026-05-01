Crate rand\_distr Copy item path

<span class="sub-heading"><a href="../src/rand_distr/lib.rs.html#9-213" class="src">Source</a>
</span>

Expand description

Generating random samples from probability distributions.

## <a href="#re-exports" class="doc-anchor">§</a>Re-exports

This crate is a super-set of the
[`rand::distributions`](../rand/distributions/index.html "mod rand::distributions")
module. See the
[`rand::distributions`](../rand/distributions/index.html "mod rand::distributions")
module documentation for an overview of the core
[`Distribution`](trait.Distribution.html "trait rand_distr::Distribution")
trait and implementations.

The following are re-exported:

-   The
    [`Distribution`](trait.Distribution.html "trait rand_distr::Distribution")
    trait and
    [`DistIter`](struct.DistIter.html "struct rand_distr::DistIter")
    helper type
-   The
    [`Standard`](struct.Standard.html "struct rand_distr::Standard"),
    [`Alphanumeric`](struct.Alphanumeric.html "struct rand_distr::Alphanumeric"),
    [`Uniform`](struct.Uniform.html "struct rand_distr::Uniform"),
    [`OpenClosed01`](struct.OpenClosed01.html "struct rand_distr::OpenClosed01"),
    [`Open01`](struct.Open01.html "struct rand_distr::Open01"),
    [`Bernoulli`](struct.Bernoulli.html "struct rand_distr::Bernoulli"),
    and
    [`WeightedIndex`](struct.WeightedIndex.html "struct rand_distr::WeightedIndex")
    distributions

## <a href="#distributions" class="doc-anchor">§</a>Distributions

This crate provides the following probability distributions:

-   Related to real-valued quantities that grow linearly (e.g. errors,
    offsets):
    -   [`Normal`](struct.Normal.html "struct rand_distr::Normal")
        distribution, and
        [`StandardNormal`](struct.StandardNormal.html "struct rand_distr::StandardNormal")
        as a primitive
    -   [`SkewNormal`](struct.SkewNormal.html "struct rand_distr::SkewNormal")
        distribution
    -   [`Cauchy`](struct.Cauchy.html "struct rand_distr::Cauchy")
        distribution
-   Related to Bernoulli trials (yes/no events, with a given
    probability):
    -   [`Binomial`](struct.Binomial.html "struct rand_distr::Binomial")
        distribution
    -   [`Geometric`](struct.Geometric.html "struct rand_distr::Geometric")
        distribution
    -   [`Hypergeometric`](struct.Hypergeometric.html "struct rand_distr::Hypergeometric")
        distribution
-   Related to positive real-valued quantities that grow exponentially
    (e.g. prices, incomes, populations):
    -   [`LogNormal`](struct.LogNormal.html "struct rand_distr::LogNormal")
        distribution
-   Related to the occurrence of independent events at a given rate:
    -   [`Pareto`](struct.Pareto.html "struct rand_distr::Pareto")
        distribution
    -   [`Poisson`](struct.Poisson.html "struct rand_distr::Poisson")
        distribution
    -   [`Exp`](struct.Exp.html "struct rand_distr::Exp")onential
        distribution, and
        [`Exp1`](struct.Exp1.html "struct rand_distr::Exp1") as a
        primitive
    -   [`Weibull`](struct.Weibull.html "struct rand_distr::Weibull")
        distribution
    -   [`Gumbel`](struct.Gumbel.html "struct rand_distr::Gumbel")
        distribution
    -   [`Frechet`](struct.Frechet.html "struct rand_distr::Frechet")
        distribution
    -   [`Zeta`](struct.Zeta.html "struct rand_distr::Zeta")
        distribution
    -   [`Zipf`](struct.Zipf.html "struct rand_distr::Zipf")
        distribution
-   Gamma and derived distributions:
    -   [`Gamma`](struct.Gamma.html "struct rand_distr::Gamma")
        distribution
    -   [`ChiSquared`](struct.ChiSquared.html "struct rand_distr::ChiSquared")
        distribution
    -   [`StudentT`](struct.StudentT.html "struct rand_distr::StudentT")
        distribution
    -   [`FisherF`](struct.FisherF.html "struct rand_distr::FisherF")
        distribution
-   Triangular distribution:
    -   [`Beta`](struct.Beta.html "struct rand_distr::Beta")
        distribution
    -   [`Triangular`](struct.Triangular.html "struct rand_distr::Triangular")
        distribution
-   Multivariate probability distributions
    -   [`Dirichlet`](struct.Dirichlet.html "struct rand_distr::Dirichlet")
        distribution
    -   [`UnitSphere`](struct.UnitSphere.html "struct rand_distr::UnitSphere")
        distribution
    -   [`UnitBall`](struct.UnitBall.html "struct rand_distr::UnitBall")
        distribution
    -   [`UnitCircle`](struct.UnitCircle.html "struct rand_distr::UnitCircle")
        distribution
    -   [`UnitDisc`](struct.UnitDisc.html "struct rand_distr::UnitDisc")
        distribution
-   Alternative implementation for weighted index sampling
    -   [`WeightedAliasIndex`](weighted_alias/struct.WeightedAliasIndex.html "struct rand_distr::weighted_alias::WeightedAliasIndex")
        distribution
-   Misc. distributions
    -   [`InverseGaussian`](struct.InverseGaussian.html "struct rand_distr::InverseGaussian")
        distribution
    -   [`NormalInverseGaussian`](struct.NormalInverseGaussian.html "struct rand_distr::NormalInverseGaussian")
        distribution

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use weighted_alias::`<a href="weighted_alias/struct.WeightedAliasIndex.html" class="struct"
title="struct rand_distr::weighted_alias::WeightedAliasIndex"><code>WeightedAliasIndex</code></a>`;`

`pub use `<a href="../num_traits/index.html" class="mod"
title="mod num_traits"><code>num_traits</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="uniform/index.html" class="mod"
title="mod rand_distr::uniform">uniform</a>  
A distribution uniformly sampling numbers within a given range.

<a href="weighted_alias/index.html" class="mod"
title="mod rand_distr::weighted_alias">weighted_alias</a>  
This module contains an implementation of alias method for sampling
random indices with probabilities proportional to a collection of
weights.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Alphanumeric.html" class="struct"
title="struct rand_distr::Alphanumeric">Alphanumeric</a>  
Sample a `u8`, uniformly distributed over ASCII letters and numbers:
a-z, A-Z and 0-9.

<a href="struct.Bernoulli.html" class="struct"
title="struct rand_distr::Bernoulli">Bernoulli</a>  
The Bernoulli distribution.

<a href="struct.Beta.html" class="struct"
title="struct rand_distr::Beta">Beta</a>  
The Beta distribution with shape parameters `alpha` and `beta`.

<a href="struct.Binomial.html" class="struct"
title="struct rand_distr::Binomial">Binomial</a>  
The binomial distribution `Binomial(n, p)`.

<a href="struct.Cauchy.html" class="struct"
title="struct rand_distr::Cauchy">Cauchy</a>  
The Cauchy distribution `Cauchy(median, scale)`.

<a href="struct.ChiSquared.html" class="struct"
title="struct rand_distr::ChiSquared">ChiSquared</a>  
The chi-squared distribution `χ²(k)`, where `k` is the degrees of
freedom.

<a href="struct.Dirichlet.html" class="struct"
title="struct rand_distr::Dirichlet">Dirichlet</a>  
The Dirichlet distribution `Dirichlet(alpha)`.

<a href="struct.DistIter.html" class="struct"
title="struct rand_distr::DistIter">DistIter</a>  
An iterator that generates random values of `T` with distribution `D`,
using `R` as the source of randomness.

<a href="struct.Exp.html" class="struct"
title="struct rand_distr::Exp">Exp</a>  
The exponential distribution `Exp(lambda)`.

<a href="struct.Exp1.html" class="struct"
title="struct rand_distr::Exp1">Exp1</a>  
Samples floating-point numbers according to the exponential
distribution, with rate parameter `λ = 1`. This is equivalent to
`Exp::new(1.0)` or sampling with `-rng.gen::<f64>().ln()`, but faster.

<a href="struct.FisherF.html" class="struct"
title="struct rand_distr::FisherF">FisherF</a>  
The Fisher F distribution `F(m, n)`.

<a href="struct.Frechet.html" class="struct"
title="struct rand_distr::Frechet">Frechet</a>  
Samples floating-point numbers according to the Fréchet distribution

<a href="struct.Gamma.html" class="struct"
title="struct rand_distr::Gamma">Gamma</a>  
The Gamma distribution `Gamma(shape, scale)` distribution.

<a href="struct.Geometric.html" class="struct"
title="struct rand_distr::Geometric">Geometric</a>  
The geometric distribution `Geometric(p)` bounded to `[0, u64::MAX]`.

<a href="struct.Gumbel.html" class="struct"
title="struct rand_distr::Gumbel">Gumbel</a>  
Samples floating-point numbers according to the Gumbel distribution

<a href="struct.Hypergeometric.html" class="struct"
title="struct rand_distr::Hypergeometric">Hypergeometric</a>  
The hypergeometric distribution `Hypergeometric(N, K, n)`.

<a href="struct.InverseGaussian.html" class="struct"
title="struct rand_distr::InverseGaussian">InverseGaussian</a>  
The [inverse Gaussian
distribution](https://en.wikipedia.org/wiki/Inverse_Gaussian_distribution)

<a href="struct.LogNormal.html" class="struct"
title="struct rand_distr::LogNormal">LogNormal</a>  
The log-normal distribution `ln N(mean, std_dev**2)`.

<a href="struct.Normal.html" class="struct"
title="struct rand_distr::Normal">Normal</a>  
The normal distribution `N(mean, std_dev**2)`.

<a href="struct.NormalInverseGaussian.html" class="struct"
title="struct rand_distr::NormalInverseGaussian">NormalInverseGaussian</a>  
The [normal-inverse Gaussian
distribution](https://en.wikipedia.org/wiki/Normal-inverse_Gaussian_distribution)

<a href="struct.Open01.html" class="struct"
title="struct rand_distr::Open01">Open01</a>  
A distribution to sample floating point numbers uniformly in the open
interval `(0, 1)`, i.e. not including either endpoint.

<a href="struct.OpenClosed01.html" class="struct"
title="struct rand_distr::OpenClosed01">OpenClosed01</a>  
A distribution to sample floating point numbers uniformly in the
half-open interval `(0, 1]`, i.e. including 1 but not 0.

<a href="struct.Pareto.html" class="struct"
title="struct rand_distr::Pareto">Pareto</a>  
Samples floating-point numbers according to the Pareto distribution

<a href="struct.Pert.html" class="struct"
title="struct rand_distr::Pert">Pert</a>  
The PERT distribution.

<a href="struct.Poisson.html" class="struct"
title="struct rand_distr::Poisson">Poisson</a>  
The Poisson distribution `Poisson(lambda)`.

<a href="struct.SkewNormal.html" class="struct"
title="struct rand_distr::SkewNormal">SkewNormal</a>  
The [skew normal
distribution](https://en.wikipedia.org/wiki/Skew_normal_distribution)
`SN(location, scale, shape)`.

<a href="struct.Standard.html" class="struct"
title="struct rand_distr::Standard">Standard</a>  
A generic random value distribution, implemented for many primitive
types. Usually generates values with a numerically uniform distribution,
and with a range appropriate to the type.

<a href="struct.StandardGeometric.html" class="struct"
title="struct rand_distr::StandardGeometric">StandardGeometric</a>  
Samples integers according to the geometric distribution with success
probability `p = 0.5`. This is equivalent to `Geometeric::new(0.5)`, but
faster.

<a href="struct.StandardNormal.html" class="struct"
title="struct rand_distr::StandardNormal">StandardNormal</a>  
Samples floating-point numbers according to the normal distribution
`N(0, 1)` (a.k.a. a standard normal, or Gaussian). This is equivalent to
`Normal::new(0.0, 1.0)` but faster.

<a href="struct.StudentT.html" class="struct"
title="struct rand_distr::StudentT">StudentT</a>  
The Student t distribution, `t(nu)`, where `nu` is the degrees of
freedom.

<a href="struct.Triangular.html" class="struct"
title="struct rand_distr::Triangular">Triangular</a>  
The triangular distribution.

<a href="struct.Uniform.html" class="struct"
title="struct rand_distr::Uniform">Uniform</a>  
Sample values uniformly between two bounds.

<a href="struct.UnitBall.html" class="struct"
title="struct rand_distr::UnitBall">UnitBall</a>  
Samples uniformly from the unit ball (surface and interior) in three
dimensions.

<a href="struct.UnitCircle.html" class="struct"
title="struct rand_distr::UnitCircle">UnitCircle</a>  
Samples uniformly from the edge of the unit circle in two dimensions.

<a href="struct.UnitDisc.html" class="struct"
title="struct rand_distr::UnitDisc">UnitDisc</a>  
Samples uniformly from the unit disc in two dimensions.

<a href="struct.UnitSphere.html" class="struct"
title="struct rand_distr::UnitSphere">UnitSphere</a>  
Samples uniformly from the surface of the unit sphere in three
dimensions.

<a href="struct.Weibull.html" class="struct"
title="struct rand_distr::Weibull">Weibull</a>  
Samples floating-point numbers according to the Weibull distribution

<a href="struct.WeightedIndex.html" class="struct"
title="struct rand_distr::WeightedIndex">WeightedIndex</a>  
A distribution using weighted sampling of discrete items

<a href="struct.Zeta.html" class="struct"
title="struct rand_distr::Zeta">Zeta</a>  
Samples integers according to the [zeta
distribution](https://en.wikipedia.org/wiki/Zeta_distribution).

<a href="struct.Zipf.html" class="struct"
title="struct rand_distr::Zipf">Zipf</a>  
Samples integers according to the Zipf distribution.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.BernoulliError.html" class="enum"
title="enum rand_distr::BernoulliError">BernoulliError</a>  
Error type returned from `Bernoulli::new`.

<a href="enum.BetaError.html" class="enum"
title="enum rand_distr::BetaError">BetaError</a>  
Error type returned from `Beta::new`.

<a href="enum.BinomialError.html" class="enum"
title="enum rand_distr::BinomialError">BinomialError</a>  
Error type returned from `Binomial::new`.

<a href="enum.CauchyError.html" class="enum"
title="enum rand_distr::CauchyError">CauchyError</a>  
Error type returned from `Cauchy::new`.

<a href="enum.ChiSquaredError.html" class="enum"
title="enum rand_distr::ChiSquaredError">ChiSquaredError</a>  
Error type returned from `ChiSquared::new` and `StudentT::new`.

<a href="enum.DirichletError.html" class="enum"
title="enum rand_distr::DirichletError">DirichletError</a>  
Error type returned from `Dirchlet::new`.

<a href="enum.ExpError.html" class="enum"
title="enum rand_distr::ExpError">ExpError</a>  
Error type returned from `Exp::new`.

<a href="enum.FisherFError.html" class="enum"
title="enum rand_distr::FisherFError">FisherFError</a>  
Error type returned from `FisherF::new`.

<a href="enum.FrechetError.html" class="enum"
title="enum rand_distr::FrechetError">FrechetError</a>  
Error type returned from `Frechet::new`.

<a href="enum.GammaError.html" class="enum"
title="enum rand_distr::GammaError">GammaError</a>  
Error type returned from `Gamma::new`.

<a href="enum.GeoError.html" class="enum"
title="enum rand_distr::GeoError">GeoError</a>  
Error type returned from `Geometric::new`.

<a href="enum.GumbelError.html" class="enum"
title="enum rand_distr::GumbelError">GumbelError</a>  
Error type returned from `Gumbel::new`.

<a href="enum.HyperGeoError.html" class="enum"
title="enum rand_distr::HyperGeoError">HyperGeoError</a>  
Error type returned from `Hypergeometric::new`.

<a href="enum.InverseGaussianError.html" class="enum"
title="enum rand_distr::InverseGaussianError">InverseGaussianError</a>  
Error type returned from `InverseGaussian::new`

<a href="enum.NormalError.html" class="enum"
title="enum rand_distr::NormalError">NormalError</a>  
Error type returned from `Normal::new` and `LogNormal::new`.

<a href="enum.NormalInverseGaussianError.html" class="enum"
title="enum rand_distr::NormalInverseGaussianError">NormalInverseGaussianError</a>  
Error type returned from `NormalInverseGaussian::new`

<a href="enum.ParetoError.html" class="enum"
title="enum rand_distr::ParetoError">ParetoError</a>  
Error type returned from `Pareto::new`.

<a href="enum.PertError.html" class="enum"
title="enum rand_distr::PertError">PertError</a>  
Error type returned from
[`Pert`](struct.Pert.html "struct rand_distr::Pert") constructors.

<a href="enum.PoissonError.html" class="enum"
title="enum rand_distr::PoissonError">PoissonError</a>  
Error type returned from `Poisson::new`.

<a href="enum.SkewNormalError.html" class="enum"
title="enum rand_distr::SkewNormalError">SkewNormalError</a>  
Error type returned from `SkewNormal::new`.

<a href="enum.TriangularError.html" class="enum"
title="enum rand_distr::TriangularError">TriangularError</a>  
Error type returned from
[`Triangular::new`](struct.Triangular.html#method.new "associated function rand_distr::Triangular::new").

<a href="enum.WeibullError.html" class="enum"
title="enum rand_distr::WeibullError">WeibullError</a>  
Error type returned from `Weibull::new`.

<a href="enum.WeightedError.html" class="enum"
title="enum rand_distr::WeightedError">WeightedError</a>  
Error type returned from `WeightedIndex::new`.

<a href="enum.ZetaError.html" class="enum"
title="enum rand_distr::ZetaError">ZetaError</a>  
Error type returned from `Zeta::new`.

<a href="enum.ZipfError.html" class="enum"
title="enum rand_distr::ZipfError">ZipfError</a>  
Error type returned from `Zipf::new`.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.Distribution.html" class="trait"
title="trait rand_distr::Distribution">Distribution</a>  
Types (distributions) that can be used to create a random instance of
`T`.
