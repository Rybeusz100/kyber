mod load;

use load::*;
use pqc_kyber::*;

// Generate KAT keypairs from seeds.
#[test]
#[cfg(feature="KATs")]
fn keypairs() {
  let kats = build_kats();
  let mut _rng = rand::thread_rng(); // no-op for KATs
  for kat in kats {
    let known_pk = decode_hex(&kat.pk);
    let known_sk = decode_hex(&kat.sk);
    let buf1 = decode_hex(&kat.keygen_buffer1);
    let buf2 = decode_hex(&kat.keygen_buffer2);
    let bufs = Some((&buf1[..], &buf2[..]));
    let mut pk = [0u8; KYBER_PUBLICKEYBYTES];
    let mut sk = [0u8; KYBER_SECRETKEYBYTES];
    let res = crypto_kem_keypair(&mut pk, &mut sk, &mut _rng, bufs);
    assert!(res.is_ok());
    assert_eq!(&pk[..], &known_pk[..], "Public key generation failure");
    assert_eq!(&sk[..], &known_sk[..], "Secret key generation failure");
  }
}

// Encapsulating KAT's using deterministic rand buffers
#[test]
#[cfg(all(feature="KATs"))]
fn encaps() {
  let kats = build_kats();
  let mut _rng = rand::thread_rng(); // no-op for KATs
  for kat in kats {
    let known_ct = decode_hex(&kat.ct);
    let known_ss = decode_hex(&kat.ss);
    let pk = decode_hex(&kat.pk);
    let buf1 = decode_hex(&kat.encap_buffer);
    let encap_buf = Some(&buf1[..]);
    let mut ct = [0u8; KYBER_CIPHERTEXTBYTES];
    let mut ss = [0u8; KYBER_SSBYTES];
    let res = crypto_kem_enc(&mut ct, &mut ss, &pk, &mut _rng, encap_buf);
    assert!(res.is_ok());
    assert_eq!(&ct[..], &known_ct[..], "Ciphertext creation failure");
    assert_eq!(&ss[..], &known_ss[..], "Shared secret creation failure");
  }
}

// Decapsulating KAT's
#[test]
#[cfg(all(feature="KATs"))]
fn decaps() {
  let kats = build_kats();
  for kat in kats {
    let sk = decode_hex(&kat.sk);
    let ct = decode_hex(&kat.ct);
    let decap_result = decapsulate(&ct, &sk);
    assert!(decap_result.is_ok(), "KEM decasulation failure");
  }
}

// Encodes a byte slice into a hex string
pub fn encode_hex(bytes: &[u8]) -> String {
  let mut output = String::new();
  for b in bytes {
        output.push_str(&format!("{:02X}", b));
    }
  output
}

// Decodes a hex string into a vector of bytes
pub fn decode_hex(s: &str) -> Vec<u8> {
  (0..s.len())
      .step_by(2)
      .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("Hex string decoding"))
      .collect::<Vec<u8>>()
}