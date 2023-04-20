MODULE_TEMPLATE='''
//! This module provides %(category)s SI units, such as %(example1)s 
//! and %(example2)s.
use std::fmt;
use super::UnitStruct;
use super::NumLike;
%(crate imports)s

// optional supports
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[cfg(feature="num-bigfloat")]
use num_bigfloat;
#[cfg(feature="num-complex")]
use num_complex;

%(content)s

%(appendix)s
'''

UNIT_STRUCT_DEFINITION_TEMPLATE='''
/// The %(desc first name)s unit type, defined as %(unit name)s in SI units
#[derive(UnitStruct, Debug, Clone)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct %(code name)s<T: NumLike>{
	/// The value of this %(capital desc name)s in %(unit name)s
	pub %(unit symbol)s: T
}

impl<T> %(code name)s<T> where T: NumLike {

	/// Returns the standard unit name of %(desc name)s: "%(unit name)s"
	pub fn unit_name() -> &'static str { "%(unit name)s" }
	
	/// Returns the abbreviated name or symbol of %(desc name)s: "%(unit symbol human)s" for %(unit name)s
	pub fn unit_symbol() -> &'static str { "%(unit symbol human)s" }
	%(non-converting methods)s
}

impl<T> fmt::Display for %(code name)s<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.%(unit symbol)s, Self::unit_symbol())
	}
}

impl<T> %(code name)s<T> where T: NumLike+From<f64> {
	%(to-and-from)s
}

%(uom integration)s
'''

NON_COEFFICIENT_TO_FROM_TEMPLATE = '''
	/// Returns a new %(desc name)s value from the given number of %(user unit name)s
	///
	/// # Arguments
	/// * `%(user unit symbol)s` - Any number-like type, representing a quantity of %(unit name)s
	pub fn from_%(user unit symbol)s(%(user unit symbol)s: T) -> Self { %(code name)s{%(unit symbol)s: %(user unit symbol)s} }
	
	/// Returns a copy of this %(desc name)s value in %(user unit name)s
	pub fn to_%(user unit symbol)s(&self) -> T { self.%(unit symbol)s.clone() }
'''

TO_FROM_SLOPE_OFFSET_TEMPLATE = '''
	/// Returns a copy of this %(desc name)s value in %(unit name)s
	pub fn to_%(unit symbol)s(&self) -> T {
		return (self.%(si unit symbol)s.clone() * T::from(%(inverse slope)s_f64)) - T::from(%(offset)s_f64);
	}

	/// Returns a new %(desc name)s value from the given number of %(unit name)s
	///
	/// # Arguments
	/// * `%(unit symbol)s` - Any number-like type, representing a quantity of %(unit name)s
	pub fn from_%(unit symbol)s(%(unit symbol)s: T) -> Self {
		%(code name)s{%(si unit symbol)s: (%(unit symbol)s + T::from(%(offset)s_f64)) * T::from(%(slope)s_f64)}
	}
'''

TO_FROM_SLOPE_TEMPLATE = '''
	/// Returns a copy of this %(desc name)s value in %(unit name)s
	pub fn to_%(unit symbol)s(&self) -> T {
		return self.%(si unit symbol)s.clone() * T::from(%(inverse slope)s_f64);
	}

	/// Returns a new %(desc name)s value from the given number of %(unit name)s
	///
	/// # Arguments
	/// * `%(unit symbol)s` - Any number-like type, representing a quantity of %(unit name)s
	pub fn from_%(unit symbol)s(%(unit symbol)s: T) -> Self {
		%(code name)s{%(si unit symbol)s: %(unit symbol)s * T::from(%(slope)s_f64)}
	}
'''

