#![allow(non_snake_case)]
#![warn(missing_docs)]
//! # Simple SI Units
//! This Rust library provides compiler-checked types for the standard set of
//! SI units, as specified by the US [National Institute of
//! Standards and Technology](https://www.nist.gov/pml/owm/metric-si/si-units)
//! (this project is not officially endorsed by NIST).
//!
//! ## What's included?
//! * Official standard SI Units
//! * Common secondary units, such as velocity
//! * Implements operators to automatically convert between units with basic
//!   arithmatic (eg distance / time = velocity)
//! * Units are templated so that you can choose whether to use `f32` or `f64` or other number-like type as your concrete number type.
//! * Optional, limited integration with [uom](https://crates.io/crates/uom)
//!
//! Since these structs use generic type templates for the internal data type, you
//! can use any number-like data type with these structs, including
//! [num_complex::Complex](https://crates.io/crates/num-complex) and
//! [num_bigfloat::BigFloat](https://crates.io/crates/num-bigfloat) (see caveat
//! section below regarding primitive types other than `f64`).
//!
//! ### Units
//! This crate provides types for the following units. Other kinds of
//! quantities not listed below (eg jolt) are beyond the scope of this crate.
//!
//! #### Base SI units (and standard unit of measure):
//! * Distance, aka Length (meters)
//! * Mass (kilogram)
//! * Time (seconds)
//! * Temperature (kelvin)
//! * Amount, aka Quantity (moles)
//! * Current (amperes)
//! * Luminosity (candela)
//!
//! #### Derived units:
//! * Angle (rad)
//! * Solid Angle (sr)
//! * Frequency (1/s, aka Hz)
//! * Area (m^2)
//! * Volume (m^3)
//! * Velocity (m/s)
//! * Acceleration (m/s^2)
//! * Force (kg.m/s^2, aka N)
//! * Pressure (N/m^2, aka Pa)
//! * Energy (kg.m^2/s^2, aka J)
//! * Coulomb (A.s, aka C)
//! * Power, aka Watt (J/s, aka W)
//! * Voltage (W/A, aka V)
//! * Resistance (V/A, aka Ohm)
//! * Conductance (1/Ohm, aka S)
//! * Capacitance (C/V)
//! * Inductance (Wb/A, aka H)
//! * Magnetic Flux (V.s, aka Wb)
//! * Magnetic Flux Density (Wb/m^2, aka T)
//! * Catalytic Activity (mol/s)
//! * Concentration (mol/m^3)
//! * Luminous Flux (cd.sr, aka lm)
//! * Illuminance (lm/m^2, aka lux)
//! * Radioactivity (1/s, aka Bq)
//! * Absorbed Dose (J/kg, aka Gy)
//! * Dose Equivalent (J/kg, aka Sv)
//!
//! ## What's NOT included?
//! * Not supporting dimensional analysis
//! * Not providing an exhaustive list of all possible unit types (but you can use
//!   this library to implement them yourself)
//! * Not supporting unusual number types (eg integers)
//! * Not aiming for full integration with [uom](https://crates.io/crates/uom)
//! 
pub use simple_si_units_macros::UnitStruct;
pub use simple_si_units_core::NumLike;

// optional supports
#[cfg(feature="serde")]
extern crate serde;
#[cfg(feature="num_bigfloat")]
extern crate num_bigfloat;
#[cfg(feature="num_complex")]
extern crate num_complex;
#[cfg(feature="astro_float")]
extern crate astro_float;

pub mod base;
pub mod chemical;
pub mod electromagnetic;
pub mod geometry;
pub mod mechanical;
pub mod nuclear;

/// Unit tests
#[cfg(test)]
mod unit_tests {
	use std::process::Output;
	use num_traits::Zero;
	use super::base::*;
	use super::chemical::*;
	use super::electromagnetic::*;
	use super::geometry::*;
	use super::mechanical::*;
	use super::nuclear::*;
	/// utility function for asserting equality of decimal values with approximations
	fn assert_approx_equal(a: f64, b: f64, sigfigs: i32) {
		if a.is_nan() {
			assert!(b.is_nan());
		} else if a.is_infinite() {
			assert!(b.is_infinite() && a.is_sign_positive() == b.is_sign_positive());
		} else if a.is_zero() {
			assert!(b.is_zero());
		} else {
			let ypsilon = 10f64.powi(-sigfigs);
			let max_delta = (a.abs() + b.abs()) * 0.5 * ypsilon;
			assert!((a - b).abs() < max_delta, "Error: {} != {} within margin of {}", a, b, max_delta);
		}
	}

