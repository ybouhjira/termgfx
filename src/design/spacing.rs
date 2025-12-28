//! Spacing scale system for consistent layout
//!
//! Provides a 6-level spacing scale that can be used throughout the codebase
//! for consistent padding, margins, and gaps.
//!
//! # Example
//! ```rust
//! use termgfx::design::spacing::{SpacingLevel, sp, Spacing};
//!
//! // Get a specific spacing value
//! let padding = sp(SpacingLevel::Sm);  // Returns 2
//!
//! // Use the Spacing struct for full configuration
//! let spacing = Spacing::default();
//! let margin = spacing.get(SpacingLevel::Md);  // Returns 4
//!
//! // Apply a multiplier for responsive layouts
//! let scaled = spacing.with_multiplier(2.0);
//! let large_padding = scaled.get(SpacingLevel::Sm);  // Returns 4
//! ```

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Spacing level enum for consistent spacing across the codebase
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SpacingLevel {
    /// No spacing (0)
    None = 0,
    /// Extra small - tight spacing (1)
    Xs = 1,
    /// Small - compact spacing (2)
    Sm = 2,
    /// Medium - default spacing (4)
    Md = 3,
    /// Large - spacious spacing (8)
    Lg = 4,
    /// Extra large - dramatic spacing (16)
    Xl = 5,
}

impl SpacingLevel {
    /// Get all spacing levels in order
    pub fn all() -> Vec<SpacingLevel> {
        vec![
            SpacingLevel::None,
            SpacingLevel::Xs,
            SpacingLevel::Sm,
            SpacingLevel::Md,
            SpacingLevel::Lg,
            SpacingLevel::Xl,
        ]
    }

    /// Get the default value for this level
    pub fn default_value(&self) -> usize {
        match self {
            SpacingLevel::None => 0,
            SpacingLevel::Xs => 1,
            SpacingLevel::Sm => 2,
            SpacingLevel::Md => 4,
            SpacingLevel::Lg => 8,
            SpacingLevel::Xl => 16,
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "none" | "0" => Some(SpacingLevel::None),
            "xs" | "extra-small" | "1" => Some(SpacingLevel::Xs),
            "sm" | "small" | "2" => Some(SpacingLevel::Sm),
            "md" | "medium" | "default" | "4" => Some(SpacingLevel::Md),
            "lg" | "large" | "8" => Some(SpacingLevel::Lg),
            "xl" | "extra-large" | "16" => Some(SpacingLevel::Xl),
            _ => None,
        }
    }

    /// Get the level name
    pub fn as_str(&self) -> &'static str {
        match self {
            SpacingLevel::None => "none",
            SpacingLevel::Xs => "xs",
            SpacingLevel::Sm => "sm",
            SpacingLevel::Md => "md",
            SpacingLevel::Lg => "lg",
            SpacingLevel::Xl => "xl",
        }
    }

    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            SpacingLevel::None => "No spacing (0)",
            SpacingLevel::Xs => "Extra small - tight (1)",
            SpacingLevel::Sm => "Small - compact (2)",
            SpacingLevel::Md => "Medium - default (4)",
            SpacingLevel::Lg => "Large - spacious (8)",
            SpacingLevel::Xl => "Extra large - dramatic (16)",
        }
    }
}

/// Spacing configuration with customizable values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    /// No spacing value
    pub none: usize,
    /// Extra small spacing
    pub xs: usize,
    /// Small spacing
    pub sm: usize,
    /// Medium spacing
    pub md: usize,
    /// Large spacing
    pub lg: usize,
    /// Extra large spacing
    pub xl: usize,
    /// Multiplier for scaling all values
    #[serde(default = "default_multiplier")]
    pub multiplier: f64,
}

fn default_multiplier() -> f64 {
    1.0
}

impl Default for Spacing {
    fn default() -> Self {
        Spacing {
            none: 0,
            xs: 1,
            sm: 2,
            md: 4,
            lg: 8,
            xl: 16,
            multiplier: 1.0,
        }
    }
}

impl Spacing {
    /// Create a new spacing configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a compact spacing configuration
    pub fn compact() -> Self {
        Spacing {
            none: 0,
            xs: 0,
            sm: 1,
            md: 2,
            lg: 4,
            xl: 8,
            multiplier: 1.0,
        }
    }

    /// Create a spacious spacing configuration
    pub fn spacious() -> Self {
        Spacing {
            none: 0,
            xs: 2,
            sm: 4,
            md: 8,
            lg: 16,
            xl: 32,
            multiplier: 1.0,
        }
    }