UNIT_CONVERSION_TEMPLATE='''
// %(code left-side)s %(operator)s %(code right-side)s -> %(code result)s
/// %(capital verbing)s a %(code left-side)s by a %(code right-side)s returns a value of type %(code result)s
impl<T> std::ops::%(capital op-function)s<%(code right-side)s<T>> for %(code left-side)s<T> where T: NumLike {
	type Output = %(code result)s<T>;
	fn %(op-function)s(self, rhs: %(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: self.%(left-side symbol)s %(operator)s rhs.%(right-side symbol)s}
	}
}
/// %(capital verbing)s a %(code left-side)s by a %(code right-side)s returns a value of type %(code result)s
impl<T> std::ops::%(capital op-function)s<%(code right-side)s<T>> for &%(code left-side)s<T> where T: NumLike {
	type Output = %(code result)s<T>;
	fn %(op-function)s(self, rhs: %(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: self.%(left-side symbol)s.clone() %(operator)s rhs.%(right-side symbol)s}
	}
}
/// %(capital verbing)s a %(code left-side)s by a %(code right-side)s returns a value of type %(code result)s
impl<T> std::ops::%(capital op-function)s<&%(code right-side)s<T>> for %(code left-side)s<T> where T: NumLike {
	type Output = %(code result)s<T>;
	fn %(op-function)s(self, rhs: &%(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: self.%(left-side symbol)s %(operator)s rhs.%(right-side symbol)s.clone()}
	}
}
/// %(capital verbing)s a %(code left-side)s by a %(code right-side)s returns a value of type %(code result)s
impl<T> std::ops::%(capital op-function)s<&%(code right-side)s<T>> for &%(code left-side)s<T> where T: NumLike {
	type Output = %(code result)s<T>;
	fn %(op-function)s(self, rhs: &%(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: self.%(left-side symbol)s.clone() %(operator)s rhs.%(right-side symbol)s.clone()}
	}
}
'''

INVERSE_CONVERSION_TEMPLATE='''
// 1/%(code right-side)s -> %(code result)s
/// Dividing a scalar value by a %(code right-side)s unit value returns a value of type %(code result)s
%(config attr prefix)simpl<T> std::ops::Div<%(code right-side)s<T>> for %(scalar type)s where T: NumLike+From<%(scalar type)s> {
	type Output = %(code result)s<T>;
	fn div(self, rhs: %(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: T::from(self) / rhs.%(right-side symbol)s}
	}
}
/// Dividing a scalar value by a %(code right-side)s unit value returns a value of type %(code result)s
%(config attr prefix)simpl<T> std::ops::Div<%(code right-side)s<T>> for &%(scalar type)s where T: NumLike+From<%(scalar type)s> {
	type Output = %(code result)s<T>;
	fn div(self, rhs: %(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: T::from(self.clone()) / rhs.%(right-side symbol)s}
	}
}
/// Dividing a scalar value by a %(code right-side)s unit value returns a value of type %(code result)s
%(config attr prefix)simpl<T> std::ops::Div<&%(code right-side)s<T>> for %(scalar type)s where T: NumLike+From<%(scalar type)s> {
	type Output = %(code result)s<T>;
	fn div(self, rhs: &%(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: T::from(self) / rhs.%(right-side symbol)s.clone()}
	}
}
/// Dividing a scalar value by a %(code right-side)s unit value returns a value of type %(code result)s
%(config attr prefix)simpl<T> std::ops::Div<&%(code right-side)s<T>> for &%(scalar type)s where T: NumLike+From<%(scalar type)s> {
	type Output = %(code result)s<T>;
	fn div(self, rhs: &%(code right-side)s<T>) -> Self::Output {
		%(code result)s{%(result symbol)s: T::from(self.clone()) / rhs.%(right-side symbol)s.clone()}
	}
}
'''

INTO_UOM_TEMPLATE='''
/// Converts a %(code name)s into the equivalent [uom](https://crates.io/crates/uom) type [%(uom name)s](https://docs.rs/uom/0.34.0/uom/si/%(uom data type)s/type.%(uom name)s.html)
#[cfg(feature = "uom")]
impl<T> Into<uom::si::%(uom data type)s::%(uom name)s> for %(code name)s<T> where T: NumLike+Into<%(data type)s> {
	fn into(self) -> uom::si::%(uom data type)s::%(uom name)s {
		uom::si::%(uom data type)s::%(uom name)s::new::<uom::si::%(uom module)s::%(uom type)s>(self.%(unit symbol)s.into())
	}
}
'''

FROM_UOM_TEMPLATE='''
/// Creates a %(code name)s from the equivalent [uom](https://crates.io/crates/uom) type [%(uom name)s](https://docs.rs/uom/0.34.0/uom/si/%(uom data type)s/type.%(uom name)s.html)
#[cfg(feature = "uom")]
impl<T> From<uom::si::%(uom data type)s::%(uom name)s> for %(code name)s<T> where T: NumLike+From<%(data type)s> {
	fn from(src: uom::si::%(uom data type)s::%(uom name)s) -> Self {
		%(code name)s{%(unit symbol)s: T::from(src.value)}
	}
}
'''