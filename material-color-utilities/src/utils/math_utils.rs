// Utility methods for mathematical operations.

/// The signum function. This differs from the rust version as zero is returned as 0.0
///
/// Returns 1 if num > 0, -1 if num < 0, and 0 if num = 0.
pub fn signum(num: f64) -> f64 {
    if num < 0.0 {
        return -1.0;
    } else if num == 0.0 {
        return 0.0;
    } else {
        return 1.0;
    }
}

/// The linear interpolation function.
///
/// Returns start if amount = 0 and stop if amount = 1.
pub fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    return (1.0 - amount) * start + amount * stop;
}

/// Clamps an integer between two integers.
///
/// Returns input when min <= input <= max, and either min or max otherwise.
pub fn clamp_int(min: i32, max: i32, input: i32) -> i32 {
    if input < min {
        return min;
    } else if input > max {
        return max;
    }

    return input;
}

/// Clamps an integer between two floating-point numbers.
///
/// Returns input when min <= input <= max, and either min or max otherwise.
pub fn clamp_double(min: f64, max: f64, input: f64) -> f64 {
    if input < min {
        return min;
    } else if input > max {
        return max;
    }

    return input;
}

/// Sanitizes a degree measure as an integer.
///
/// Returns a degree measure between 0 (inclusive) and 360 (exclusive).
pub fn sanitize_degrees_int(degrees: i32) -> i32 {
    let mut degrees = degrees % 360;
    if degrees < 0 {
        degrees = degrees + 360;
    }
    return degrees;
}

/// Sanitizes a degree measure as a floating-point number.
///
/// Returns a degree measure between 0.0 (inclusive) and 360.0 (exclusive).
pub fn sanitize_degrees_double(degrees: f64) -> f64 {
    let mut degrees = degrees % 360.0;
    if degrees < 0.0 {
        degrees = degrees + 360.0;
    }
    return degrees;
}

/// Sign of direction change needed to travel from one angle to another.
///
/// For angles that are 180 degrees apart from each other, both directions have the same travel distance, so either direction is shortest. The value 1.0 is returned in this case.
///
/// * `from` - The angle travel starts from, in degrees.
/// * `to` - The angle travel ends at, in degrees.
///
/// Returns -1 if decreasing from leads to the shortest travel distance, 1 if increasing from leads to the shortest travel distance.
pub fn rotation_direction(from: f64, to: f64) -> f64 {
    let increasing_difference = sanitize_degrees_double(to - from);
    return if increasing_difference <= 180.0 {
        1.0
    } else {
        -1.0
    };
}

/// Distance of two points on a circle, represented using degrees.
pub fn difference_degrees(a: f64, b: f64) -> f64 {
    return 180.0 - ((a - b).abs() - 180.0).abs();
}

/// Multiplies a 1x3 row vector with a 3x3 matrix.
pub fn matrix_multiply(row: &[f64], matrix: &[[f64; 3]; 3]) -> [f64; 3] {
    let a = row[0] * matrix[0][0] + row[1] * matrix[0][1] + row[2] * matrix[0][2];
    let b = row[0] * matrix[1][0] + row[1] * matrix[1][1] + row[2] * matrix[1][2];
    let c = row[0] * matrix[2][0] + row[1] * matrix[2][1] + row[2] * matrix[2][2];
    return [a, b, c];
}
