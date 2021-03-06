use crate::base::dimension::{U2, U3};

use crate::geometry::Rotation;

/// A 2-dimensional rotation matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Rotation`](crate::Rotation) type too.**
pub type Rotation2<N> = Rotation<N, U2>;

/// A 3-dimensional rotation matrix.
///
/// **Because this is an alias, not all its methods are listed here. See the [`Rotation`](crate::Rotation) type too.**
pub type Rotation3<N> = Rotation<N, U3>;
