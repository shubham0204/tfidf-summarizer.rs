mod tokenizer;
mod summarizer;

use self::summarizer::Summarizer;

pub fn summarize( text: &str , reduction_factor: f32 ) -> String {
    Summarizer::compute(text, reduction_factor)
}

pub fn par_summarize( text: &str , reduction_factor: f32 ) -> String {
    Summarizer::par_compute(text, reduction_factor)
}

mod c_binding {

    use std::ffi::CString;
    use crate::summarizer::Summarizer;

    #[no_mangle]
    pub extern "C" fn summarize( text: *const u8 , length: usize , reduction_factor: f32 ) -> *const u8 {
        unsafe {
            match std::str::from_utf8( std::slice::from_raw_parts( text , length ) ) {
                Ok( text ) => {
                    let summary = Summarizer::compute(text, reduction_factor) ;
                    let c_summary = CString::new( summary ).unwrap() ;
                    let c_summary_ptr = c_summary.as_ptr() ; 
                    std::mem::forget( c_summary );
                    c_summary_ptr as *const u8
                } , 
                Err( e ) => {
                    let c_summary = CString::new( e.to_string() ).unwrap() ;
                    c_summary.as_ptr() as *const u8
                }
            }
        }    
    }

    #[no_mangle]
    pub extern "C" fn par_summarize( text: *const u8 , length: usize , reduction_factor: f32 ) -> *const u8 {
        unsafe {
            match std::str::from_utf8( std::slice::from_raw_parts( text , length ) ) {
                Ok( text ) => {
                    let summary = Summarizer::par_compute(text, reduction_factor) ;
                    let c_summary = CString::new( summary ).unwrap() ;
                    let c_summary_ptr = c_summary.as_ptr() ; 
                    std::mem::forget( c_summary );
                    c_summary_ptr as *const u8
                } , 
                Err( e ) => {
                    let c_summary = CString::new( e.to_string() ).unwrap() ;
                    c_summary.as_ptr() as *const u8
                }
            }
        }    
    }

}