	#[test]
	fn distance_units() {
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_cm(100.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_mm(1000.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_um(1e6_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_nm(1e9_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.0_f64).to_m(),
			Distance::from_pm(1e12_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1000.0_f64).to_m(),
			Distance::from_km(1.0_f64).to_m(), 9
		);
		assert_approx_equal(
			Distance::from_m(1.495979e11_f64).to_m(),
			Distance::from_au(1.0_f64).to_m(), 6
		);
		assert_approx_equal(
			Distance::from_m(9.4607e15_f64).to_m(),
			Distance::from_lyr(1.0_f64).to_m(), 4
		);
		assert_approx_equal(
			Distance::from_m(3.0857e16_f64).to_m(),
			Distance::from_parsec(1.0_f64).to_m(), 4
		);
		let _ = Distance::from_m(1.0_f64).to_m();
		let _ = Distance::from_m(1.0_f64).to_cm();
		let _ = Distance::from_m(1.0_f64).to_mm();
		let _ = Distance::from_m(1.0_f64).to_um();
		let _ = Distance::from_m(1.0_f64).to_nm();
		let _ = Distance::from_m(1.0_f64).to_pm();
		let _ = Distance::from_m(1.0_f64).to_km();
		let _ = Distance::from_m(1.0_f64).to_au();
		let _ = Distance::from_m(1.0_f64).to_lyr();
		let _ = Distance::from_m(1.0_f64).to_parsec();
	}

	#[test]
	fn mass_units() {
		assert_approx_equal(
			Mass::from_kg(1.0_f64).to_kg(),
			Mass::from_g(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_g(1.0_f64).to_kg(),
			Mass::from_mg(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_mg(1.0_f64).to_kg(),
			Mass::from_ug(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_ug(1.0_f64).to_kg(),
			Mass::from_ng(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_ng(1.0_f64).to_kg(),
			Mass::from_pg(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_tons(1.0_f64).to_kg(),
			Mass::from_kg(1000.0_f64).to_kg(), 9
		);
		assert_approx_equal(
			Mass::from_earth_mass(1.0_f64).to_kg(),
			Mass::from_kg(5.972e24_f64).to_kg(), 3
		);
		assert_approx_equal(
			Mass::from_jupiter_mass(1.0_f64).to_kg(),
			Mass::from_kg(1.898e27_f64).to_kg(), 3
		);
		assert_approx_equal(
			Mass::from_solar_mass(1.0_f64).to_kg(),
			Mass::from_kg(1.988e30_f64).to_kg(), 3
		);
		let _ = Mass::from_kg(1.0_f64).to_kg();
		let _ = Mass::from_kg(1.0_f64).to_g();
		let _ = Mass::from_kg(1.0_f64).to_mg();
		let _ = Mass::from_kg(1.0_f64).to_ug();
		let _ = Mass::from_kg(1.0_f64).to_ng();
		let _ = Mass::from_kg(1.0_f64).to_pg();
		let _ = Mass::from_kg(1.0_f64).to_tons();
		let _ = Mass::from_kg(1.0_f64).to_earth_mass();
		let _ = Mass::from_kg(1.0_f64).to_jupiter_mass();
		let _ = Mass::from_kg(1.0_f64).to_solar_mass();
	}
	#[test]
	fn time_units() {
		assert_approx_equal(
			Time::from_s(1.0_f64).to_s(),
			Time::from_ms(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_ms(1.0_f64).to_s(),
			Time::from_us(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_us(1.0_f64).to_s(),
			Time::from_ns(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_ns(1.0_f64).to_s(),
			Time::from_ps(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_min(1.0_f64).to_s(),
			Time::from_s(60.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_hr(1.0_f64).to_s(),
			Time::from_min(60.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_days(1.0_f64).to_s(),
			Time::from_hr(24.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_weeks(1.0_f64).to_s(),
			Time::from_days(7.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_yr(1.0_f64).to_s(),
			Time::from_days(365.2425_f64).to_s(), 6
		);
		assert_approx_equal(
			Time::from_kyr(1.0_f64).to_s(),
			Time::from_yr(1000.0_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_Myr(1.0_f64).to_s(),
			Time::from_yr(1e6_f64).to_s(), 9
		);
		assert_approx_equal(
			Time::from_Gyr(1.0_f64).to_s(),
			Time::from_yr(1e9_f64).to_s(), 9
		);
		let _ = Time::from_s(1.0_f64).to_s();
		let _ = Time::from_s(1.0_f64).to_ms();
		let _ = Time::from_s(1.0_f64).to_us();
		let _ = Time::from_s(1.0_f64).to_ns();
		let _ = Time::from_s(1.0_f64).to_ps();
		let _ = Time::from_s(1.0_f64).to_min();
		let _ = Time::from_s(1.0_f64).to_hr();
		let _ = Time::from_s(1.0_f64).to_days();
		let _ = Time::from_s(1.0_f64).to_weeks();
		let _ = Time::from_s(1.0_f64).to_yr();
		let _ = Time::from_s(1.0_f64).to_kyr();
		let _ = Time::from_s(1.0_f64).to_Myr();
		let _ = Time::from_s(1.0_f64).to_Gyr();
	}
	#[test]
	fn temperature_units() {
		assert_approx_equal(
			Temperature::from_K(273.15_f64).to_K(),
			Temperature::from_C(0.0_f64).to_K(), 9
		);
		assert_approx_equal(
			Temperature::from_C(0.0_f64).to_K(),
			Temperature::from_F(32_f64).to_K(), 3
		);
		let _ = Temperature::from_K(300_f64).to_K();
		let _ = Temperature::from_K(300_f64).to_C();
		let _ = Temperature::from_K(300_f64).to_F();
	}
	#[test]
	fn quantity_units() {
		assert_approx_equal(
			Amount::from_count(6.0221415e23_f64).to_count(),
			Amount::from_mol(1.0_f64).to_count(), 6
		);
		assert_approx_equal(
			Amount::from_mol(1.0_f64).to_count(),
			Amount::from_mmol(1000.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Amount::from_mmol(1.0_f64).to_count(),
			Amount::from_umol(1000.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Amount::from_umol(1.0_f64).to_count(),
			Amount::from_nmol(1000.0_f64).to_count(), 9
		);
		assert_approx_equal(
			Amount::from_nmol(1.0_f64).to_count(),
			Amount::from_pmol(1000.0_f64).to_count(), 9
		);
		let _ = Amount::from_mol(1.0_f64).to_mol();
		let _ = Amount::from_mol(1.0_f64).to_mmol();
		let _ = Amount::from_mol(1.0_f64).to_umol();
		let _ = Amount::from_mol(1.0_f64).to_nmol();
		let _ = Amount::from_mol(1.0_f64).to_pmol();
	}
	#[test]
	fn current_units() {
		assert_approx_equal(
			Current::from_A(1.0_f64).to_A(),
			Current::from_mA(1000.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_mA(1.0_f64).to_A(),
			Current::from_uA(1000.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_uA(1.0_f64).to_A(),
			Current::from_nA(1000.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_A(1000.0_f64).to_A(),
			Current::from_kA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_kA(1000.0_f64).to_A(),
			Current::from_MA(1.0_f64).to_A(), 9
		);
		assert_approx_equal(
			Current::from_MA(1000.0_f64).to_A(),
			Current::from_GA(1.0_f64).to_A(), 9
		);
		let _ = Current::from_A(1.0_f64).to_A();
		let _ = Current::from_A(1.0_f64).to_mA();
		let _ = Current::from_A(1.0_f64).to_uA();
		let _ = Current::from_A(1.0_f64).to_nA();
		let _ = Current::from_A(1.0_f64).to_kA();
		let _ = Current::from_A(1.0_f64).to_MA();
		let _ = Current::from_A(1.0_f64).to_GA();
	}
	#[test]
	fn luminosity_units() {
		assert_approx_equal(
			Luminosity::from_cd(1.0_f64).to_cd(),
			Luminosity::from_mcd(1000.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_mcd(1.0_f64).to_cd(),
			Luminosity::from_ucd(1000.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_ucd(1.0_f64).to_cd(),
			Luminosity::from_ncd(1000.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_cd(1000.0_f64).to_cd(),
			Luminosity::from_kcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_kcd(1000.0_f64).to_cd(),
			Luminosity::from_Mcd(1.0_f64).to_cd(), 9
		);
		assert_approx_equal(
			Luminosity::from_Mcd(1000.0_f64).to_cd(),
			Luminosity::from_Gcd(1.0_f64).to_cd(), 9
		);
		let _ = Luminosity::from_cd(1.0_f64).to_cd();
		let _ = Luminosity::from_cd(1.0_f64).to_mcd();
		let _ = Luminosity::from_cd(1.0_f64).to_ucd();
		let _ = Luminosity::from_cd(1.0_f64).to_ncd();
		let _ = Luminosity::from_cd(1.0_f64).to_kcd();
		let _ = Luminosity::from_cd(1.0_f64).to_Mcd();
		let _ = Luminosity::from_cd(1.0_f64).to_Gcd();
	}

	#[test]
	fn angle_units() {
		assert_approx_equal(
			Angle::from_deg(360.0_f64).to_rad(),
			Angle::from_rad(6.283185307179586_f64).to_rad(), 9
		);
		let _ = Angle::from_deg(360.0_f64).to_rad();
		let _ = Angle::from_deg(360.0_f64).to_deg();
	}
	#[test]
	fn solid_angle_units() {
		let _ = SolidAngle::from_sr(1.0_f64).to_sr();
	}
	#[test]
	fn angular_velocity_units() {
		assert_approx_equal(
			AngularVelocity::from_radps(6.283185307179586_f64 / 60.0_f64).to_radps(),
			AngularVelocity::from_rpm(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_radps(6.283185307179586_f64).to_radps(),
			AngularVelocity::from_rps(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_degps(360.0_f64 / 60.0_f64).to_radps(),
			AngularVelocity::from_rpm(1.0_f64).to_radps(), 9
		);
		assert_approx_equal(
			AngularVelocity::from_degps(360.0_f64).to_radps(),
			AngularVelocity::from_rps(1.0_f64).to_radps(), 9
		);
		let _ = AngularVelocity::from_radps(1.0_f64).to_radps();
		let _ = AngularVelocity::from_radps(1.0_f64).to_degps();
		let _ = AngularVelocity::from_radps(1.0_f64).to_rpm();
		let _ = AngularVelocity::from_radps(1.0_f64).to_rps();
	}
	#[test]
	fn angular_acceleration_units() {
		assert_approx_equal(
			AngularAcceleration::from_radps2(6.283185307179586_f64 / 60.0_f64 / 60.0_f64).to_radps2(),
			AngularAcceleration::from_rpm2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_radps2(6.283185307179586_f64).to_radps2(),
			AngularAcceleration::from_rps2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_degps2(360.0_f64 / 60.0_f64 / 60.0_f64).to_radps2(),
			AngularAcceleration::from_rpm2(1.0_f64).to_radps2(), 9
		);
		assert_approx_equal(
			AngularAcceleration::from_degps2(360.0_f64).to_radps2(),
			AngularAcceleration::from_rps2(1.0_f64).to_radps2(), 9
		);
		let _ = AngularAcceleration::from_radps2(1.0_f64).to_radps2();
		let _ = AngularAcceleration::from_radps2(1.0_f64).to_degps2();
		let _ = AngularAcceleration::from_radps2(1.0_f64).to_rpm2();
		let _ = AngularAcceleration::from_radps2(1.0_f64).to_rps2();}
	#[test]
	fn moment_of_inertia_units() {
		assert_approx_equal(
			MomentOfInertia::from_kgm2(1.0_f64).to_kgm2(),
			MomentOfInertia::from_gm2(1000.0_f64).to_kgm2(), 9
		);
		assert_approx_equal(
			MomentOfInertia::from_kgm2(1.0_f64).to_kgm2(),
			MomentOfInertia::from_gcm2(0.1_f64).to_kgm2(), 9
		);
		let _ = MomentOfInertia::from_kgm2(1.0_f64).to_kgm2();
		let _ = MomentOfInertia::from_kgm2(1.0_f64).to_gm2();
		let _ = MomentOfInertia::from_kgm2(1.0_f64).to_gcm2();
	}
	#[test]
	fn torque_units() {
		assert_approx_equal(
			Torque::from_Nm(1.3558179483_f64).to_Nm(),
			Torque::from_ftlb(1.0_f64).to_Nm(), 3
		);
		let _ = Torque::from_Nm(1.0_f64).to_Nm();
		let _ = Torque::from_Nm(1.0_f64).to_ftlb();
	}
	#[test]
	fn momentum_units() {
		assert_approx_equal(
			Momentum::from_kgmps(1.0_f64).to_kgmps(),
			Momentum::from_gcmps(100000.0_f64).to_kgmps(), 9
		);
		let _ = Momentum::from_kgmps(1.0_f64).to_kgmps();
		let _ = Momentum::from_kgmps(1.0_f64).to_gcmps();
	}
	#[test]
	fn angular_momentum_units() {
		// only one unit of measure supported
		let _ = AngularMomentum::from_kgm2radps(1.0_f64).to_kgm2radps();
	}
	#[test]
	fn frequency_units() {
		assert_approx_equal(
			Frequency::from_kHz(1.0_f64).to_Hz(),
			Frequency::from_Hz(1000.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_MHz(1.0_f64).to_Hz(),
			Frequency::from_kHz(1000.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_GHz(1.0_f64).to_Hz(),
			Frequency::from_MHz(1000.0_f64).to_Hz(), 9
		);
		assert_approx_equal(
			Frequency::from_THz(1.0_f64).to_Hz(),
			Frequency::from_GHz(1000.0_f64).to_Hz(), 9
		);
		let _ = Frequency::from_Hz(1.0_f64).to_Hz();
		let _ = Frequency::from_Hz(1.0_f64).to_kHz();
		let _ = Frequency::from_Hz(1.0_f64).to_MHz();
		let _ = Frequency::from_Hz(1.0_f64).to_GHz();
		let _ = Frequency::from_Hz(1.0_f64).to_THz();
	}
	#[test]
	fn area_units() {
		assert_approx_equal(
			Area::from_m2(1.0_f64).to_m2(),
			Area::from_cm2(10000.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_cm2(1.0_f64).to_m2(),
			Area::from_mm2(100.0_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_mm2(1.0_f64).to_m2(),
			Area::from_um2(1e6_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_um2(1.0_f64).to_m2(),
			Area::from_nm2(1e6_f64).to_m2(), 9
		);
		assert_approx_equal(
			Area::from_km2(1.0_f64).to_m2(),
			Area::from_m2(1e6_f64).to_m2(), 9
		);
		let _ = Area::from_m2(1.0_f64).to_km2();
		let _ = Area::from_m2(1.0_f64).to_m2();
		let _ = Area::from_m2(1.0_f64).to_cm2();
		let _ = Area::from_m2(1.0_f64).to_mm2();
		let _ = Area::from_m2(1.0_f64).to_um2();
		let _ = Area::from_m2(1.0_f64).to_nm2();
	}
	#[test]
	fn area_density_units() {
		assert_approx_equal(
			AreaDensity::from_kgpm2(1.0_f64).to_kgpm2(),
			AreaDensity::from_gpm2(1000.0_f64).to_kgpm2(), 9
		);
		assert_approx_equal(
			AreaDensity::from_kgpm2(1.0_f64).to_kgpm2(),
			AreaDensity::from_gpcm2(0.1_f64).to_kgpm2(), 9
		);
		let _ = AreaDensity::from_kgpm2(1.0_f64).to_kgpm2();
		let _ = AreaDensity::from_kgpm2(1.0_f64).to_gpm2();
		let _ = AreaDensity::from_kgpm2(1.0_f64).to_gpcm2();
	}
	#[test]
	fn volume_units() {
		assert_approx_equal(
			Volume::from_L(1.0_f64).to_L(),
			Volume::from_mL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_mL(1.0_f64).to_L(),
			Volume::from_uL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_uL(1.0_f64).to_L(),
			Volume::from_nL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_nL(1.0_f64).to_L(),
			Volume::from_pL(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_L(1.0_f64).to_L(),
			Volume::from_cc(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_m3(1.0_f64).to_L(),
			Volume::from_L(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_kL(1.0_f64).to_L(),
			Volume::from_L(1000.0_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_ML(1.0_f64).to_L(),
			Volume::from_L(1e6_f64).to_L(), 9
		);
		assert_approx_equal(
			Volume::from_GL(1.0_f64).to_L(),
			Volume::from_L(1e9_f64).to_L(), 9
		);
		let _ = Volume::from_L(1.0_f64).to_L();
		let _ = Volume::from_L(1.0_f64).to_mL();
		let _ = Volume::from_L(1.0_f64).to_uL();
		let _ = Volume::from_L(1.0_f64).to_nL();
		let _ = Volume::from_L(1.0_f64).to_pL();
		let _ = Volume::from_L(1.0_f64).to_cc();
		let _ = Volume::from_L(1.0_f64).to_m3();
		let _ = Volume::from_L(1.0_f64).to_kL();
		let _ = Volume::from_L(1.0_f64).to_ML();
		let _ = Volume::from_L(1.0_f64).to_GL();
	}

	#[test]
	fn density_units() {
		assert_approx_equal(
			Density::from_kgpL(1.0_f64).to_kgpL(),
			Density::from_kgpm3(1000.0_f64).to_kgpL(), 9
		);
		assert_approx_equal(
			Density::from_kgpL(1.0_f64).to_kgpL(),
			Density::from_gpcc(1.0_f64).to_kgpL(), 9
		);
		assert_approx_equal(
			Density::from_kgpm3(1.0_f64).to_kgpL(),
			Density::from_gpm3(1000.0_f64).to_kgpL(), 9
		);
		let _ = Density::from_kgpL(1.0_f64).to_kgpL();
		let _ = Density::from_kgpL(1.0_f64).to_kgpm3();
		let _ = Density::from_kgpL(1.0_f64).to_gpcc();
		let _ = Density::from_kgpL(1.0_f64).to_gpm3();
	}
	#[test]
	fn velocity_units() {
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps(),
			Velocity::from_cmps(100.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps(),
			Velocity::from_mmps(1000.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_mps(1.0_f64).to_mps(),
			Velocity::from_mmph(1000.0_f64 * 3600.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_kph(1.609344_f64).to_mps(),
			Velocity::from_mph(1.0_f64).to_mps(), 5
		);
		assert_approx_equal(
			Velocity::from_kph(1.0_f64).to_mps(),
			Velocity::from_mps(1000.0_f64 / 3600.0_f64).to_mps(), 9
		);
		assert_approx_equal(
			Velocity::from_c(1.0_f64).to_mps(),
			Velocity::from_mps(299792458_f64).to_mps(), 8
		);
		let _ = Velocity::from_mps(1.0_f64).to_mps();
		let _ = Velocity::from_mps(1.0_f64).to_cmps();
		let _ = Velocity::from_mps(1.0_f64).to_mmps();
		let _ = Velocity::from_mps(1.0_f64).to_mph();
		let _ = Velocity::from_mps(1.0_f64).to_kph();
		let _ = Velocity::from_mps(1.0_f64).to_c();
	}
	#[test]
	fn acceleration_units() {
		assert_approx_equal(
			Acceleration::from_mps2(1.0_f64).to_mps2(),
			Acceleration::from_mmps2(1000.0_f64).to_mps2(), 9
		);
		let _ = Acceleration::from_mps2(1.0_f64).to_mps2();
		let _ = Acceleration::from_mps2(1.0_f64).to_mmps2();
	}
	#[test]
	fn force_units() {
		assert_approx_equal(
			Force::from_N(1.0_f64).to_N(),
			Force::from_mN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_kgG(1.0_f64).to_N(),
			Force::from_N(9.80665_f64).to_N(), 4
		);
		assert_approx_equal(
			Force::from_kgG(1.0_f64).to_N(),
			Force::from_lb(2.2_f64).to_N(), 4
		);
		assert_approx_equal(
			Force::from_mN(1.0_f64).to_N(),
			Force::from_uN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_uN(1.0_f64).to_N(),
			Force::from_nN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_kN(1.0_f64).to_N(),
			Force::from_N(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_MN(1.0_f64).to_N(),
			Force::from_kN(1000.0_f64).to_N(), 9
		);
		assert_approx_equal(
			Force::from_GN(1.0_f64).to_N(),
			Force::from_MN(1000.0_f64).to_N(), 9
		);
		let _ = Force::from_N(1.0_f64).to_N();
		let _ = Force::from_N(1.0_f64).to_kgG();
		let _ = Force::from_N(1.0_f64).to_lb();
		let _ = Force::from_N(1.0_f64).to_mN();
		let _ = Force::from_N(1.0_f64).to_uN();
		let _ = Force::from_N(1.0_f64).to_nN();
		let _ = Force::from_N(1.0_f64).to_kN();
		let _ = Force::from_N(1.0_f64).to_MN();
		let _ = Force::from_N(1.0_f64).to_GN();
	}
	#[test]
	fn pressure_units() {
		assert_approx_equal(
			Pressure::from_Pa(1000.0_f64).to_Pa(),
			Pressure::from_kPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_hPa(10.0_f64).to_Pa(),
			Pressure::from_kPa(1.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_bar(1.0_f64).to_Pa(),
			Pressure::from_kPa(100.0_f64).to_Pa(), 9
		);
		assert_approx_equal(
			Pressure::from_atm(1.0_f64).to_Pa(),
			Pressure::from_kPa(101.325_f64).to_Pa(), 3
		);
		assert_approx_equal(
			Pressure::from_atm(1.0_f64).to_Pa(),
			Pressure::from_mmHg(760_f64).to_Pa(), 3
		);
		assert_approx_equal(
			Pressure::from_psi(1.0_f64).to_Pa(),
			Pressure::from_Pa(6894.757_f64).to_Pa(), 5
		);
		let _ = Pressure::from_Pa(1000.0_f64).to_Pa();
		let _ = Pressure::from_Pa(1000.0_f64).to_kPa();
		let _ = Pressure::from_Pa(1000.0_f64).to_hPa();
		let _ = Pressure::from_Pa(1000.0_f64).to_bar();
		let _ = Pressure::from_Pa(1000.0_f64).to_atm();
		let _ = Pressure::from_Pa(1000.0_f64).to_mmHg();
		let _ = Pressure::from_Pa(1000.0_f64).to_psi();
	}
	#[test]
	fn energy_units() {
		assert_approx_equal(
			Energy::from_J(1000.0_f64).to_J(),
			Energy::from_kJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_kcal(1.0_f64).to_J(),
			Energy::from_kJ(4.184_f64).to_J(), 3
		);
		assert_approx_equal(
			Energy::from_BTU(1.0_f64).to_J(),
			Energy::from_kJ(1.055_f64).to_J(), 3
		);
		assert_approx_equal(
			Energy::from_J(1.0_f64).to_J(),
			Energy::from_mJ(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_mJ(1.0_f64).to_J(),
			Energy::from_uJ(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_uJ(1.0_f64).to_J(),
			Energy::from_nJ(1000.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_kJ(1000.0_f64).to_J(),
			Energy::from_MJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_MJ(1000.0_f64).to_J(),
			Energy::from_GJ(1.0_f64).to_J(), 9
		);
		assert_approx_equal(
			Energy::from_eV(1.0_f64).to_J(),
			Energy::from_J(1.602177e-19_f64).to_J(), 5
		);
		assert_approx_equal(
			Energy::from_Whr(1.0_f64).to_J(),
			Energy::from_J(3600.0_f64).to_J(), 5
		);
		assert_approx_equal(
			Energy::from_kWhr(1.0_f64).to_J(),
			Energy::from_Whr(1000.0_f64).to_J(), 5
		);
		let _ = Energy::from_J(1.0_f64).to_J();
		let _ = Energy::from_J(1.0_f64).to_mJ();
		let _ = Energy::from_J(1.0_f64).to_uJ();
		let _ = Energy::from_J(1.0_f64).to_nJ();
		let _ = Energy::from_J(1.0_f64).to_kJ();
		let _ = Energy::from_J(1.0_f64).to_MJ();
		let _ = Energy::from_J(1.0_f64).to_GJ();
		let _ = Energy::from_J(1.0_f64).to_eV();
		let _ = Energy::from_J(1.0_f64).to_Whr();
		let _ = Energy::from_J(1.0_f64).to_kWhr();
	}
	#[test]
	fn coulomb_units() {
		assert_approx_equal(
			Charge::from_C(-1.60217646e-19_f64).to_C(),
			Charge::from_e(1.0_f64).to_C(), 5
		);
		assert_approx_equal(
			Charge::from_C(1.60217646e-19_f64).to_C(),
			Charge::from_p(1.0_f64).to_C(), 5
		);
		let _ = Charge::from_C(1.0_f64).to_C();
		let _ = Charge::from_C(1.0_f64).to_e();
		let _ = Charge::from_C(1.0_f64).to_p();
	}
	#[test]
	fn power_units() {
		assert_approx_equal(
			Power::from_W(1.0_f64).to_W(),
			Power::from_mW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_mW(1.0_f64).to_W(),
			Power::from_uW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_uW(1.0_f64).to_W(),
			Power::from_nW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_kW(1.0_f64).to_W(),
			Power::from_W(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_MW(1.0_f64).to_W(),
			Power::from_kW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_GW(1.0_f64).to_W(),
			Power::from_MW(1000.0_f64).to_W(), 9
		);
		assert_approx_equal(
			Power::from_horsepower(1.0_f64).to_W(),
			Power::from_W(745.70_f64).to_W(), 4
		);
		let _ = Power::from_W(1.0_f64).to_W();
		let _ = Power::from_W(1.0_f64).to_horsepower();
		let _ = Power::from_W(1.0_f64).to_mW();
		let _ = Power::from_W(1.0_f64).to_uW();
		let _ = Power::from_W(1.0_f64).to_nW();
		let _ = Power::from_W(1.0_f64).to_kW();
		let _ = Power::from_W(1.0_f64).to_MW();
		let _ = Power::from_W(1.0_f64).to_GW();
	}
	#[test]
	fn voltage_units() {
		assert_approx_equal(
			Voltage::from_V(1.0_f64).to_V(),
			Voltage::from_mV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_mV(1.0_f64).to_V(),
			Voltage::from_uV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_uV(1.0_f64).to_V(),
			Voltage::from_nV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_kV(1.0_f64).to_V(),
			Voltage::from_V(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_MV(1.0_f64).to_V(),
			Voltage::from_kV(1000.0_f64).to_V(), 9
		);
		assert_approx_equal(
			Voltage::from_GV(1.0_f64).to_V(),
			Voltage::from_MV(1000.0_f64).to_V(), 9
		);
		let _ = Voltage::from_V(1.0_f64).to_V();
		let _ = Voltage::from_V(1.0_f64).to_mV();
		let _ = Voltage::from_V(1.0_f64).to_uV();
		let _ = Voltage::from_V(1.0_f64).to_nV();
		let _ = Voltage::from_V(1.0_f64).to_kV();
		let _ = Voltage::from_V(1.0_f64).to_MV();
		let _ = Voltage::from_V(1.0_f64).to_GV();
	}
	#[test]
	fn resistance_units() {
		assert_approx_equal(
			Resistance::from_Ohm(1.0_f64).to_Ohm(),
			Resistance::from_mOhm(1000.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_mOhm(1.0_f64).to_Ohm(),
			Resistance::from_uOhm(1000.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_uOhm(1.0_f64).to_Ohm(),
			Resistance::from_nOhm(1000.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_kOhm(1.0_f64).to_Ohm(),
			Resistance::from_Ohm(1000.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_MOhm(1.0_f64).to_Ohm(),
			Resistance::from_kOhm(1000.0_f64).to_Ohm(), 9
		);
		assert_approx_equal(
			Resistance::from_GOhm(1.0_f64).to_Ohm(),
			Resistance::from_MOhm(1000.0_f64).to_Ohm(), 9
		);
		let _ = Resistance::from_Ohm(1.0_f64).to_Ohm();
		let _ = Resistance::from_Ohm(1.0_f64).to_mOhm();
		let _ = Resistance::from_Ohm(1.0_f64).to_uOhm();
		let _ = Resistance::from_Ohm(1.0_f64).to_nOhm();
		let _ = Resistance::from_Ohm(1.0_f64).to_kOhm();
		let _ = Resistance::from_Ohm(1.0_f64).to_MOhm();
		let _ = Resistance::from_Ohm(1.0_f64).to_GOhm();
	}
	#[test]
	fn conductance_units() {
		assert_approx_equal(
			Conductance::from_S(1.0_f64).to_S(),
			Conductance::from_mS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_mS(1.0_f64).to_S(),
			Conductance::from_uS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_uS(1.0_f64).to_S(),
			Conductance::from_nS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_kS(1.0_f64).to_S(),
			Conductance::from_S(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_MS(1.0_f64).to_S(),
			Conductance::from_kS(1000.0_f64).to_S(), 9
		);
		assert_approx_equal(
			Conductance::from_GS(1.0_f64).to_S(),
			Conductance::from_MS(1000.0_f64).to_S(), 9
		);
		let _ = Conductance::from_S(1.0_f64).to_S();
		let _ = Conductance::from_S(1.0_f64).to_mS();
		let _ = Conductance::from_S(1.0_f64).to_uS();
		let _ = Conductance::from_S(1.0_f64).to_nS();
		let _ = Conductance::from_S(1.0_f64).to_kS();
		let _ = Conductance::from_S(1.0_f64).to_MS();
		let _ = Conductance::from_S(1.0_f64).to_GS();
	}
	#[test]
	fn capacitance_units() {
		assert_approx_equal(
			Capacitance::from_F(1.0_f64).to_F(),
			Capacitance::from_mF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_mF(1.0_f64).to_F(),
			Capacitance::from_uF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_uF(1.0_f64).to_F(),
			Capacitance::from_nF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_kF(1.0_f64).to_F(),
			Capacitance::from_F(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_MF(1.0_f64).to_F(),
			Capacitance::from_kF(1000.0_f64).to_F(), 9
		);
		assert_approx_equal(
			Capacitance::from_GF(1.0_f64).to_F(),
			Capacitance::from_MF(1000.0_f64).to_F(), 9
		);
		let _ = Capacitance::from_F(1.0_f64).to_F();
		let _ = Capacitance::from_F(1.0_f64).to_mF();
		let _ = Capacitance::from_F(1.0_f64).to_uF();
		let _ = Capacitance::from_F(1.0_f64).to_nF();
		let _ = Capacitance::from_F(1.0_f64).to_kF();
		let _ = Capacitance::from_F(1.0_f64).to_MF();
		let _ = Capacitance::from_F(1.0_f64).to_GF();
	}
	#[test]
	fn inductance_units() {
		assert_approx_equal(
			Inductance::from_H(1.0_f64).to_H(),
			Inductance::from_mH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_mH(1.0_f64).to_H(),
			Inductance::from_uH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_uH(1.0_f64).to_H(),
			Inductance::from_nH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_kH(1.0_f64).to_H(),
			Inductance::from_H(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_MH(1.0_f64).to_H(),
			Inductance::from_kH(1000.0_f64).to_H(), 9
		);
		assert_approx_equal(
			Inductance::from_GH(1.0_f64).to_H(),
			Inductance::from_MH(1000.0_f64).to_H(), 9
		);
		let _ = Inductance::from_H(1.0_f64).to_H();
		let _ = Inductance::from_H(1.0_f64).to_mH();
		let _ = Inductance::from_H(1.0_f64).to_uH();
		let _ = Inductance::from_H(1.0_f64).to_nH();
		let _ = Inductance::from_H(1.0_f64).to_kH();
		let _ = Inductance::from_H(1.0_f64).to_MH();
		let _ = Inductance::from_H(1.0_f64).to_GH();
	}
	#[test]
	fn magnetic_flux_units() {
		assert_approx_equal(
			MagneticFlux::from_Wb(1.0_f64).to_Wb(),
			MagneticFlux::from_mWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_mWb(1.0_f64).to_Wb(),
			MagneticFlux::from_uWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_uWb(1.0_f64).to_Wb(),
			MagneticFlux::from_nWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_kWb(1.0_f64).to_Wb(),
			MagneticFlux::from_Wb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_MWb(1.0_f64).to_Wb(),
			MagneticFlux::from_kWb(1000.0_f64).to_Wb(), 9
		);
		assert_approx_equal(
			MagneticFlux::from_GWb(1.0_f64).to_Wb(),
			MagneticFlux::from_MWb(1000.0_f64).to_Wb(), 9
		);
		let _ = MagneticFlux::from_Wb(1.0_f64).to_Wb();
		let _ = MagneticFlux::from_Wb(1.0_f64).to_mWb();
		let _ = MagneticFlux::from_Wb(1.0_f64).to_uWb();
		let _ = MagneticFlux::from_Wb(1.0_f64).to_nWb();
		let _ = MagneticFlux::from_Wb(1.0_f64).to_kWb();
		let _ = MagneticFlux::from_Wb(1.0_f64).to_MWb();
		let _ = MagneticFlux::from_Wb(1.0_f64).to_GWb();
	}
	#[test]
	fn magnetic_flux_density_units() {
		assert_approx_equal(
			MagneticFluxDensity::from_T(1.0_f64).to_T(),
			MagneticFluxDensity::from_mT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_mT(1.0_f64).to_T(),
			MagneticFluxDensity::from_uT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_uT(1.0_f64).to_T(),
			MagneticFluxDensity::from_nT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_kT(1.0_f64).to_T(),
			MagneticFluxDensity::from_T(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_MT(1.0_f64).to_T(),
			MagneticFluxDensity::from_kT(1000.0_f64).to_T(), 9
		);
		assert_approx_equal(
			MagneticFluxDensity::from_GT(1.0_f64).to_T(),
			MagneticFluxDensity::from_MT(1000.0_f64).to_T(), 9
		);
		let _ = MagneticFluxDensity::from_T(1.0_f64).to_T();
		let _ = MagneticFluxDensity::from_T(1.0_f64).to_mT();
		let _ = MagneticFluxDensity::from_T(1.0_f64).to_uT();
		let _ = MagneticFluxDensity::from_T(1.0_f64).to_nT();
		let _ = MagneticFluxDensity::from_T(1.0_f64).to_kT();
		let _ = MagneticFluxDensity::from_T(1.0_f64).to_MT();
		let _ = MagneticFluxDensity::from_T(1.0_f64).to_GT();
	}
	#[test]
	fn catalytic_activity_units() {
		assert_approx_equal(
			CatalyticActivity::from_molps(1.0_f64).to_molps(),
			CatalyticActivity::from_mmolps(1000.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_molps(1.0_f64).to_molps(),
			CatalyticActivity::from_Nps(6.0221415e23_f64).to_molps(), 4
		);
		assert_approx_equal(
			CatalyticActivity::from_mmolps(1.0_f64).to_molps(),
			CatalyticActivity::from_umolps(1000.0_f64).to_molps(), 9
		);
		assert_approx_equal(
			CatalyticActivity::from_umolps(1.0_f64).to_molps(),
			CatalyticActivity::from_nmolps(1000.0_f64).to_molps(), 9
		);
		let _ = CatalyticActivity::from_molps(1.0_f64).to_Nps();
		let _ = CatalyticActivity::from_molps(1.0_f64).to_molps();
		let _ = CatalyticActivity::from_molps(1.0_f64).to_mmolps();
		let _ = CatalyticActivity::from_molps(1.0_f64).to_umolps();
		let _ = CatalyticActivity::from_molps(1.0_f64).to_nmolps();
	}
	#[test]
	fn concentration_units() {
		assert_approx_equal(
			Concentration::from_M(1.0_f64).to_M(),
			Concentration::from_mM(1000.0_f64).to_M(), 9
		);
		assert_approx_equal(
			Concentration::from_mM(1.0_f64).to_M(),
			Concentration::from_uM(1000.0_f64).to_M(), 9
		);
		assert_approx_equal(
			Concentration::from_uM(1.0_f64).to_M(),
			Concentration::from_nM(1000.0_f64).to_M(), 9
		);
		assert_approx_equal(
			Concentration::from_count_per_L(6.02214e23_f64).to_M(),
			Concentration::from_M(1.0_f64).to_M(), 4
		);
		assert_approx_equal(
			Concentration::from_Npm3(6.02214e23_f64).to_M(),
			Concentration::from_mM(1.0_f64).to_M(), 4
		);
		assert_approx_equal(
			Concentration::from_count_per_cc(6.02214e23_f64).to_M(),
			Concentration::from_M(1000.0_f64).to_M(), 5
		);
		let _ = Concentration::from_M(1.0_f64).to_M();
		let _ = Concentration::from_M(1.0_f64).to_mM();
		let _ = Concentration::from_M(1.0_f64).to_uM();
		let _ = Concentration::from_M(1.0_f64).to_nM();
		let _ = Concentration::from_M(1.0_f64).to_count_per_cc();
		let _ = Concentration::from_M(1.0_f64).to_Npm3();
	}
	#[test]
	fn luminous_flux_units() {
		assert_approx_equal(
			LuminousFlux::from_lm(1.0_f64).to_lm(),
			LuminousFlux::from_mlm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_mlm(1.0_f64).to_lm(),
			LuminousFlux::from_ulm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_ulm(1.0_f64).to_lm(),
			LuminousFlux::from_nlm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_klm(1.0_f64).to_lm(),
			LuminousFlux::from_lm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_Mlm(1.0_f64).to_lm(),
			LuminousFlux::from_klm(1000.0_f64).to_lm(), 9
		);
		assert_approx_equal(
			LuminousFlux::from_Glm(1.0_f64).to_lm(),
			LuminousFlux::from_Mlm(1000.0_f64).to_lm(), 9
		);
		let _ = LuminousFlux::from_lm(1.0_f64).to_lm();
		let _ = LuminousFlux::from_lm(1.0_f64).to_mlm();
		let _ = LuminousFlux::from_lm(1.0_f64).to_ulm();
		let _ = LuminousFlux::from_lm(1.0_f64).to_nlm();
		let _ = LuminousFlux::from_lm(1.0_f64).to_klm();
		let _ = LuminousFlux::from_lm(1.0_f64).to_Mlm();
		let _ = LuminousFlux::from_lm(1.0_f64).to_Glm();
	}
	#[test]
	fn illuminance_units() {
		assert_approx_equal(
			Illuminance::from_lux(1.0_f64).to_lux(),
			Illuminance::from_mlux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_mlux(1.0_f64).to_lux(),
			Illuminance::from_ulux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_ulux(1.0_f64).to_lux(),
			Illuminance::from_nlux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_klux(1.0_f64).to_lux(),
			Illuminance::from_lux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_Mlux(1.0_f64).to_lux(),
			Illuminance::from_klux(1000.0_f64).to_lux(), 9
		);
		assert_approx_equal(
			Illuminance::from_Glux(1.0_f64).to_lux(),
			Illuminance::from_Mlux(1000.0_f64).to_lux(), 9
		);
		let _ = Illuminance::from_lux(1.0_f64).to_lux();
		let _ = Illuminance::from_lux(1.0_f64).to_mlux();
		let _ = Illuminance::from_lux(1.0_f64).to_ulux();
		let _ = Illuminance::from_lux(1.0_f64).to_nlux();
		let _ = Illuminance::from_lux(1.0_f64).to_klux();
		let _ = Illuminance::from_lux(1.0_f64).to_Mlux();
		let _ = Illuminance::from_lux(1.0_f64).to_Glux();
	}
	#[test]
	fn radioactivity_units() {
		assert_approx_equal(
			Radioactivity::from_Bq(1.0_f64).to_Bq(),
			Radioactivity::from_mBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_mBq(1.0_f64).to_Bq(),
			Radioactivity::from_uBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_uBq(1.0_f64).to_Bq(),
			Radioactivity::from_nBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_kBq(1.0_f64).to_Bq(),
			Radioactivity::from_Bq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_MBq(1.0_f64).to_Bq(),
			Radioactivity::from_kBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_GBq(1.0_f64).to_Bq(),
			Radioactivity::from_MBq(1000.0_f64).to_Bq(), 9
		);
		assert_approx_equal(
			Radioactivity::from_Ci(1.0_f64).to_Bq(),
			Radioactivity::from_GBq(37.0_f64).to_Bq(), 2
		);
		assert_approx_equal(
			Radioactivity::from_Rd(1.0_f64).to_Bq(),
			Radioactivity::from_MBq(1.0_f64).to_Bq(), 9
		);
		let _ = Radioactivity::from_Bq(1.0_f64).to_Bq();
		let _ = Radioactivity::from_Bq(1.0_f64).to_mBq();
		let _ = Radioactivity::from_Bq(1.0_f64).to_uBq();
		let _ = Radioactivity::from_Bq(1.0_f64).to_nBq();
		let _ = Radioactivity::from_Bq(1.0_f64).to_kBq();
		let _ = Radioactivity::from_Bq(1.0_f64).to_MBq();
		let _ = Radioactivity::from_Bq(1.0_f64).to_GBq();
		let _ = Radioactivity::from_Bq(1.0_f64).to_Ci();
		let _ = Radioactivity::from_Bq(1.0_f64).to_Rd();
	}
	#[test]
	fn absorbed_dose_units() {
		assert_approx_equal(
			AbsorbedDose::from_Gy(1.0_f64).to_Gy(),
			AbsorbedDose::from_mGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_mGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_uGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_uGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_nGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_kGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_Gy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_MGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_kGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_GGy(1.0_f64).to_Gy(),
			AbsorbedDose::from_MGy(1000.0_f64).to_Gy(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(0.01_f64).to_Gy(),
			AbsorbedDose::from_rad(1.0_f64).to_Gy(), 3
		);
		assert_approx_equal(
			AbsorbedDose::from_Gy(1e-4_f64).to_Gy(),
			AbsorbedDose::from_erg(1.0_f64).to_Gy(), 3
		);
		assert_approx_equal(
			AbsorbedDose::from_rad(1.0_f64).to_rad(),
			AbsorbedDose::from_mrad(1000.0_f64).to_rad(), 9
		);
		assert_approx_equal(
			AbsorbedDose::from_krad(1.0_f64).to_rad(),
			AbsorbedDose::from_rad(1000.0_f64).to_rad(), 9
		);
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_Gy();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_mGy();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_uGy();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_nGy();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_kGy();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_MGy();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_GGy();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_rad();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_mrad();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_krad();
		let _ = AbsorbedDose::from_Gy(1.0_f64).to_erg();
	}
	#[test]
	fn dose_equivalent_units() {
		assert_approx_equal(
			DoseEquivalent::from_Sv(1.0_f64).to_Sv(),
			DoseEquivalent::from_mSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_mSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_uSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_uSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_nSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_kSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_Sv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_MSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_kSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_GSv(1.0_f64).to_Sv(),
			DoseEquivalent::from_MSv(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_rem(1.0_f64).to_Sv(),
			DoseEquivalent::from_Sv(0.01_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_rem(1.0_f64).to_Sv(),
			DoseEquivalent::from_mrem(1000.0_f64).to_Sv(), 9
		);
		assert_approx_equal(
			DoseEquivalent::from_krem(1.0_f64).to_Sv(),
			DoseEquivalent::from_rem(1000.0_f64).to_Sv(), 9
		);
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_Sv();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_mSv();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_uSv();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_nSv();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_kSv();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_MSv();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_GSv();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_rem();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_mrem();
		let _ = DoseEquivalent::from_Sv(1.0_f64).to_krem();
	}
	#[test]
	#[allow(dead_code)]
	#[allow(unused_variables)]
	fn unit_conversion_test(){
		let av: AngularVelocity<f64> = Angle::from_rad(1.0) / Time::from_s(1.0);
		let aa: AngularAcceleration<f64> = av / Time::from_s(1.0);
		let mi: MomentOfInertia<f64> = MomentOfInertia::from_gcm2(4.0);
		let am: AngularMomentum<f64> = mi * av;
		// let t:  Torque<f64> = Force::from_N(1.0) * Distance::from_m(1.0);
		let f:  Frequency<f64> = 1.0 / Time::from_s(1.0);
		let a:  Area<f64> = Distance::from_m(1.0) * Distance::from_m(1.0);
		let ad: AreaDensity<f64> = Mass::from_kg(1.0) / a;
		let v:  Volume<f64> = a * Distance::from_m(1.0);
		let v2: Velocity<f64> = Distance::from_m(1.0) / Time::from_s(1.0);
		let a2: Acceleration<f64> = v2 / Time::from_s(1.0);
		let m:  Momentum<f64> = Mass::from_kg(1.0) * v2;
		let f:  Force<f64> = Mass::from_kg(1.0) * a2;
		let p:  Pressure<f64> = f / a;
		let e:  Energy<f64> = f * Distance::from_m(1.0);
		let c:  Charge<f64> = Current::from_A(1.0) * Time::from_s(1.0);
		let p2: Power<f64> = e / Time::from_s(1.0);
		let v3: Voltage<f64> = p2 / Current::from_A(1.0);
		let r:  Resistance<f64> = v3 / Current::from_A(1.0);
		let c2: Conductance<f64> = 1.0 / r;
		let c3: Capacitance<f64> = c / v3;
		let mf: MagneticFlux<f64> = v3 * Time::from_s(1.0);
		let i:  Inductance<f64> = mf / Current::from_A(1.0);
		let md: MagneticFluxDensity<f64> = mf / a;
		let c4: Concentration<f64> = Concentration::from_M(1.0);
		let ca: Amount<f64> = c4 * v;
		let ca: CatalyticActivity<f64> = ca / Time::from_s(1.0);
		let lm: LuminousFlux<f64> = Luminosity::from_cd(1.0) * SolidAngle::from_sr(1.0);
		let lx: Illuminance<f64> = lm / a;
		// note: nuclear radiation units are not uniquely identifiable
		let tr: Time<f64> = 1.0 / Radioactivity::from_Bq(1.0); // equivalent to Hz
		let md: Energy<f64> = AbsorbedDose::from_Gy(1.0) * Mass::from_kg(1.0);
		let me: Energy<f64> = DoseEquivalent::from_Sv(1.0) * Mass::from_kg(1.0);
	}
	/// Unit test
	#[test]
	fn add_subtract_test() {
		// Note: math operators are implemented by the proc macro, so all units behave identically
		// therefore only need to test one of them to ensure all are compliant
		let d1 = Distance::from_m(2.5);
		let d2 = Distance::from_m(1.0);
		assert_approx_equal((d1+d2).to_m(), 3.5, 9);
		assert_approx_equal((d2+d1).to_m(), 3.5, 9);
		assert_approx_equal((d1-d2).to_m(), 1.5, 9);
		assert_approx_equal((d2-d1).to_m(), -1.5, 9);
		assert_approx_equal((d1-d1).to_m(), 0.0, 9);
	}
	/// Unit test
	#[test]
	fn mul_div_test() {
		let d1 = Distance::from_m(2.5);
		let d2 = Distance::from_m(2.0);
		assert_approx_equal(d1/d2, 1.25, 9);
		assert_approx_equal(d2/d1, 0.8, 9);
		assert_approx_equal((d1*d2).to_m2(), 5.0, 9);
		assert_approx_equal((d2*d1).to_m2(), 5.0, 9);
	}
	/// Unit test
	#[test]
	fn op_assign_test() {
		// +=, -=, *=, /=
		let mut d1 = Distance::from_m(2.5);
		let mut d2 = Distance::from_m(2.0);
		d1 += d2;
		assert_approx_equal(d1.to_m(), 4.5, 9);
		d2 -= d1;
		assert_approx_equal(d2.to_m(), -2.5, 9);
		d1 *= 2.0;
		assert_approx_equal(d1.to_m(), 9.0, 9);
		d2 /= -0.5;
		assert_approx_equal(d2.to_m(), 5.0, 9);
	}

	/// Unit test
	#[test]
	fn unit_names_and_symbols_test(){
		// test the .unit_name() and .unit_symbol() functions
		assert!(Amount::<f64>::unit_name().eq("moles"));
		assert!(Amount::<f64>::unit_symbol().eq("mol"));
		assert!(Current::<f64>::unit_name().eq("amperes"));
		assert!(Current::<f64>::unit_symbol().eq("A"));
		assert!(Distance::<f64>::unit_name().eq("meters"));
		assert!(Distance::<f64>::unit_symbol().eq("m"));
		assert!(Luminosity::<f64>::unit_name().eq("candela"));
		assert!(Luminosity::<f64>::unit_symbol().eq("cd"));
		assert!(Mass::<f64>::unit_name().eq("kilograms"));
		assert!(Mass::<f64>::unit_symbol().eq("kg"));
		assert!(Temperature::<f64>::unit_name().eq("degrees kelvin"));
		assert!(Temperature::<f64>::unit_symbol().eq("K"));
		assert!(Time::<f64>::unit_name().eq("seconds"));
		assert!(Time::<f64>::unit_symbol().eq("s"));
		assert!(CatalyticActivity::<f64>::unit_name().eq("moles per second"));
		assert!(CatalyticActivity::<f64>::unit_symbol().eq("mol/s"));
		assert!(Concentration::<f64>::unit_name().eq("moles per cubic meter"));
		assert!(Concentration::<f64>::unit_symbol().eq("mol/m³"));
		assert!(Capacitance::<f64>::unit_name().eq("farads"));
		assert!(Capacitance::<f64>::unit_symbol().eq("F"));
		assert!(Charge::<f64>::unit_name().eq("coulombs"));
		assert!(Charge::<f64>::unit_symbol().eq("C"));
		assert!(Conductance::<f64>::unit_name().eq("siemens"));
		assert!(Conductance::<f64>::unit_symbol().eq("S"));
		assert!(Illuminance::<f64>::unit_name().eq("lux"));
		assert!(Illuminance::<f64>::unit_symbol().eq("lux"));
		assert!(Inductance::<f64>::unit_name().eq("henries"));
		assert!(Inductance::<f64>::unit_symbol().eq("H"));
		assert!(LuminousFlux::<f64>::unit_name().eq("lumens"));
		assert!(LuminousFlux::<f64>::unit_symbol().eq("lm"));
		assert!(MagneticFlux::<f64>::unit_name().eq("webers"));
		assert!(MagneticFlux::<f64>::unit_symbol().eq("Wb"));
		assert!(MagneticFluxDensity::<f64>::unit_name().eq("teslas"));
		assert!(MagneticFluxDensity::<f64>::unit_symbol().eq("T"));
		assert!(Resistance::<f64>::unit_name().eq("ohms"));
		assert!(Resistance::<f64>::unit_symbol().eq("Ohm"));
		assert!(Voltage::<f64>::unit_name().eq("volts"));
		assert!(Voltage::<f64>::unit_symbol().eq("V"));
		assert!(Angle::<f64>::unit_name().eq("radians"));
		assert!(Angle::<f64>::unit_symbol().eq("rad"));
		assert!(Area::<f64>::unit_name().eq("square meters"));
		assert!(Area::<f64>::unit_symbol().eq("m²"));
		assert!(SolidAngle::<f64>::unit_name().eq("steradian"));
		assert!(SolidAngle::<f64>::unit_symbol().eq("sr"));
		assert!(Volume::<f64>::unit_name().eq("cubic meters"));
		assert!(Volume::<f64>::unit_symbol().eq("m³"));
		assert!(Acceleration::<f64>::unit_name().eq("meters per second squared"));
		assert!(Acceleration::<f64>::unit_symbol().eq("m/s²"));
		assert!(AngularAcceleration::<f64>::unit_name().eq("radians per second squared"));
		assert!(AngularAcceleration::<f64>::unit_symbol().eq("rad/s²"));
		assert!(AngularMomentum::<f64>::unit_name().eq("kilogram meters squared radians per second"));
		assert!(AngularMomentum::<f64>::unit_symbol().eq("kg·m²·rad/s"));
		assert!(AngularVelocity::<f64>::unit_name().eq("radians per second"));
		assert!(AngularVelocity::<f64>::unit_symbol().eq("rad/s"));
		assert!(AreaDensity::<f64>::unit_name().eq("kilograms per square meter"));
		assert!(AreaDensity::<f64>::unit_symbol().eq("kg/m²"));
		assert!(Density::<f64>::unit_name().eq("kilograms per cubic meter"));
		assert!(Density::<f64>::unit_symbol().eq("kg/m³"));
		assert!(Energy::<f64>::unit_name().eq("joules"));
		assert!(Energy::<f64>::unit_symbol().eq("J"));
		assert!(Force::<f64>::unit_name().eq("newtons"));
		assert!(Force::<f64>::unit_symbol().eq("N"));
		assert!(Frequency::<f64>::unit_name().eq("hertz"));
		assert!(Frequency::<f64>::unit_symbol().eq("Hz"));
		assert!(MomentOfInertia::<f64>::unit_name().eq("kilogram meters squared"));
		assert!(MomentOfInertia::<f64>::unit_symbol().eq("kg·m²"));
		assert!(Momentum::<f64>::unit_name().eq("kilogram meters per second"));
		assert!(Momentum::<f64>::unit_symbol().eq("kg·m/s"));
		assert!(Power::<f64>::unit_name().eq("watts"));
		assert!(Power::<f64>::unit_symbol().eq("W"));
		assert!(Pressure::<f64>::unit_name().eq("pascals"));
		assert!(Pressure::<f64>::unit_symbol().eq("Pa"));
		assert!(Torque::<f64>::unit_name().eq("newton meters"));
		assert!(Torque::<f64>::unit_symbol().eq("Nm"));
		assert!(Velocity::<f64>::unit_name().eq("meters per second"));
		assert!(Velocity::<f64>::unit_symbol().eq("m/s"));
		assert!(AbsorbedDose::<f64>::unit_name().eq("grays"));
		assert!(AbsorbedDose::<f64>::unit_symbol().eq("Gy"));
		assert!(DoseEquivalent::<f64>::unit_name().eq("sieverts"));
		assert!(DoseEquivalent::<f64>::unit_symbol().eq("Sv"));
		assert!(Radioactivity::<f64>::unit_name().eq("becquerels"));
		assert!(Radioactivity::<f64>::unit_symbol().eq("Bq"));
	}

	#[test]
	fn unit_print_display_test() {
		println!("Printing 1 of every unit to terminal");
		println!("{}", Amount{mol: 1});
		println!("{}", Current{A: 1});
		println!("{}", Distance{m: 1});
		println!("{}", Luminosity{cd: 1});
		println!("{}", Mass{kg: 1});
		println!("{}", Temperature{K: 1});
		println!("{}", Time{s: 1});
		println!("{}", CatalyticActivity{molps: 1});
		println!("{}", Concentration{molpm3: 1});
		println!("{}", Capacitance{F: 1});
		println!("{}", Charge{C: 1});
		println!("{}", Conductance{S: 1});
		println!("{}", Illuminance{lux: 1});
		println!("{}", Inductance{H: 1});
		println!("{}", LuminousFlux{lm: 1});
		println!("{}", MagneticFlux{Wb: 1});
		println!("{}", MagneticFluxDensity{T: 1});
		println!("{}", Resistance{Ohm: 1});
		println!("{}", Voltage{V: 1});
		println!("{}", Angle{rad: 1});
		println!("{}", Area{m2: 1});
		println!("{}", SolidAngle{sr: 1});
		println!("{}", Volume{m3: 1});
		println!("{}", Acceleration{mps2: 1});
		println!("{}", AngularAcceleration{radps2: 1});
		println!("{}", AngularMomentum{kgm2radps: 1});
		println!("{}", AngularVelocity{radps: 1});
		println!("{}", AreaDensity{kgpm2: 1});
		println!("{}", Density{kgpm3: 1});
		println!("{}", Energy{J: 1});
		println!("{}", Force{N: 1});
		println!("{}", Frequency{Hz: 1});
		println!("{}", MomentOfInertia{kgm2: 1});
		println!("{}", Momentum{kgmps: 1});
		println!("{}", Power{W: 1});
		println!("{}", Pressure{Pa: 1});
		println!("{}", Torque{Nm: 1});
		println!("{}", Velocity{mps: 1});
		println!("{}", AbsorbedDose{Gy: 1});
		println!("{}", DoseEquivalent{Sv: 1});
		println!("{}", Radioactivity{Bq: 1});
	}

	fn mul_check<
		'y,
		A: std::ops::Mul<B, Output = X>+Clone+'y,
		B: std::ops::Mul<A, Output = X>+Clone+'y,
		X: std::cmp::PartialEq+Clone
	>(a: &'y A, b: &'y B) -> X where &'y B: std::ops::Mul<&'y A, Output = X>, &'y A: std::ops::Mul<&'y B, Output = X>
	{
		let x1: X = a * b;
		let x2: X = b * a;
		let x3: X = a.clone() * b.clone();
		let x4: X = b.clone() * a.clone();
		assert!((x1.eq(&x2)));
		assert!((x1.eq(&x3)));
		assert!((x1.eq(&x4)));
		return x1;
	}

	fn div_check<
		'y,
		A: std::ops::Div<B, Output = X>+Clone+'y,
		B: Clone+'y,
		X: std::cmp::PartialEq+Clone
	>(a: &'y A, b: &'y B) -> X where &'y A: std::ops::Div<&'y B, Output = X>
	{
		let x1: X = a / b;
		let x2: X = a.clone() / b.clone();
		assert!((x1.eq(&x2)));
		return x1;
	}

	#[test]
	fn test_unit_converions(){
		assert_eq!(div_check(&Amount{mol:2.5}, &Time{s:0.5}), CatalyticActivity{molps: 2.5/0.5});
		assert_eq!(div_check(&Amount{mol:2.5}, &Concentration{molpm3:0.5}), Volume{m3: 2.5/0.5});
		todo!();
	}
	// TODO: templated function to test all unit struct operators and then use it on all structs
}
