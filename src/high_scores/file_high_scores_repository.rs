use crate::game_logic::GameResult;
// src/high_scores/file_repository.rs
use crate::high_scores::{HighScore, HighScores, HighScoresRepository};
use std::fs;
use std::path::Path;

pub struct FileHighScoresRepository {
    file_path: String,
    encryption_key: u8,
}

impl FileHighScoresRepository {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            encryption_key: 42, // Simple XOR key - not secure but deters casual editing
        }
    }

    fn encrypt_decrypt(&self, data: &[u8]) -> Vec<u8> {
        data.iter().map(|b| b ^ self.encryption_key).collect()
    }

    fn calculate_checksum(&self, data: &str) -> u32 {
        // Simple checksum - sum of all bytes XOR'd with a secret
        let secret = 0xDEADBEEF_u32;
        let sum: u32 = data.bytes().map(|b| b as u32).sum();
        sum ^ secret
    }

    fn add_checksum(&self, data: &str) -> String {
        let checksum = self.calculate_checksum(data);
        format!("CHKSUM:{checksum}\n{data}")
    }

    fn verify_and_extract(&self, data: &str) -> Result<String, String> {
        let lines: Vec<&str> = data.splitn(2, '\n').collect();
        if lines.len() != 2 {
            return Err("Invalid file format - missing checksum".to_string());
        }

        let checksum_line = lines[0];
        let content = lines[1];

        if !checksum_line.starts_with("CHKSUM:") {
            return Err("Invalid file format - no checksum header".to_string());
        }

        let stored_checksum: u32 = checksum_line[7..]
            .parse()
            .map_err(|_| "Invalid checksum format".to_string())?;

        let calculated_checksum = self.calculate_checksum(content);
        if stored_checksum != calculated_checksum {
            return Err("File has been tampered with - checksum mismatch".to_string());
        }

        Ok(content.to_string())
    }

    fn serialize_high_scores(&self, high_scores: &HighScores) -> String {
        let scores = high_scores.get_scores();
        let mut lines = Vec::new();

        for score in scores {
            lines.push(format!(
                "{}|{}|{}",
                score.name, score.game_result.score, score.game_result.level
            ));
        }

        lines.join("\n")
    }

    fn deserialize_high_scores(&self, data: &str) -> Result<HighScores, String> {
        let mut scores = Vec::new();

        for line in data.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 3 {
                return Err(format!("Invalid line format: {line}"));
            }

            let name = parts[0].to_string();
            let score = parts[1]
                .parse::<u32>()
                .map_err(|_| format!("Invalid score: {}", parts[1]))?;
            let level = parts[2]
                .parse::<u32>()
                .map_err(|_| format!("Invalid level: {}", parts[2]))?;

            scores.push(HighScore::new(name, GameResult { score, level }));
        }

        Ok(HighScores::from_vec(scores))
    }
}

impl HighScoresRepository for FileHighScoresRepository {
    fn load(&self) -> Result<HighScores, String> {
        if !Path::new(&self.file_path).exists() {
            return Ok(HighScores::new());
        }

        let encrypted_data =
            fs::read(&self.file_path).map_err(|e| format!("Failed to read file: {e}"))?;

        let decrypted_data = self.encrypt_decrypt(&encrypted_data);
        let data_str =
            String::from_utf8(decrypted_data).map_err(|e| format!("Invalid UTF-8 data: {e}"))?;

        let verified_content = self.verify_and_extract(&data_str)?;
        self.deserialize_high_scores(&verified_content)
    }

    fn save(&self, high_scores: &HighScores) -> Result<(), String> {
        let serialized = self.serialize_high_scores(high_scores);
        let with_checksum = self.add_checksum(&serialized);
        let encrypted_data = self.encrypt_decrypt(with_checksum.as_bytes());

        fs::write(&self.file_path, encrypted_data).map_err(|e| format!("Failed to write file: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn serialize_deserialize_round_trip() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let mut high_scores = HighScores::new();
        high_scores.add(HighScore::new(
            "SAM".to_string(),
            GameResult {
                score: 1000,
                level: 5,
            },
        ));
        high_scores.add(HighScore::new(
            "BOB".to_string(),
            GameResult {
                score: 2000,
                level: 3,
            },
        ));

        // Act
        let serialized = sut.serialize_high_scores(&high_scores);
        let result = sut.deserialize_high_scores(&serialized);

        // Assert
        assert!(result.is_ok());
        let deserialized = result.unwrap();
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized.get_scores()[0].name, "BOB"); // Should be sorted by score
        assert_eq!(deserialized.get_scores()[0].game_result.score, 2000);
        assert_eq!(deserialized.get_scores()[1].name, "SAM");
        assert_eq!(deserialized.get_scores()[1].game_result.score, 1000);
    }

    #[test]
    fn encrypt_decrypt_round_trip() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let original = b"Hello World!";

        // Act
        let encrypted = sut.encrypt_decrypt(original);
        let decrypted = sut.encrypt_decrypt(&encrypted);

        // Assert
        assert_eq!(original, decrypted.as_slice());
    }

