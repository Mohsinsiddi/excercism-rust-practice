pub fn abbreviate(phrase: &str) -> String {
   let phrase = phrase.trim();

   let phrase_words = phrase.split(" ").collect();

   let mut ans = String::from("");

   for words in  phrase_words {
    let c = words.chars().collect::<Vec<char>>();
       ans+=c[0]
   }

   ans
}