    /// Get the spacing value for a level
    pub fn get(&self, level: SpacingLevel) -> usize {
        let base = match level {
            SpacingLevel::None => self.none,
            SpacingLevel::Xs => self.xs,
            SpacingLevel::Sm => self.sm,
            SpacingLevel::Md => self.md,
            SpacingLevel::Lg => self.lg,
            SpacingLevel::Xl => self.xl,
        };
        (base as f64 * self.multiplier).round() as usize
    }

    /// Create a new spacing with a multiplier applied
    pub fn with_multiplier(&self, multiplier: f64) -> Self {
        Spacing {
            multiplier,
            ..self.clone()
        }
    }

    /// Set a custom value for a level
    pub fn set(&mut self, level: SpacingLevel, value: usize) {
        match level {
            SpacingLevel::None => self.none = value,
            SpacingLevel::Xs => self.xs = value,
            SpacingLevel::Sm => self.sm = value,
            SpacingLevel::Md => self.md = value,
            SpacingLevel::Lg => self.lg = value,
            SpacingLevel::Xl => self.xl = value,
        }
    }
}

/// Quick access to default spacing values
///
/// # Example
/// ```rust
/// use termgfx::design::spacing::{SpacingLevel, sp};
///
/// let padding = sp(SpacingLevel::Sm);  // Returns 2
/// let margin = sp(SpacingLevel::Md);   // Returns 4
/// ```
pub fn sp(level: SpacingLevel) -> usize {
    level.default_value()
}

/// Quick access with multiplier
///
/// # Example
/// ```rust
/// use termgfx::design::spacing::{SpacingLevel, spm};
///
/// let padding = spm(SpacingLevel::Sm, 2.0);  // Returns 4
/// ```
pub fn spm(level: SpacingLevel, multiplier: f64) -> usize {
    (level.default_value() as f64 * multiplier).round() as usize
}

/// Spacing helper for component configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingConfig {
    pub padding: SpacingLevel,
    pub margin: SpacingLevel,
    pub gap: SpacingLevel,
}

impl Default for SpacingConfig {
    fn default() -> Self {
        SpacingConfig {
            padding: SpacingLevel::Sm,
            margin: SpacingLevel::None,
            gap: SpacingLevel::Xs,
        }
    }
}

impl SpacingConfig {
    /// Create a tight configuration for dense layouts
    pub fn tight() -> Self {
        SpacingConfig {
            padding: SpacingLevel::Xs,
            margin: SpacingLevel::None,
            gap: SpacingLevel::None,
        }
    }

    /// Create a comfortable configuration for regular use
    pub fn comfortable() -> Self {
        SpacingConfig {
            padding: SpacingLevel::Md,
            margin: SpacingLevel::Sm,
            gap: SpacingLevel::Sm,
        }
    }

    /// Create a spacious configuration for dramatic layouts
    pub fn spacious() -> Self {
        SpacingConfig {
            padding: SpacingLevel::Lg,
            margin: SpacingLevel::Md,
            gap: SpacingLevel::Md,
        }
    }

    /// Get padding value using default scale
    pub fn padding_value(&self) -> usize {
        sp(self.padding)
    }

    /// Get margin value using default scale
    pub fn margin_value(&self) -> usize {
        sp(self.margin)
    }

