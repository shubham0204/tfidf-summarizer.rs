use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use rayon::prelude::*;
use crate::tokenizer::Tokenizer;

pub struct Summarizer {}

impl Summarizer {

    pub fn compute( 
        text: &str , 
        reduction_factor: f32
     ) -> String {
        let sentences_owned: Vec<String> = Tokenizer::text_to_sentences( text ) ; 
        let mut sentences: Vec<&str> = sentences_owned
                                                .iter()
                                                .map( |s| s.as_str() )
                                                .collect() ; 
        let mut tokens: Vec<Vec<&str>> = Vec::new() ; 
        for sentence in sentences.iter() {
            tokens.push( Tokenizer::sentence_to_tokens(sentence) ) 
        }

        let mut sentence_scores: HashMap<&str,f32> = HashMap::new() ; 
        let mut i: usize = 0;
        for tokenized_sentence in tokens.iter() {
            let tf: HashMap<&str,f32> = Summarizer::compute_term_frequency(tokenized_sentence) ; 
            let idf: HashMap<&str,f32> = Summarizer::compute_inverse_doc_frequency(tokenized_sentence, &tokens) ; 
            let mut tfidf_sum: f32 = 0.0 ; 
            
            for word in tokenized_sentence.iter() {
                tfidf_sum += tf.get( word ).unwrap() * idf.get( word ).unwrap() ; 
            }
            sentence_scores.insert( sentences[i] , tfidf_sum ) ; 
            i += 1
        }
        
        sentences.sort_by( | a , b | 
            sentence_scores.get(b).unwrap().total_cmp(sentence_scores.get(a).unwrap()) ) ; 

        let num_summary_sents = (reduction_factor * (sentences.len() as f32) ) as usize;
        let summary = sentences[ 0..num_summary_sents ].join( " " ) ;

        summary
    }

    pub fn par_compute( 
        text: &str , 
        reduction_factor: f32
     ) -> String {
        let sentences_owned: Vec<String> = Tokenizer::text_to_sentences( text ) ; 
        let mut sentences: Vec<&str> = sentences_owned
                                                .iter()
                                                .map( |s| s.as_str() )
                                                .collect() ; 

        let tokens_ptr: Arc<Mutex<Vec<Vec<&str>>>> = Arc::new( Mutex::new( Vec::new() ) ) ; 
        sentences.par_iter()
                 .for_each( |sentence| { 
                    let sent_tokens: Vec<&str> = Tokenizer::sentence_to_tokens(sentence) ; 
                    tokens_ptr.lock().unwrap().push( sent_tokens ) ; 
                 } ) ; 
        let tokens = tokens_ptr.lock().unwrap() ; 

        let sentence_scores_ptr: Arc<Mutex<HashMap<&str,f32>>> = Arc::new( Mutex::new( HashMap::new() ) ) ; 
        tokens.par_iter()
              .zip( sentences.par_iter() )
              .for_each( |(tokenized_sentence , sentence)| {
            let tf: HashMap<&str,f32> = Summarizer::compute_term_frequency(tokenized_sentence) ; 
            let idf: HashMap<&str,f32> = Summarizer::compute_inverse_doc_frequency(tokenized_sentence, &tokens ) ; 
            let mut tfidf_sum: f32 = 0.0 ; 
            
            for word in tokenized_sentence.iter() {
                tfidf_sum += tf.get( word ).unwrap() * idf.get( word ).unwrap() ; 
            }
            tfidf_sum = tfidf_sum / (tokenized_sentence.len() as f32) ; 
            sentence_scores_ptr.lock().unwrap().insert( sentence , tfidf_sum ) ; 
        } ) ; 
        let sentence_scores = sentence_scores_ptr.lock().unwrap() ;
        
        sentences.sort_by( | a , b | 
            sentence_scores.get(b).unwrap().total_cmp(sentence_scores.get(a).unwrap()) ) ; 

        let num_summary_sents = (reduction_factor * (sentences.len() as f32) ) as usize;
        let summary = sentences[ 0..num_summary_sents ].join( ". " ) ;

        summary
    }

    fn compute_term_frequency<'a>(
        tokenized_sentence: &'a Vec<&str>
    ) -> HashMap<&'a str,f32> {
        let words_frequencies = Tokenizer::get_freq_map( tokenized_sentence ) ;
        let mut term_frequency: HashMap<&str,f32> = HashMap::new() ;  
        let num_tokens = tokenized_sentence.len() ; 
        for (word , count) in words_frequencies {
            term_frequency.insert( word , ( count as f32 ) / ( num_tokens as f32 ) ) ; 
        }

        term_frequency
    }

    fn compute_inverse_doc_frequency<'a>(
        tokenized_sentence: &'a Vec<&str> ,
        tokens: &'a Vec<Vec<&'a str>>
    ) -> HashMap<&'a str,f32> {

        let num_docs = tokens.len() as f32 ; 
        let mut idf: HashMap<&str,f32> = HashMap::new() ; 

        for word in tokenized_sentence {
            let mut word_count_in_docs: usize = 0 ; 
            for doc in tokens.iter() {
                word_count_in_docs += doc.iter().filter( |&token| token == word ).count()
            }
            idf.insert( word , ( (num_docs) / (word_count_in_docs as f32) ).log10() ) ;
        }

        idf
    }

}