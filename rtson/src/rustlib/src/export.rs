// Generated by using rustinr::rustrize() -> do not edit by hand

use super::*;

#[no_mangle]
pub extern "C" fn rustr_from_tson(rbytes : SEXP)->SEXP{

 let rbytes_ : RawVec = unwrapr!( RawVec::rnew(rbytes) );
 let res  = unwrapr!( from_tson(rbytes_));

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}
