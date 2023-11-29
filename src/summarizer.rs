use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use rayon::prelude::*;
use crate::tokenizer::Tokenizer;

pub struct Summarizer {}

impl Summarizer {

    /// Extracts summary from the given `text` with length of the summary 
    /// controlled by `reduction_factor`
    /// 
    /// # Arguments
    /// * `text`: A reference to the text/document that has to summarized
    /// * `reduction_factor`: Proportion of sentences that have to be included in the summary. For instance, if `text` contains
    /// 10 sentences and `reduction_factor=0.6`, then the extracted summary will contain 6 sentences (i.e. 60%)
    /// 
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

        // Sort sentences by their scores
        sentences.sort_by( | a , b | 
            sentence_scores.get(b).unwrap().total_cmp(sentence_scores.get(a).unwrap()) ) ; 

        // Compute number of sentences to be included in the summary
        // and return the extracted summary
        let num_summary_sents = (reduction_factor * (sentences.len() as f32) ) as usize;
        let summary = sentences[ 0..num_summary_sents ].join( " " ) ;
        summary
    }

    /// Extracts summary from the given `text` with length of the summary 
    /// controlled by `reduction_factor`. It utilizes multiple threads to perform 
    /// summarization faster on larger texts.
    /// 
    /// # Arguments
    /// * `text`: A reference to the text/document that has to summarized
    /// * `reduction_factor`: Proportion of sentences that have to be included in the summary. For instance, if `text` contains
    /// 10 sentences and `reduction_factor=0.6`, then the extracted summary will contain 6 sentences (i.e. 60%)
    /// 
    pub fn par_compute( 
        text: &str , 
        reduction_factor: f32
     ) -> String {
        let sentences_owned: Vec<String> = Tokenizer::text_to_sentences( text ) ; 
        let mut sentences: Vec<&str> = sentences_owned
                                                .iter()
                                                .map( |s| s.as_str() )
                                                .collect() ; 
        
        // Tokenize sentences in parallel with Rayon
        // Declare a thread-safe Vec<Vec<&str>> to hold the tokenized sentences
        let tokens_ptr: Arc<Mutex<Vec<Vec<&str>>>> = Arc::new( Mutex::new( Vec::new() ) ) ; 
        sentences.par_iter()
                 .for_each( |sentence| { 
                    let sent_tokens: Vec<&str> = Tokenizer::sentence_to_tokens(sentence) ; 
                    tokens_ptr.lock().unwrap().push( sent_tokens ) ; 
                 } ) ; 
        let tokens = tokens_ptr.lock().unwrap() ; 

        // Compute scores for sentences in parallel
        // Declare a thread-safe Hashmap<&str,f32> to hold the sentence scores
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

        // Sort sentences by their scores
        sentences.sort_by( | a , b | 
            sentence_scores.get(b).unwrap().total_cmp(sentence_scores.get(a).unwrap()) ) ; 

        // Compute number of sentences to be included in the summary
        // and return the extracted summary
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