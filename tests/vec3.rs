use glam::*;
#[cfg(feature = "rand")]
use rand::{Rng, SeedableRng};
#[cfg(feature = "rand")]
use rand_xoshiro::Xoshiro256Plus;
use std::mem;

#[test]
fn test_vec3_new() {
    let v = vec3(1.0, 2.0, 3.0);

    assert_eq!(mem::size_of_val(&v), 16);
    assert_eq!(mem::align_of_val(&v), 16);

    assert_eq!(v.get_x(), 1.0);
    assert_eq!(v.get_y(), 2.0);
    assert_eq!(v.get_z(), 3.0);

    let t = (1.0, 2.0, 3.0);
    let v = Vec3::from(t);
    assert_eq!(t, v.into());

    let a = [1.0, 2.0, 3.0];
    let v = Vec3::from(a);
    let a1: [f32; 3] = v.into();
    assert_eq!(a, a1);

    let v = Vec3::new(t.0, t.1, t.2);
    assert_eq!(t, v.into());
}

#[test]
fn test_vec3_zero() {
    let v = Vec3::zero();
    assert_eq!((0.0, 0.0, 0.0), v.into());
}

#[test]
fn test_vec3_splat() {
    let v = Vec3::splat(1.0);
    assert_eq!((1.0, 1.0, 1.0), v.into());
}

#[test]
fn test_vec3_accessors() {
    let mut a = vec3(0.0, 0.0, 0.0);
    a.set_x(1.0);
    a.set_y(2.0);
    a.set_z(3.0);
    assert_eq!(1.0, a.get_x());
    assert_eq!(2.0, a.get_y());
    assert_eq!(3.0, a.get_z());
}

#[test]
fn test_vec3_funcs() {
    let x = vec3(1.0, 0.0, 0.0);
    let y = vec3(0.0, 1.0, 0.0);
    let z = vec3(0.0, 0.0, 1.0);
    assert_eq!(1.0, x.dot(x));
    assert_eq!(0.0, x.dot(y));
    assert_eq!(-1.0, z.dot(-z));
    assert_eq!(y, z.cross(x));
    assert_eq!(z, x.cross(y));
    assert_eq!(4.0, (2.0 * x).length_squared());
    assert_eq!(9.0, (-3.0 * y).length_squared());
    assert_eq!(16.0, (4.0 * z).length_squared());
    assert_eq!(2.0, (-2.0 * x).length());
    assert_eq!(3.0, (3.0 * y).length());
    assert_eq!(4.0, (-4.0 * z).length());
    assert_eq!(x, (2.0 * x).normalize());
}

#[test]
fn test_vec3_ops() {
    let a = vec3(1.0, 2.0, 3.0);
    assert_eq!((2.0, 4.0, 6.0), (a + a).into());
    assert_eq!((0.0, 0.0, 0.0), (a - a).into());
    assert_eq!((1.0, 4.0, 9.0), (a * a).into());
    assert_eq!((2.0, 4.0, 6.0), (a * 2.0).into());
    assert_eq!((1.0, 1.0, 1.0), (a / a).into());
    assert_eq!((0.5, 1.0, 1.5), (a / 2.0).into());
    assert_eq!((-1.0, -2.0, -3.0), (-a).into());
}

#[test]
fn test_vec3_assign_ops() {
    let a = vec3(1.0, 2.0, 3.0);
    let mut b = a;
    b += a;
    assert_eq!((2.0, 4.0, 6.0), b.into());
    b -= a;
    assert_eq!((1.0, 2.0, 3.0), b.into());
    b *= a;
    assert_eq!((1.0, 4.0, 9.0), b.into());
    b /= a;
    assert_eq!((1.0, 2.0, 3.0), b.into());
    b *= 2.0;
    assert_eq!((2.0, 4.0, 6.0), b.into());
    b /= 2.0;
    assert_eq!((1.0, 2.0, 3.0), b.into());
}

#[test]
fn test_vec3_min_max() {
    let a = vec3(-1.0, 2.0, -3.0);
    let b = vec3(1.0, -2.0, 3.0);
    assert_eq!((-1.0, -2.0, -3.0), a.min(b).into());
    assert_eq!((-1.0, -2.0, -3.0), b.min(a).into());
    assert_eq!((1.0, 2.0, 3.0), a.max(b).into());
    assert_eq!((1.0, 2.0, 3.0), b.max(a).into());
}

#[test]
fn test_vec3_hmin_hmax() {
    let a = vec3(-1.0, 2.0, -3.0);
    assert_eq!(-3.0, a.hmin());
    assert_eq!(2.0, a.hmax());
}

#[test]
fn test_vec3_eq() {
    let a = vec3(1.0, 1.0, 1.0);
    let b = vec3(1.0, 2.0, 3.0);
    assert!(a.cmpeq(a).all());
    assert!(b.cmpeq(b).all());
    assert!(a.cmpne(b).any());
    assert!(b.cmpne(a).any());
    assert!(b.cmpeq(a).any());
}

#[test]
fn test_vec3_cmp() {
    let a = vec3(-1.0, -1.0, -1.0);
    let b = vec3(1.0, 1.0, 1.0);
    let c = vec3(-1.0, -1.0, 1.0);
    let d = vec3(1.0, -1.0, -1.0);
    assert_eq!(a.cmplt(a).mask(), 0x0);
    assert_eq!(a.cmplt(b).mask(), 0x7);
    assert_eq!(a.cmplt(c).mask(), 0x4);
    assert_eq!(c.cmple(a).mask(), 0x3);
    assert_eq!(a.cmplt(d).mask(), 0x1);
    assert!(a.cmplt(b).all());
    assert!(a.cmplt(c).any());
    assert!(a.cmple(b).all());
    assert!(a.cmple(a).all());
    assert!(b.cmpgt(a).all());
    assert!(b.cmpge(a).all());
    assert!(b.cmpge(b).all());
    assert!(!(a.cmpge(c).all()));
    assert!(c.cmple(c).all());
    assert!(c.cmpge(c).all());
}

#[test]
fn test_extend_truncate() {
    let a = vec3(1.0, 2.0, 3.0);
    let b = a.extend(4.0);
    assert_eq!((1.0, 2.0, 3.0, 4.0), b.into());
    let c = b.truncate();
    assert_eq!(a, c);
}

#[test]
fn test_vec3b() {
    // make sure the unused 'w' value doesn't break Vec3b behaviour
    let a = Vec4::zero();
    let mut b = a.truncate();
    b.set_x(1.0);
    b.set_y(1.0);
    b.set_z(1.0);
    assert!(!b.cmpeq(Vec3::zero()).any());
    assert!(b.cmpeq(Vec3::splat(1.0)).all());
}

#[cfg(feature = "rand")]
#[test]
fn test_vec3_rand() {
    let mut rng1 = Xoshiro256Plus::seed_from_u64(0);
    let a: (f32, f32, f32) = rng1.gen();
    let mut rng2 = Xoshiro256Plus::seed_from_u64(0);
    let b: Vec3 = rng2.gen();
    assert_eq!(a, b.into());
}