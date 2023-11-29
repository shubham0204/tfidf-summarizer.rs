mod tokenizer;
mod summarizer;

use self::summarizer::Summarizer;

pub fn summarize( text: &str , reduction_factor: f32 ) -> String {
    Summarizer::compute(text, reduction_factor)
}

pub fn par_summarize( text: &str , reduction_factor: f32 ) -> String {
    Summarizer::par_compute(text, reduction_factor)
}

/// functions exposing Rust methods as C interfaces
/// These methods are accessible with the ABI (compiled object code)
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

                    // Eliminate `c_summary` from reference/ownership tracking
                    // hence transferring the ownership to the calling program
                    std::mem::forget( c_summary );

                    c_summary_ptr as *const u8
                } , 
                Err( e ) => {
                    // Return an empty string as a summary if error occurred
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

                    // Eliminate `c_summary` from reference/ownership tracking
                    // hence transferring the ownership to the calling program
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

/// JNI methods for using in Android
/// `Cargo.toml` has a conditional dependence of `jni` for this module
#[cfg(feature="android")]
mod android {

    extern crate jni ; 
    use jni::objects::{JClass, JString};
    use jni::sys::jfloat;
    use jni::JNIEnv;
    use crate::summarize ;
    use crate::par_summarize ;

    #[no_mangle]
    pub extern "C" fn Java_com_projects_ml_summarizer_Summarizer_summarize<'a>(
        mut env: JNIEnv<'a>,
        _: JClass<'a>,
        text: JString<'a>,
        reduction_factor: jfloat
    ) -> JString<'a> {
        let text: String = env
            .get_string(&text)
            .expect("Could not open text in summarize")
            .into();
        let summary = summarize( text.as_str() , reduction_factor ) ; 
        let output = env
            .new_string( summary )
            .expect("Could not create output string");
        output
    }

    #[no_mangle]
    pub extern "C" fn Java_com_projects_ml_summarizer_Summarizer_parallelSummarize<'a>(
        mut env: JNIEnv<'a>,
        _: JClass<'a>,
        text: JString<'a>,
        reduction_factor: jfloat
    ) -> JString<'a> {
        let text: String = env
            .get_string(&text)
            .expect("Could not open text in par_summarize")
            .into();
        let summary = par_summarize( text.as_str() , reduction_factor ) ; 
        let output = env
            .new_string( summary )
            .expect("Could not create output string");
        output
    }

}