    #[test]
    fn checksum_detects_tampering() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let original = "SAM|1000|5";
        let tampered = "SAM|9999|5"; // Modified score

        // Act
        let original_checksum = sut.calculate_checksum(original);
        let tampered_checksum = sut.calculate_checksum(tampered);

        // Assert
        assert_ne!(original_checksum, tampered_checksum);
    }

    #[test]
    fn verify_and_extract_rejects_tampered_data() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let original_data = "SAM|1000|5";
        let with_checksum = sut.add_checksum(original_data);

        // Tamper with the content but keep the original checksum
        let tampered = with_checksum.replace("1000", "9999");

        // Act
        let result = sut.verify_and_extract(&tampered);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("checksum mismatch"));
    }

    #[test]
    fn load_returns_empty_when_file_does_not_exist() {
        // Arrange
        let sut = FileHighScoresRepository::new("nonexistent.dat".to_string());

        // Act
        let result = sut.load();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn save_and_load_persistence() {
        // Arrange
        let test_file = "test_high_scores.dat";
        let sut = FileHighScoresRepository::new(test_file.to_string());
        let mut high_scores = HighScores::new();
        high_scores.add(HighScore::new(
            "TEST".to_string(),
            GameResult {
                score: 500,
                level: 2,
            },
        ));

        // Act
        let save_result = sut.save(&high_scores);
        let load_result = sut.load();

        // Assert
        assert!(save_result.is_ok());
        assert!(load_result.is_ok());

        let loaded = load_result.unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded.get_scores()[0].name, "TEST");
        assert_eq!(loaded.get_scores()[0].game_result.score, 500);
        assert_eq!(loaded.get_scores()[0].game_result.level, 2);

        // Cleanup
        if Path::new(test_file).exists() {
            fs::remove_file(test_file).ok();
        }
    }

    #[test]
    fn add_checksum_and_verify_round_trip() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let original_data = "SAM|1000|5\nBOB|2000|3";

        // Act
        let with_checksum = sut.add_checksum(original_data);
        let result = sut.verify_and_extract(&with_checksum);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), original_data);
    }

    #[test]
    fn verify_and_extract_returns_error_for_missing_newline() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let data_without_newline = "CHKSUM:12345";

        // Act
        let result = sut.verify_and_extract(data_without_newline);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing checksum"));
    }

    #[test]
    fn verify_and_extract_returns_error_for_missing_checksum_header() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let data_without_header = "12345\ncontent";

        // Act
        let result = sut.verify_and_extract(data_without_header);

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("no checksum header"));
    }

    #[test]
    fn deserialize_high_scores_skips_empty_lines() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let data_with_empty_lines = "SAM|1000|5\n\n\nBOB|2000|3\n   \n";

        // Act
        let result = sut.deserialize_high_scores(data_with_empty_lines);

        // Assert
        assert!(result.is_ok());
        let high_scores = result.unwrap();
        assert_eq!(high_scores.len(), 2);
        assert_eq!(high_scores.get_scores()[0].name, "BOB"); // Sorted by score
        assert_eq!(high_scores.get_scores()[1].name, "SAM");
    }

    #[test]
    fn deserialize_high_scores_returns_error_for_invalid_line_format() {
        // Arrange
        let sut = FileHighScoresRepository::new("test.dat".to_string());
        let data_with_invalid_format = "SAM|1000|5\nINVALID_LINE\nBOB|2000|3";

        // Act
        let result = sut.deserialize_high_scores(data_with_invalid_format);

        // Assert
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Invalid line format: INVALID_LINE"));
    }
}
