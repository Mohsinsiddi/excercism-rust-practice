// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
      let magazine_words = count_words(magazine);
      let note_words = count_words(note);

      for (word,word_count) in note_words {
        let magazine_word_count = magazine_words.get(word).unwrap_or(&0);
        if *magazine_word_count < word_count {
            return  false;
        }
      }
      return true;
}

fn count_words<'a>(text: &[& 'a str]) -> HashMap<&'a str,u32>{
     let mut word_count = HashMap::new();
     for word in text {
        let mut count = word_count.entry(*word).or_insert(0);
        *count+=1;
     }
     word_count
}