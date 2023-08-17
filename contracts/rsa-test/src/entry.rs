#![allow(dead_code)]
#![allow(unused_imports)]

// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{format, vec};

use ckb_std::syscalls::debug;
use num_traits::{FromPrimitive, Num};
use rsa::pkcs1v15::VerifyingKey;
use rsa::sha2::Sha256;
use rsa::RsaPublicKey;
use rsa::{
    pkcs1v15,
    signature::{Keypair, Verifier},
    BigUint, RsaPrivateKey,
};

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use crate::error::Error;
use ckb_std::syscalls::current_cycles;

fn generate_msg() -> [u8; 32] {
    let hello = b"hello, world";
    let mut msg = [0u8; 32];
    msg[0..hello.len()].copy_from_slice(hello);
    msg
}

fn get_private_key() -> RsaPrivateKey {
    RsaPrivateKey::from_components(
        BigUint::from_str_radix("14314132931241006650998084889274020608918049032671858325988396851334124245188214251956198731333464217832226406088020736932173064754214329009979944037640912127943488972644697423190955557435910767690712778463524983667852819010259499695177313115447116110358524558307947613422897787329221478860907963827160223559690523660574329011927531289655711860504630573766609239332569210831325633840174683944553667352219670930408593321661375473885147973879086994006440025257225431977751512374815915392249179976902953721486040787792801849818254465486633791826766873076617116727073077821584676715609985777563958286637185868165868520557", 10).unwrap(),
        BigUint::from_u32(3).unwrap(),
        BigUint::from_str_radix("9542755287494004433998723259516013739278699355114572217325597900889416163458809501304132487555642811888150937392013824621448709836142886006653296025093941418628992648429798282127303704957273845127141852309016655778568546006839666463451542076964744073572349705538631742281931858219480985907271975884773482372966847639853897890615456605598071088189838676728836833012254065983259638538107719766738032720239892094196108713378822882383694456030043492571063441943847195939549773271694647657549658603365629458610273821292232646334717612674519997533901052790334279661754176490593041941863932308687197618671528035670452762731", 10).unwrap(),
        vec![
            BigUint::from_str_radix("130903255182996722426771613606077755295583329135067340152947172868415809027537376306193179624298874215608270802054347609836776473930072411958753044562214537013874103802006369634761074377213995983876788718033850153719421695468704276694983032644416930879093914927146648402139231293035971427838068945045019075433",10).unwrap(),
            BigUint::from_str_radix("109348945610485453577574767652527472924289229538286649661240938988020367005475727988253438647560958573506159449538793540472829815903949343191091817779240101054552748665267574271163617694640513549693841337820602726596756351006149518830932261246698766355347898158548465400674856021497190430791824869615170301029", 10).unwrap()
        ],
    ).unwrap()
}

fn get_public_key() -> RsaPublicKey {
    RsaPublicKey::new(
        BigUint::from_str_radix("14314132931241006650998084889274020608918049032671858325988396851334124245188214251956198731333464217832226406088020736932173064754214329009979944037640912127943488972644697423190955557435910767690712778463524983667852819010259499695177313115447116110358524558307947613422897787329221478860907963827160223559690523660574329011927531289655711860504630573766609239332569210831325633840174683944553667352219670930408593321661375473885147973879086994006440025257225431977751512374815915392249179976902953721486040787792801849818254465486633791826766873076617116727073077821584676715609985777563958286637185868165868520557", 10).unwrap(),
        BigUint::from_u32(3).unwrap()
    ).unwrap()
}

pub fn test_rsa_2048(msg: &[u8]) {
    let public_key = get_public_key();

    // skip signing step to reduce binary size
    // let signing_key = pkcs1v15::SigningKey::<Sha256>::new(private_key);
    // let signature_bytes = signing_key.sign(msg).to_bytes();
    // debug(format!("signature_bytes = {:?}", signature_bytes));
    let signature_bytes: [u8; 256] = [
        35, 155, 174, 221, 58, 78, 62, 182, 178, 141, 112, 223, 215, 101, 242, 209, 35, 71, 146,
        245, 250, 24, 56, 146, 225, 188, 79, 215, 160, 10, 150, 57, 41, 176, 62, 27, 172, 40, 233,
        33, 190, 68, 66, 12, 83, 19, 30, 215, 99, 50, 118, 32, 246, 250, 191, 150, 42, 190, 219,
        57, 182, 157, 41, 22, 48, 18, 105, 26, 112, 157, 35, 181, 42, 227, 251, 228, 81, 162, 94,
        248, 172, 243, 31, 70, 91, 253, 197, 111, 179, 53, 53, 211, 45, 246, 36, 125, 85, 102, 1,
        47, 89, 172, 78, 167, 212, 202, 189, 18, 14, 150, 189, 103, 93, 65, 14, 62, 228, 114, 86,
        50, 151, 80, 21, 98, 183, 193, 35, 186, 170, 71, 109, 13, 26, 122, 86, 45, 202, 69, 195, 5,
        122, 182, 90, 30, 227, 167, 186, 82, 126, 62, 110, 187, 62, 175, 108, 153, 149, 161, 252,
        113, 111, 97, 40, 151, 41, 9, 162, 14, 170, 234, 210, 44, 3, 2, 164, 85, 248, 126, 107, 8,
        229, 116, 39, 238, 115, 193, 83, 66, 146, 5, 158, 116, 102, 246, 188, 23, 137, 12, 9, 130,
        9, 146, 92, 221, 83, 38, 97, 181, 7, 243, 181, 93, 176, 185, 54, 47, 54, 198, 127, 234,
        217, 25, 199, 32, 49, 67, 76, 217, 200, 35, 158, 173, 114, 17, 208, 49, 24, 131, 130, 11,
        96, 80, 178, 8, 47, 3, 73, 7, 204, 205, 38, 54, 9, 72, 23, 232,
    ];
    let verifying_key: VerifyingKey<Sha256> = VerifyingKey::new(public_key);
    let signature = pkcs1v15::Signature::try_from(&signature_bytes[..]).unwrap();

    let last = current_cycles();
    verifying_key.verify(&msg, &signature).unwrap();
    let cycles = current_cycles() - last;
    debug(format!(
        "cost of rsa-2048 verifying cycles: {} K",
        cycles / 1024
    ));
}

pub fn main() -> Result<(), Error> {
    let msg = generate_msg();
    test_rsa_2048(&msg);

    Ok(())
}
