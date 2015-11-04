use super::types::*;
use super::ifdformat::*;

static INV: &'static str = "Invalid data for this tag";

/// No-op for readable value tag function. Should not be used by any EXIF tag descriptor,
/// except for the catch-all match that handles unknown tags
pub fn nop(_: &TagValue, s: &String) -> String
{
	return s.clone();
}

/// No-op for readable value tag function. Used for ASCII string tags, or when the
/// default readable representation of value is pretty enough.
pub fn strpass(_: &TagValue, s: &String) -> String
{
	return s.clone();
}

pub fn orientation(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Straight",
				3 => "Upside down",
				6 => "Rotated to left",
				8 => "Rotated to right",
				9 => "Undefined",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn rational_value(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{}", v[0].value())
		},
		&TagValue::IRational(ref v) => {
			format!("{}", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn rational_values(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			let ve: Vec<f64> = v.iter().map(|&x| x.value()).collect();
			numarray_to_string(&ve)
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn resolution_unit(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Unitless",
				2 => "in",
				3 => "cm",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn exposure_time(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{} s", v[0])
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn f_number(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("f/{:.1}", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn exposure_program(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			let n = v[0];
			match n {
				1 => "Manual control",
				2 => "Program control",
				3 => "Aperture priority",
				4 => "Shutter priority",
				5 => "Program creative (slow program)",
				6 => "Program creative (high-speed program)",
				7 => "Portrait mode",
				8 => "Landscape mode",
				_ => return format!("Unknown ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn focal_length(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{} mm", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn focal_length_35(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::U16(ref v) => {
			format!("{} mm", v[0])
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn meters(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::URational(ref v) => {
			format!("{:.1} m", v[0].value())
		},
		_ => panic!(INV),
	};

	return s.to_string();
}

pub fn iso_speeds(e: &TagValue, _: &String) -> String
{
	let s = match e {
	&TagValue::U16(ref v) => {
		if v.len() == 1 {
			format!("ISO {}", v[0])
		} else if v.len() == 2 {
			format!("ISO {} latitude {}", v[0], v[1])
		} else {
			format!("Unknown ({})", numarray_to_string(&v))
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn dms(e: &TagValue, _: &String) -> String
{
	let s = match e {
	&TagValue::URational(ref v) => {
		let deg = v[0];
		let min = v[1];
		let sec = v[2];
		if deg.denominator == 1 && min.denominator == 1 {
			format!("{}º{}'{:.2}\"", deg.value(), min.value(), sec.value())
		} else if deg.denominator == 1 {
			format!("{}º{:.4}'", deg.value(), min.value() + sec.value() / 60.0)
		} else {
			// untypical format
			format!("{:.7}º", deg.value() + min.value() / 60.0 + sec.value() / 3600.0)
		}
	},
	_ => panic!(INV),
	};

	return s.to_string();
}

pub fn gps_alt_ref(e: &TagValue, _: &String) -> String
{
	let s = match e {
		&TagValue::U8(ref v) => {
			let n = v[0];
			match n {
				0 => "Above sea level",
				1 => "Below sea level",
				_ => return format!("Unknown, assumed below sea level ({})", n),
			}
		},
		_ => panic!(INV),
	};

	return s.to_string();
}
