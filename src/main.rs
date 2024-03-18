use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::{short_weierstrass::curves::bls12_381::{curve::BLS12381Curve, twist::BLS12381TwistCurve}, traits::IsEllipticCurve}, field::fields::u64_prime_field::U64FieldElement};

fn main() {
    let secret_key: u64 = 0x6C616D6264617370;
    let g = BLS12381Curve::generator();
    let public_key = g.operate_with_self(secret_key);

    // Secret and Public Key
    println!("Secret Key --> {:?}", secret_key);
    println!("Public Key --> {:?}", public_key);

    let key_affine = public_key.to_affine();
    let x = key_affine.x();
    let y = key_affine.y();

    // Coordinates
    println!("X --> {:?}", x);
    println!("Y --> {:?}", y)
}