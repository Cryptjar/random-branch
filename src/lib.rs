#![no_std]

// Enable annotating features requirements in docs
#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]

// This crate is entirely safe, actually it's just macros
#![forbid(unsafe_code)]

// Ensures that `pub` means published in the public API.
// This property is useful for reasoning about breaking API changes.
#![deny(unreachable_pub)]

// Denies invalid links in docs
#![deny(broken_intra_doc_links)]

//! Provides a macro to select a random branch.
//!
//! This crate provides the [`branch`](crate::branch) and
//! [`branch_using`](crate::branch_using) macro, which will
//! execute randomly one of the given expressions.
//!
//! It is maybe best visualized by the following example:
//!
//! ```rust
//! # use random_branch::branch;
//! branch!(
//!     println!("First line."),
//!     println!("Second line?"),
//!     println!("Third line!"),
//! );
//! ```
//!
//! This will be turned into something similar to this:
//!
//! ```rust
//! # use rand::Rng;
//! match rand::thread_rng().gen_range(0..3) {
//!     0 => println!("First line."),
//!     1 => println!("Second line?"),
//!     2 => println!("Third line!"),
//!     _ => unreachable!(),
//! }
//! ```
//!
//! For more details see [`branch`](crate::branch) and
//! [`branch_using`](crate::branch_using). The basic difference between them is,
//! that `branch` uses [`rand::thread_rng()`](rand::thread_rng()) whereas
//! `branch_using` uses the the given [`rand::Rng`](rand::Rng).
//!



/// Branches into one of the given expressions using the given RNG.
///
/// This macro dose essentially the same as [`branch`] but uses the given
/// [`Rng`](rand::Rng).
///
/// This macro turns something like this:
///
/// ```rust
/// # use rand_pcg::Lcg64Xsh32;
/// # use random_branch::branch_using;
/// let mut my_rng = /* snip */
/// # Lcg64Xsh32::new(0,0);
///
/// branch_using!( my_rng, {
///     println!("First line."),
///     println!("Second line?"),
///     println!("Third line!"),
/// });
/// ```
///
/// into something similar to this:
///
/// ```rust
/// # use rand_pcg::Lcg64Xsh32;
/// let mut my_rng = /* snip */
/// # Lcg64Xsh32::new(0,0);
/// # use rand::Rng;
///
/// match my_rng.gen_range(0..3) {
///     0 => println!("First line."),
///     1 => println!("Second line?"),
///     2 => println!("Third line!"),
///     _ => unreachable!(),
/// }
/// ```
///
/// # Examples
///
/// You can use functions, macros and other arbitrary expressions:
///
/// ```rust
/// # use rand_pcg::Lcg64Xsh32;
/// use random_branch::branch_using;
/// fn do_something() {
///      println!("There is no such thing")
/// }
/// let thing = "fuliluf";
/// let mut my_rng = /* snip */
/// # Lcg64Xsh32::new(0,0);
///
/// branch_using!( my_rng, {
///     println!("A {} is an animal!", thing),
///     {
///         let thing = "lufiful";
///         println!("Two {}s will never meet.", thing)
///     },
///     println!("Only a {} can see other {0}s.", thing),
///     do_something(),
/// });
/// ```
///
/// You can also use it as an expression to yield some randomly chosen value:
///
/// ```rust
/// # use rand_pcg::Lcg64Xsh32;
/// use random_branch::branch_using;
/// let mut my_rng = /* snip */
/// # Lcg64Xsh32::new(0,0);
///
/// let num = branch_using!( my_rng, {
///     10,
///     10 + 11,
///     2 * (10 + 11),
///     85,
/// });
/// assert!(num == 10 || num == 21 || num == 42 || num == 85);
/// ```
#[macro_export]
macro_rules! branch_using {
	( $rng:expr, { $( $branch:expr ),* $(,)? }) => {
		{
			random_branch::branch_internal!(@parseRule $rng, 0,
				{ },
				{ $( { $branch } )* },
			)
		}
	};
}


/// Branches into one of the given expressions.
///
/// This macro dose essentially the same as [`branch_using`] instead of giving
/// it some RNG, this macro will simply use the [`rand::thread_rng()`].
/// However, this then requires `std`, unlike `branch_using`.
///
/// This macro turns something like this:
///
/// ```rust
/// # use random_branch::branch;
/// branch!(
///     println!("First line."),
///     println!("Second line?"),
///     println!("Third line!"),
/// );
/// ```
///
/// into something similar to this using the `thread_rng()`:
///
/// ```rust
/// # use rand::Rng;
/// match rand::thread_rng().gen_range(0..3) {
///     0 => println!("First line."),
///     1 => println!("Second line?"),
///     2 => println!("Third line!"),
///     _ => unreachable!(),
/// }
/// ```
///
///
/// # Examples
///
/// You can use functions, macros and other arbitrary expressions:
///
/// ```rust
/// use random_branch::branch;
///
/// fn do_something() {
///      println!("There is no such thing")
/// }
/// let thing = "fuliluf";
///
/// branch!(
///     println!("A {} is an animal!", thing),
///     {
///         let thing = "lufiful";
///         println!("Two {}s will never meet.", thing)
///     },
///     println!("Only a {} can see other {0}s.", thing),
///     do_something(),
/// );
/// ```
///
/// You can also use it as an expression to yield some randomly chosen value:
///
/// ```rust
/// use random_branch::branch;
///
/// let num = branch!(
///     10,
///     10 + 11,
///     2 * (10 + 11),
///     85,
/// );
/// println!("The best number is {}", num);
/// # assert!(num == 10 || num == 21 || num == 42 || num == 85);
/// ```
#[macro_export]
#[cfg(feature = "std")]
#[cfg_attr(feature = "doc_cfg", doc(cfg(feature = "std")))]
macro_rules! branch {
	( $( $branch:expr ),* $(,)? ) => {
		{
			random_branch::branch_internal!(@parseRule rand::thread_rng(), 0,
				{ },
				{ $( { $branch } )* },
			)
		}
	};
}


/// Internal branching macro
///
/// Each branch must be enclosed in braces e.g. `{ }` so it is a single `tt`.
///
/// Syntax:
/// ```text
/// branch_internal!([RNG], [BRANCHES]+)
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! branch_internal {
	// Entry pattern
	( $rng:expr, $( $branches:tt )* ) => {
		random_branch::branch_internal!(@parseRule $rng, 0, {}, { $( $branches:tt )* })
	};

	// Invalid, base case
	(@parseRule $rng:expr, $cnt:expr,
		{  },
		{  },
	) => {
		compile_error!("You must provide at least one choice.")
	};
	// Prepares one branch at a time
	(@parseRule $rng:expr, $cnt:expr,
		{ $( $stuff:tt )* },
		{ $branch:tt $( $rest:tt )* },
	) => {
		{
			random_branch::branch_internal!(@parseRule $rng, $cnt + 1,
				{ $( $stuff )* { $cnt => $branch } },
				{ $( $rest )* },
			)
		}
	};
	// Assembles all branches into a big match
	(@parseRule $rng:expr, $cnt:expr,
		{ $( { $cc:expr => $branch:tt } )* },
		{ },
	) => {{
		match rand::Rng::gen_range(&mut $rng, 0 .. ($cnt)) {
			$( n if n == $cc => $branch )*
			_ => unreachable!()
		}
	}};
}

#[cfg(test)]
mod tests {
	// We actually use mostly doc-tests, which are better suited for macro tests
}
