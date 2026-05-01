Crate ndarray\_stats Copy item path

<span class="sub-heading"><a href="../src/ndarray_stats/lib.rs.html#1-109" class="src">Source</a>
</span>

Expand description

The [`ndarray-stats`](https://github.com/rust-ndarray/ndarray-stats/)
crate exposes statistical routines for `ArrayBase`, the *n*-dimensional
array data structure provided by
[`ndarray`](https://github.com/rust-ndarray/ndarray).

Currently available routines include:

-   [order statistics](trait.QuantileExt.html) (minimum, maximum,
    median, quantiles, etc.);
-   [summary statistics](trait.SummaryStatisticsExt.html) (mean,
    skewness, kurtosis, central moments, etc.)
-   [partitioning](trait.Sort1dExt.html);
-   [correlation analysis](trait.CorrelationExt.html) (covariance,
    pearson correlation);
-   [measures from information theory](trait.EntropyExt.html) (entropy,
    KL divergence, etc.);
-   [measures of deviation](trait.DeviationExt.html) (count equal, L1,
    L2 distances, mean squared err etc.)
-   [histogram computation](histogram/index.html).

Please feel free to contribute new functionality! A roadmap can be found
[here](https://github.com/rust-ndarray/ndarray-stats/issues/1).

Our work is inspired by other existing statistical packages such as
[`NumPy`](https://docs.scipy.org/doc/numpy-1.14.1/reference/routines.statistics.html)
(Python) and
[`StatsBase.jl`](https://juliastats.github.io/StatsBase.jl/latest/)
(Julia) - any contribution bringing us closer to feature parity is more
than welcome!

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::histogram::`<a href="histogram/trait.HistogramExt.html" class="trait"
title="trait ndarray_stats::histogram::HistogramExt"><code>HistogramExt</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="errors/index.html" class="mod"
title="mod ndarray_stats::errors">errors</a>  
Custom errors returned from our methods and functions.

<a href="histogram/index.html" class="mod"
title="mod ndarray_stats::histogram">histogram</a>  
Histogram functionalities.

<a href="interpolate/index.html" class="mod"
title="mod ndarray_stats::interpolate">interpolate</a>  
Interpolation strategies.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.CorrelationExt.html" class="trait"
title="trait ndarray_stats::CorrelationExt">CorrelationExt</a>  
Extension trait for `ArrayBase` providing functions to compute different
correlation measures.

<a href="trait.DeviationExt.html" class="trait"
title="trait ndarray_stats::DeviationExt">DeviationExt</a>  
An extension trait for `ArrayBase` providing functions to compute
different deviation measures.

<a href="trait.EntropyExt.html" class="trait"
title="trait ndarray_stats::EntropyExt">EntropyExt</a>  
Extension trait for `ArrayBase` providing methods to compute information
theory quantities (e.g. entropy, Kullback–Leibler divergence, etc.).

<a href="trait.MaybeNan.html" class="trait"
title="trait ndarray_stats::MaybeNan">MaybeNan</a>  
A number type that can have not-a-number values.

<a href="trait.MaybeNanExt.html" class="trait"
title="trait ndarray_stats::MaybeNanExt">MaybeNanExt</a>  
Extension trait for `ArrayBase` providing NaN-related functionality.

<a href="trait.Quantile1dExt.html" class="trait"
title="trait ndarray_stats::Quantile1dExt">Quantile1dExt</a>  
Quantile methods for 1-D arrays.

<a href="trait.QuantileExt.html" class="trait"
title="trait ndarray_stats::QuantileExt">QuantileExt</a>  
Quantile methods for `ArrayBase`.

<a href="trait.Sort1dExt.html" class="trait"
title="trait ndarray_stats::Sort1dExt">Sort1dExt</a>  
Methods for sorting and partitioning 1-D arrays.

<a href="trait.SummaryStatisticsExt.html" class="trait"
title="trait ndarray_stats::SummaryStatisticsExt">SummaryStatisticsExt</a>  
Extension trait for `ArrayBase` providing methods to compute several
summary statistics (e.g. mean, variance, etc.).
