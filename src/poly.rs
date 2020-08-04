use crate::{
  params::*,
  ntt::*,
  reduce::*,
  cbd::*,
  symmetric::*
};

#[derive(Clone)]
pub struct Poly {
  pub coeffs: [i16; KYBER_N]  
}

impl Copy for Poly {}

impl Default for Poly {
  fn default() -> Self {
    Poly {
      coeffs: [0i16; KYBER_N]
    }
  }
}

impl Poly {
  pub fn new() -> Self {
    Self::default()
  }
}
/*************************************************
* Name:        poly_compress
*
* Description: Compression and subsequent serialization of a polynomial
*
* Arguments:   - unsigned char *r: pointer to output byte array (needs space for KYBER_POLYCOMPRESSEDBYTES bytes)
*              - const poly *a:    pointer to input polynomial
**************************************************/
pub fn poly_compress(r: &mut[u8], a: &mut Poly)
{
  let mut t = [0u8; 8];
  let mut k = 0usize;

  match KYBER_POLYCOMPRESSEDBYTES {
    96 => {
      for i in (0..KYBER_N).step_by(8) {
        for j in 0..8 {
          // TODO: Check conversion wrapping
          t[j] = (((a.coeffs[i+j] << 3) + (KYBER_Q/2) as i16 / KYBER_Q as i16) & 7) as u8;
        }
        r[k]   =  t[0]       | (t[1] << 3) | (t[2] << 6);
        r[k+1] = (t[2] >> 2) | (t[3] << 1) | (t[4] << 4) | (t[5] << 7);
        r[k+2] = (t[5] >> 1) | (t[6] << 2) | (t[7] << 5);
        k += 3;
      }
    },
    128 => {
      for i in (0..KYBER_N).step_by(8) {
        for j in 0..8 {
          t[j] = (((a.coeffs[i+j] << 4) + (KYBER_Q/2) as i16 / KYBER_Q as i16) & 15) as u8;
        }
        r[k]   = t[0] | (t[1] << 4);
        r[k+1] = t[2] | (t[3] << 4);
        r[k+2] = t[4] | (t[5] << 4);
        r[k+3] = t[6] | (t[7] << 4);
        k += 4;
      }
    },
    160 => {
      for i in (0..KYBER_N).step_by(8) {
        for j in 0..8 {
          t[j] = (((a.coeffs[i+j] << 5) + (KYBER_Q/2) as i16 / KYBER_Q as i16) & 31) as u8;
        }
        r[k]   =  t[0]       | (t[1] << 5);
        r[k+1] = (t[1] >> 3) | (t[2] << 2) | (t[3] << 7);
        r[k+2] = (t[3] >> 1) | (t[4] << 4);
        r[k+3] = (t[4] >> 4) | (t[5] << 1) | (t[6] << 6);
        r[k+4] = (t[6] >> 2) | (t[7] << 3);
        k += 5;
      }
    },
    _ => panic!("KYBER_POLYCOMPRESSEDBYTES needs to be in {96, 128, 160}")
  }
}

/*************************************************
* Name:        poly_decompress
*
* Description: De-serialization and subsequent decompression of a polynomial;
*              approximate inverse of poly_compress
*
* Arguments:   - poly *r:                pointer to output polynomial
*              - const unsigned char *a: pointer to input byte array (of length KYBER_POLYCOMPRESSEDBYTES bytes)
**************************************************/

