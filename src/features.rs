// Feature Flag System
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use crate::error::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Features {
    #[serde(default)]
    pub features: FeatureFlags,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeatureFlags {
    #[serde(default)]
    pub cortex_plugins: bool,
    #[serde(default)]
    pub passive_sigint: bool,
    #[serde(default)]
    pub sentry_mode: bool,
    #[serde(default)]
    pub mta: bool,
    #[serde(default)]
    pub triangulate: bool,
    #[serde(default)]
    pub the_mule: bool,
    #[serde(default)]
    pub the_hydra: bool,
    #[serde(default)]
    pub sonar: bool,
    #[serde(default)]
    pub mirage: bool,
    #[serde(default)]
    pub the_mole: bool,
    #[serde(default)]
    pub spectrum_agility: bool,
    #[serde(default)]
    pub ghost_reckoning: bool,
    #[serde(default)]
    pub hunter_mode: bool,
    #[serde(default)]
    pub typing_dna: bool,
    #[serde(default)]
    pub honey_tokens: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            cortex_plugins: false,
            passive_sigint: false,
            sentry_mode: false,
            mta: true,
            triangulate: false,
            the_mule: false,
            the_hydra: false,
            sonar: true,
            mirage: true,
            the_mole: true,
            spectrum_agility: true,
            ghost_reckoning: true,
            hunter_mode: true,
            typing_dna: false,
            honey_tokens: false,
        }
    }
}

static FEATURES: OnceLock<FeatureFlags> = OnceLock::new();

/// Load features from features.toml
pub fn load_features() -> Result<()> {
    let path = Path::new("features.toml");
    
    let flags = if path.exists() {
        let content = fs::read_to_string(path)?;
        let config: Features = toml::from_str(&content)?;
        config.features
    } else {
        // Return default if file doesn't exist (or warn)
        FeatureFlags::default()
    };
    
    // Set global features, ignore if already set
    let _ = FEATURES.set(flags);
    Ok(())
}

/// Check if a feature is enabled
pub fn is_enabled(feature: &str) -> bool {
    let flags = FEATURES.get_or_init(FeatureFlags::default);
    
    match feature {
        "cortex_plugins" => flags.cortex_plugins,
        "passive_sigint" => flags.passive_sigint,
        "sentry_mode" => flags.sentry_mode,
        "mta" => flags.mta,
        "triangulate" => flags.triangulate,
        "the_mule" => flags.the_mule,
        "the_hydra" => flags.the_hydra,
        "sonar" => flags.sonar,
        "mirage" => flags.mirage,
        "the_mole" => flags.the_mole,
        "spectrum_agility" => flags.spectrum_agility,
        "ghost_reckoning" => flags.ghost_reckoning,
        "hunter_mode" => flags.hunter_mode,
        "typing_dna" => flags.typing_dna,
        "honey_tokens" => flags.honey_tokens,
        _ => false,
    }
}

/// Get all flags
pub fn get_flags() -> &'static FeatureFlags {
    FEATURES.get_or_init(FeatureFlags::default)
}
