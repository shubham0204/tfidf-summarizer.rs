use std::collections::HashMap;
use punkt::{SentenceTokenizer, TrainingData};
use punkt::params::Standard;

static KEYWORDS: [ &str ; 127 ] = [ "i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", 
    "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her", "hers", "herself", 
    "it", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this",
     "that", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", 
     "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", 
     "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above",
     "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once",
       "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", 
       "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can",
        "will", "just", "don", "should", "now" ] ;

pub struct Tokenizer {}

impl Tokenizer {

    pub fn text_to_sentences( text: &str ) -> Vec<String> {
        let english = TrainingData::english();
        let mut sentences: Vec<String> = Vec::new() ; 
        for s in SentenceTokenizer::<Standard>::new(text, &english) {
            sentences.push( s.to_owned() ) ; 
        }
        sentences
    }

    pub fn sentence_to_tokens( sentence: &str ) -> Vec<&str> {
        let tokens: Vec<&str> = sentence.split_ascii_whitespace().collect() ; 
        let filtered_tokens: Vec<&str> = tokens
                                    .into_iter()
                                    .filter( |token| !KEYWORDS.contains( &token.to_lowercase().as_str() ) )
                                    .collect() ;
        filtered_tokens
    }

    pub fn get_freq_map<'a>( words: &'a Vec<&'a str> ) -> HashMap<&'a str,usize> {
        let mut freq_map: HashMap<&str,usize> = HashMap::new() ; 
        for word in words.iter() {
            if freq_map.contains_key( word ) {
                freq_map
                    .entry( word )
                    .and_modify( | e | { 
                        *e = *e + 1 
                    } ) ; 
            }
            else {
                freq_map.insert( *word , 1 ) ; 
            }
        }
        freq_map
    }

}