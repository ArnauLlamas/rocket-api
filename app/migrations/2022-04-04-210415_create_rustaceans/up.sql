-- Your SQL goes here
CREATE TABLE rustaceans (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(191) NOT NULL,
  email VARCHAR(191) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)