pub fn poly_decompress(r: &mut Poly, a: &[u8])
{
  match KYBER_POLYCOMPRESSEDBYTES {
    96 => {
      let mut idx = 0usize;
      for i in (0..KYBER_N).step_by(8) {
        r.coeffs[i+0] =  ((((a[idx+0] & 7) as usize * KYBER_Q) + 4) >> 3) as i16;
        r.coeffs[i+1] = (((((a[idx+0] >> 3) & 7) as usize * KYBER_Q) + 4) >> 3) as i16;
        r.coeffs[i+2] = (((((a[idx+0] >> 6) | ((a[idx+1] << 2) & 4)) as usize * KYBER_Q) + 4) >> 3) as i16;
        r.coeffs[i+3] = (((((a[idx+1] >> 1) & 7) as usize * KYBER_Q) + 4) >> 3) as i16;
        r.coeffs[i+4] = (((((a[idx+1] >> 4) & 7) as usize * KYBER_Q) + 4) >> 3) as i16;
        r.coeffs[i+5] = (((((a[idx+1] >> 7) | ((a[idx+2] << 1) & 6)) as usize * KYBER_Q) + 4) >> 3) as i16;
        r.coeffs[i+6] = (((((a[idx+2] >> 2) & 7) as usize * KYBER_Q) + 4) >> 3) as i16;
        r.coeffs[i+7] = (((((a[idx+2] >> 5)) as usize * KYBER_Q) + 4) >> 3) as i16;
        idx += 3;
      }
    },
    128 => {
      let mut idx = 0usize;
      for i in (0..KYBER_N).step_by(8) {
        r.coeffs[i+0] = ((((a[idx+0] & 15) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[i+1] = ((((a[idx+0] >> 4) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[i+2] = ((((a[idx+1] & 15) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[i+3] = ((((a[idx+1] >> 4) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[i+4] = ((((a[idx+2] & 15) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[i+5] = ((((a[idx+2] >> 4) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[i+6] = ((((a[idx+3] & 15) as usize * KYBER_Q) + 8) >> 4) as i16;
        r.coeffs[i+7] = ((((a[idx+3] >> 4) as usize * KYBER_Q) + 8) >> 4) as i16;
        idx += 4;
      }
    },
    160 => {
      let mut idx = 0usize;
      for i in (0..KYBER_N).step_by(8) {
        r.coeffs[i+0] =  ((((a[idx+0] & 31) as usize * KYBER_Q) + 16) >> 5) as i16;
        r.coeffs[i+1] = (((((a[idx+0] >> 5) | ((a[idx+1] & 3) << 3)) as usize * KYBER_Q) + 16) >> 5) as i16;
        r.coeffs[i+2] = (((((a[idx+1] >> 2) & 31) as usize * KYBER_Q) + 16) >> 5) as i16;
        r.coeffs[i+3] = (((((a[idx+1] >> 7) | ((a[idx+2] & 15) << 1)) as usize * KYBER_Q) + 16) >> 5) as i16;
        r.coeffs[i+4] = (((((a[idx+2] >> 4) | ((a[idx+3] &  1) << 4)) as usize * KYBER_Q) + 16) >> 5) as i16;
        r.coeffs[i+5] = (((((a[idx+3] >> 1) & 31) as usize * KYBER_Q) + 16) >> 5) as i16;
        r.coeffs[i+6] = (((((a[idx+3] >> 6) | ((a[idx+4] &  7) << 2)) as usize * KYBER_Q) + 16) >> 5) as i16;
        r.coeffs[i+7] =  ((((a[idx+4] >> 3) as usize * KYBER_Q) + 16) >> 5) as i16;
        idx += 5;
      }
    },
    _ => panic!("KYBER_POLYCOMPRESSEDBYTES needs to be in {96, 128, 160}")
  }
}

/*************************************************
* Name:        poly_tobytes
*
* Description: Serialization of a polynomial
*
* Arguments:   - unsigned char *r: pointer to output byte array (needs space for KYBER_POLYBYTES bytes)
*              - const poly *a:    pointer to input polynomial
**************************************************/

pub fn poly_tobytes(r: &mut[u8], a: &mut Poly)
{
  poly_csubq(a);
  let (mut t0, mut t1) = (0i16, 0i16);

  for i in 0..(KYBER_N/2) {
    t0 = a.coeffs[2*i];
    t1 = a.coeffs[2*i+1];
    r[3*i] = (t0 & 0xff) as u8;
    r[3*i+1] = ((t0 >> 8) | ((t1 & 0xf) << 4)) as u8;
    r[3*i+2] = (t1 >> 4) as u8;
  }
}

/*************************************************
* Name:        poly_frombytes
*
* Description: De-serialization of a polynomial;
*              inverse of poly_tobytes
*
* Arguments:   - poly *r:                pointer to output polynomial
*              - const unsigned char *a: pointer to input byte array (of KYBER_POLYBYTES bytes)
**************************************************/

pub fn poly_frombytes(r: &mut Poly, a: &[u8])
{
  for i in 0..(KYBER_N/2) {
    r.coeffs[2*i]   = ((a[3*i] as u16        | a[3*i+1] as u16 & 0x0f) << 8) as i16;
    r.coeffs[2*i+1] = ((a[3*i+1] as u16 >> 4 | a[3*i+2] as u16 & 0xff) << 4) as i16;
  }
}

/*************************************************
* Name:        poly_getnoise
*
* Description: Sample a polynomial deterministically from a seed and a nonce,
*              with output polynomial close to centered binomial distribution
*              with parameter KYBER_ETA
*
* Arguments:   - poly *r:                   pointer to output polynomial
*              - const unsigned char *seed: pointer to input seed (pointing to array of length KYBER_SYMBYTES bytes)
*              - unsigned char nonce:       one-byte input nonce
**************************************************/

pub fn poly_getnoise(r: &mut Poly, seed: &[u8], nonce: u8)
{
  const length: usize = KYBER_ETA*KYBER_N/4;
  let mut buf = [0u8; length];
  prf(&mut buf, length as u64, seed, nonce);
  cbd(r, &mut buf);
}


/*************************************************
* Name:        poly_ntt
*
* Description: Computes negacyclic number-theoretic transform (NTT) of
*              a polynomial in place;
*              inputs assumed to be in normal order, output in bitreversed order
*
* Arguments:   - uint16_t *r: pointer to in/output polynomial
**************************************************/

pub fn poly_ntt(r: &mut Poly) 
{
  ntt(&mut r.coeffs);
  poly_reduce(r);
}


/*************************************************
* Name:        poly_invntt
*
* Description: Computes inverse of negacyclic number-theoretic transform (NTT) of
*              a polynomial in place;
*              inputs assumed to be in bitreversed order, output in normal order
*
* Arguments:   - uint16_t *a: pointer to in/output polynomial
**************************************************/

pub fn poly_invntt(r: &mut Poly)
{
  invntt(&mut r.coeffs);
}


/*************************************************
* Name:        poly_basemul
*
* Description: Multiplication of two polynomials in NTT domain
*
* Arguments:   - poly *r:       pointer to output polynomial
*              - const poly *a: pointer to first input polynomial
*              - const poly *b: pointer to second input polynomial
**************************************************/

pub fn poly_basemul(r: &mut Poly, a: &Poly, b: &Poly)
{
  for i in 0..(KYBER_N/4) {
    
    basemul(
      &mut r.coeffs[4*i..], 
      &a.coeffs[4*i..],
      &b.coeffs[4*i..], 
      zetas[64 + i]
    );
    basemul(
      &mut r.coeffs[4*i+2..], 
      &a.coeffs[4*i+2..],
      &b.coeffs[4*i+2..],
-(zetas[64 + i]));
  }
}


/*************************************************
* Name:        poly_frommont
*
* Description: Inplace conversion of all coefficients of a polynomial 
*              from Montgomery domain to normal domain
*
* Arguments:   - poly *r:       pointer to input/output polynomial
**************************************************/

pub fn poly_frommont(r: &mut Poly)
{
  let f = (1u64 << 32) % KYBER_Q as u64;
  for i in 0..KYBER_N {
    r.coeffs[i] = montgomery_reduce((r.coeffs[i] as u64 * f) as i32);
  }
}

/*************************************************
* Name:        poly_reduce
*
* Description: Applies Barrett reduction to all coefficients of a polynomial
*              for details of the Barrett reduction see comments in reduce.c
*
* Arguments:   - poly *r:       pointer to input/output polynomial
**************************************************/
pub fn poly_reduce(r: &mut Poly)
{
  for i in 0..KYBER_N {
    r.coeffs[i] = barrett_reduce(r.coeffs[i]);
  }
}


/*************************************************
* Name:        poly_csubq
*
* Description: Applies conditional subtraction of q to each coefficient of a polynomial
*              for details of conditional subtraction of q see comments in reduce.c
*
* Arguments:   - poly *r:       pointer to input/output polynomial
**************************************************/
pub fn poly_csubq(r: &mut Poly)
{
  for i in 0..KYBER_N {
    r.coeffs[i] = csubq(r.coeffs[i]);
  }
}


/*************************************************
* Name:        poly_add
*
* Description: Add two polynomials
*
* Arguments: - poly *r:       pointer to output polynomial
*            - const poly *a: pointer to first input polynomial
*            - const poly *b: pointer to second input polynomial
**************************************************/
pub fn poly_add(r: &mut Poly, b: &Poly)
{
  for i in 0..KYBER_N {
    r.coeffs[i] += b.coeffs[i];
  }
}


/*************************************************
* Name:        poly_sub
*
* Description: Subtract two polynomials
*
* Arguments: - poly *r:       pointer to output polynomial
*            - const poly *a: pointer to first input polynomial
*            - const poly *b: pointer to second input polynomial
**************************************************/
pub fn  poly_sub(r: &mut Poly, a: &Poly)
{
  for i in 0..KYBER_N {
    r.coeffs[i] = a.coeffs[i] -  r.coeffs[i];
  }
}


/*************************************************
* Name:        poly_frommsg
*
* Description: Convert 32-byte message to polynomial
*
* Arguments:   - poly *r:                  pointer to output polynomial
*              - const unsigned char *msg: pointer to input message
**************************************************/
pub fn poly_frommsg(r: &mut Poly, msg: &[u8])
{
  let mut mask = 0u16;
  for i in 0..KYBER_SYMBYTES {
    for j in 0..8 {
      mask = ((msg[i] >> j)&1).wrapping_neg() as u16;
      r.coeffs[8*i+j] = (mask & ((KYBER_Q+1)/2) as u16) as i16;
    }
  }
}


/*************************************************
* Name:        poly_tomsg
*
* Description: Convert polynomial to 32-byte message
*
* Arguments:   - unsigned char *msg: pointer to output message
*              - const poly *a:      pointer to input polynomial
**************************************************/
pub fn poly_tomsg(msg: &mut[u8], a: &mut Poly)
{
  poly_csubq(a);
  let mut t = 0u16;

  for i in 0..KYBER_SYMBYTES {
    msg[i] = 0;
    for j in 0..8 {
      // TODO: Consider making KYBER_Q i16 everywhere 
      t = ((((a.coeffs[8*i+j] << 1) + (KYBER_Q/2) as i16) / KYBER_Q as i16) & 1) as u16;
      // TODO: Check conversion
      msg[i] |= (t << j) as u8;
    }
  }
}