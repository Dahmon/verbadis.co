CREATE TABLE words (
  id INTEGER PRIMARY KEY,
  word TEXT NOT NULL,
  class TEXT CHECK(class IN ('noun', 'verb', 'adjective', 'adverb', 'interjection')) NOT NULL,
  definition TEXT NOT NULL,
  example TEXT NOT NULL
);

CREATE TABLE challenges (
  id INTEGER PRIMARY KEY,
  word_id INTEGER NOT NULL,
  answer TEXT,
  corrected_answer TEXT,
  score INTEGER,
  FOREIGN KEY (word_id) REFERENCES words(id)
);