    /// Get gap value using default scale
    pub fn gap_value(&self) -> usize {
        sp(self.gap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_level_default_values() {
        assert_eq!(SpacingLevel::None.default_value(), 0);
        assert_eq!(SpacingLevel::Xs.default_value(), 1);
        assert_eq!(SpacingLevel::Sm.default_value(), 2);
        assert_eq!(SpacingLevel::Md.default_value(), 4);
        assert_eq!(SpacingLevel::Lg.default_value(), 8);
        assert_eq!(SpacingLevel::Xl.default_value(), 16);
    }

    #[test]
    fn test_spacing_level_from_str() {
        assert_eq!(SpacingLevel::from_str("none"), Some(SpacingLevel::None));
        assert_eq!(SpacingLevel::from_str("xs"), Some(SpacingLevel::Xs));
        assert_eq!(SpacingLevel::from_str("sm"), Some(SpacingLevel::Sm));
        assert_eq!(SpacingLevel::from_str("MD"), Some(SpacingLevel::Md));
        assert_eq!(SpacingLevel::from_str("large"), Some(SpacingLevel::Lg));
        assert_eq!(
            SpacingLevel::from_str("extra-large"),
            Some(SpacingLevel::Xl)
        );
        assert_eq!(SpacingLevel::from_str("invalid"), None);
    }

    #[test]
    fn test_spacing_level_as_str() {
        assert_eq!(SpacingLevel::None.as_str(), "none");
        assert_eq!(SpacingLevel::Xs.as_str(), "xs");
        assert_eq!(SpacingLevel::Sm.as_str(), "sm");
        assert_eq!(SpacingLevel::Md.as_str(), "md");
        assert_eq!(SpacingLevel::Lg.as_str(), "lg");
        assert_eq!(SpacingLevel::Xl.as_str(), "xl");
    }

    #[test]
    fn test_spacing_default() {
        let spacing = Spacing::default();
        assert_eq!(spacing.get(SpacingLevel::None), 0);
        assert_eq!(spacing.get(SpacingLevel::Xs), 1);
        assert_eq!(spacing.get(SpacingLevel::Sm), 2);
        assert_eq!(spacing.get(SpacingLevel::Md), 4);
        assert_eq!(spacing.get(SpacingLevel::Lg), 8);
        assert_eq!(spacing.get(SpacingLevel::Xl), 16);
    }

    #[test]
    fn test_spacing_compact() {
        let spacing = Spacing::compact();
        assert_eq!(spacing.get(SpacingLevel::Sm), 1);
        assert_eq!(spacing.get(SpacingLevel::Md), 2);
        assert_eq!(spacing.get(SpacingLevel::Lg), 4);
    }

    #[test]
    fn test_spacing_spacious() {
        let spacing = Spacing::spacious();
        assert_eq!(spacing.get(SpacingLevel::Sm), 4);
        assert_eq!(spacing.get(SpacingLevel::Md), 8);
        assert_eq!(spacing.get(SpacingLevel::Lg), 16);
    }

    #[test]
    fn test_spacing_with_multiplier() {
        let spacing = Spacing::default().with_multiplier(2.0);
        assert_eq!(spacing.get(SpacingLevel::Sm), 4);
        assert_eq!(spacing.get(SpacingLevel::Md), 8);
        assert_eq!(spacing.get(SpacingLevel::Lg), 16);
    }

    #[test]
    fn test_sp_function() {
        assert_eq!(sp(SpacingLevel::None), 0);
        assert_eq!(sp(SpacingLevel::Sm), 2);
        assert_eq!(sp(SpacingLevel::Md), 4);
    }

    #[test]
    fn test_spm_function() {
        assert_eq!(spm(SpacingLevel::Sm, 2.0), 4);
        assert_eq!(spm(SpacingLevel::Md, 0.5), 2);
        assert_eq!(spm(SpacingLevel::Lg, 1.5), 12);
    }

    #[test]
    fn test_spacing_config_default() {
        let config = SpacingConfig::default();
        assert_eq!(config.padding, SpacingLevel::Sm);
        assert_eq!(config.margin, SpacingLevel::None);
        assert_eq!(config.gap, SpacingLevel::Xs);
    }

    #[test]
    fn test_spacing_config_values() {
        let config = SpacingConfig::comfortable();
        assert_eq!(config.padding_value(), 4);
        assert_eq!(config.margin_value(), 2);
        assert_eq!(config.gap_value(), 2);
    }

    #[test]
    fn test_spacing_set() {
        let mut spacing = Spacing::default();
        spacing.set(SpacingLevel::Sm, 3);
        assert_eq!(spacing.get(SpacingLevel::Sm), 3);
    }

    #[test]
    fn test_spacing_level_all() {
        let levels = SpacingLevel::all();
        assert_eq!(levels.len(), 6);
        assert_eq!(levels[0], SpacingLevel::None);
        assert_eq!(levels[5], SpacingLevel::Xl);
    }

    #[test]
    fn test_spacing_serialization() {
        let spacing = Spacing::default();
        let json = serde_json::to_string(&spacing).unwrap();
        let deserialized: Spacing = serde_json::from_str(&json).unwrap();
        assert_eq!(spacing.sm, deserialized.sm);
        assert_eq!(spacing.md, deserialized.md);
    }

    #[test]
    fn test_spacing_config_tight() {
        let config = SpacingConfig::tight();
        assert_eq!(config.padding_value(), 1);
        assert_eq!(config.margin_value(), 0);
        assert_eq!(config.gap_value(), 0);
    }

    #[test]
    fn test_spacing_config_spacious() {
        let config = SpacingConfig::spacious();
        assert_eq!(config.padding_value(), 8);
        assert_eq!(config.margin_value(), 4);
        assert_eq!(config.gap_value(), 4);
    }
}
