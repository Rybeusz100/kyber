

fn main() {

  #[cfg(not(any(feature = "reference", feature = "wasm")))]
  {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
      // if std::is_x86_feature_detected!("avx2") {
        const ROOT: &str = "src/avx2/";
        const FILES: [&str; 5] = ["basemul.S", "fq.S", "invntt.S", "ntt.S", "shuffle.S"];
    
        // Separate asm files export underscored symbols for Apple
        fn filepath(name: &str) -> String {
          if cfg!(target_vendor = "apple") 
          {
            format!("{}_{}", ROOT, name) 
          } else {
            format!("{}{}", ROOT, name) 
          }
        }
    
        let paths = FILES.iter().map(|x| filepath(x));
        cc::Build::new()
          .include(ROOT)
          .files(paths)
          .compile("pqc_kyber");
      }
    // }

  }
}