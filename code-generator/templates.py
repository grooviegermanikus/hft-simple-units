MODULE_TEMPLATE='''
//! This module provides %(category)s SI units, such as %(example1)s 
//! and %(example2)s.
use std::fmt;
use super::base::*;
use super::chemical::*;
use super::electromagnetic::*;
use super::geometry::*;
use super::mechanical::*;
use super::nuclear::*;

%(content)s

'''

UNIT_STRUCT_DEFINITION_TEMPLATE='''
/// The %(desc first name)s unit type, defined as %(unit name)s in SI units
#[derive(UnitStruct, Debug, Clone)]
pub struct %(code name)s<T: NumLike>{
	pub %(unit symbol)s: T
}

impl<T> %(code name)s<T> where T: NumLike {

	/// Returns the standard unit name of this unit, eg "meters" or "hertz"
	pub fn unit_name() -> &'static str {
		return "%(unit name)s";
	}
	
	/// Returns the abbreviated name or symbol of this unit, eg "m" for meters or "Hz" for hertz
	pub fn unit_symbol() -> &'static str {
		return "%(unit symbol)s";
	}

	/// Returns a new %(desc name)s value from the given number of %(unit name)s
	///
	/// # Arguments
	/// * `%(unit symbol)s` - Any number-like type, representing a quantity of %(unit name)s
	pub fn from_%(unit symbol)s(%(unit symbol)s: T) -> Self {
		%(code name)s{%(unit symbol)s}
	}
	
	/// Returns a copy of this %(desc name)s value in %(unit name)s
	pub fn to_%(unit symbol)s(self) -> T {
		return self.%(unit symbol)s.clone();
	}
}

impl<T> fmt::Display for %(code name)s<T> where T: NumLike {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", &self.%(unit symbol)s, Self::unit_symbol())
	}
}

impl<T> %(code name)s<T> where T: NumLike+From<f64> {
	%(to-and-from)s
}
'''

UNIT_CONVERSION_TEMPLATE='''
	// %(code left-side)s %(operator)s %(code right-side)s -> %(code result)s
	// TODO: docstring
	impl<T> std::ops::%(capital op-function)s<%(code right-side)s<T>> for %(code left-side)s<T> where T: NumLike {
		type Output = %(code result)s<T>;
		fn %(op-function)s(self, rhs: %(code right-side)s<T>) -> Self::Output {
			%(code result)s{%(result symbol)s: self.%(left-side symbol)s %(operator)s rhs.%(right-side symbol)s}
		}
	}
'''
