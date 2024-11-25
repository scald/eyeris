use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct ImagePrompt {
    pub text: String,
    pub format: PromptFormat,
    pub config: AnalysisConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptFormat {
    Concise,
    Detailed,
    Json,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentCategory {
    // Digital Content
    Screenshot { platform: Option<String> },
    UserInterface,
    SocialMediaPost,
    DigitalArt,
    Website,
    Software,
    VideoGame,
    
    // Documents
    Document,
    Receipt,
    BusinessCard,
    Invoice,
    Form,
    Identification,
    Certificate,
    
    // Visual Content
    Photo,
    Artwork,
    Illustration,
    Meme,
    Comic,
    Advertisement,
    Poster,
    
    // Instructional
    Recipe,
    Tutorial,
    Diagram,
    Blueprint,
    Schematic,
    Manual,
    Guide,
    
    // Data Visualization
    Chart,
    Graph,
    Dashboard,
    Infographic,
    Timeline,
    Flowchart,
    MindMap,
    
    // Location/Space
    Map,
    FloorPlan,
    Architecture,
    Landscape,
    Satellite,
    
    // Special Purpose
    Medical,
    Scientific,
    Technical,
    Educational,
    Legal,
    Financial,
    
    // Dynamic - for discovered categories
    Discovered {
        name: String,
        confidence: f32,
        traits: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub extract_text: bool,
    pub detect_faces: bool,
    pub identify_brands: bool,
    pub analyze_layout: bool,
    pub extract_data: bool,
    pub color_analysis: bool,
    pub spatial_analysis: bool,
    pub semantic_analysis: bool,
    pub detect_emotions: bool,
    pub identify_patterns: bool,
    pub historical_context: bool,
    pub cultural_analysis: bool,
    pub technical_details: bool,
    pub accessibility_analysis: bool,
    pub content_category: Option<ContentCategory>,
    pub custom_traits: Vec<String>,
}

impl Default for PromptFormat {
    fn default() -> Self {
        Self::Json // Default to JSON for structured output
    }
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            extract_text: true,
            detect_faces: true,
            identify_brands: true,
            analyze_layout: true,
            extract_data: true,
            color_analysis: true,
            spatial_analysis: true,
            semantic_analysis: true,
            detect_emotions: true,
            identify_patterns: true,
            historical_context: true,
            cultural_analysis: true,
            technical_details: true,
            accessibility_analysis: true,
            content_category: None,
            custom_traits: Vec::new(),
        }
    }
}

impl ImagePrompt {
    pub fn new(format: PromptFormat) -> Self {
        Self::with_config(format, AnalysisConfig::default())
    }

    pub fn with_config(format: PromptFormat, config: AnalysisConfig) -> Self {
        // Get base prompt text based on format
        let mut base_text = match format {
            PromptFormat::Concise => Self::get_concise_prompt(&config),
            PromptFormat::Detailed => Self::get_detailed_prompt(&config),
            PromptFormat::Json => Self::get_json_prompt(&config),
            PromptFormat::List => Self::get_list_prompt(&config),
        };

        // Add category-specific instructions if a category is specified
        if let Some(category) = &config.content_category {
            base_text.push_str("\n\n");
            base_text.push_str(&Self::get_category_specific_instructions(category));
        }

        // Create initial prompt instance
        let mut prompt = Self {
            text: base_text,
            format,
            config,
        };
        
        // Add dynamic discovery instructions
        let dynamic_text = prompt.add_dynamic_discovery_prompt();
        prompt.text.push_str("\n\n");
        prompt.text.push_str(&dynamic_text);
        
        prompt
    }
    fn get_concise_prompt(config: &AnalysisConfig) -> String {
        let mut prompt = "Analyze this image and describe its contents concisely.".to_string();
        
        if config.extract_text {
            prompt.push_str(" Extract any visible text.");
        }
        if config.detect_faces {
            prompt.push_str(" Note any faces present.");
        }
        if config.identify_brands {
            prompt.push_str(" Identify any brands or logos.");
        }

        prompt
    }

    fn get_detailed_prompt(config: &AnalysisConfig) -> String {
        let mut sections = vec![
            "Main subjects and their characteristics",
            "Background elements and setting",
            "Colors and lighting",
            "Notable details or unusual elements",
        ];

        if config.extract_text {
            sections.push("Any visible text or written content");
        }
        if config.detect_faces {
            sections.push("Presence and characteristics of any faces");
        }
        if config.identify_brands {
            sections.push("Visible brands, logos, or trademarked content");
        }
        if config.analyze_layout {
            sections.push("Layout and composition analysis");
        }

        format!(
            "Provide a detailed analysis of this image, including:\n{}",
            sections
                .iter()
                .map(|s| format!("- {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn get_list_prompt(config: &AnalysisConfig) -> String {
        let mut items = vec![
            "Main subject(s)",
            "Setting/location",
            "Notable actions/activities",
            "Key details",
        ];

        if config.extract_text {
            items.push("Visible text");
        }
        if config.detect_faces {
            items.push("Faces present");
        }
        if config.identify_brands {
            items.push("Brands and logos");
        }
        if config.analyze_layout {
            items.push("Layout structure");
        }

        format!(
            "Analyze this image and provide:\n{}",
            items
                .iter()
                .enumerate()
                .map(|(i, item)| format!("{}. {}", i + 1, item))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn get_json_prompt(_config: &AnalysisConfig) -> String {
        r#"You are an expert image analysis system with deep understanding across multiple domains. Analyze this image comprehensively and return a structured JSON response. While the structure below provides a framework, you are encouraged to:

1. Discover and add new relevant categories or traits not explicitly listed
2. Expand analysis based on patterns you recognize
3. Add domain-specific insights when relevant
4. Include any unique or unexpected observations
5. Note interesting relationships or implications

{
    "classification": {
        "primary_category": "string",
        "secondary_categories": ["string"],
        "confidence": 0.0-1.0,
        "discovered_categories": [{
            "name": "string",
            "confidence": 0.0-1.0,
            "reasoning": "string",
            "traits": ["string"]
        }]
    },

    "content": {
        "main_elements": [{
            "type": "string",
            "description": "string",
            "location": "string",
            "confidence": 0.0-1.0,
            "relationships": [{
                "related_to": "string",
                "relationship_type": "string",
                "significance": "string"
            }]
        }],
        "context": {
            "setting": "string",
            "time_period": "string",
            "cultural_context": "string",
            "purpose": "string"
        },
        "discovered_patterns": [{
            "pattern_type": "string",
            "description": "string",
            "implications": "string"
        }]
    },

    "analysis_layers": {
        "visual": {
            "composition": {
                "layout": "string",
                "style": "string",
                "techniques": ["string"],
                "quality": "string"
            },
            "colors": [{
                "name": "string",
                "hex": "string",
                "dominance": 0.0-1.0,
                "psychological_impact": "string"
            }],
            "discovered_visual_elements": [{
                "element": "string",
                "significance": "string"
            }]
        },
        
        "semantic": {
            "themes": ["string"],
            "symbolism": [{
                "symbol": "string",
                "meaning": "string",
                "cultural_relevance": "string"
            }],
            "emotional_tone": {
                "primary": "string",
                "secondary": ["string"],
                "confidence": 0.0-1.0
            }
        },
        
        "technical": {
            "creation_method": "string",
            "technical_quality": {
                "resolution": "string",
                "clarity": "string",
                "issues": ["string"]
            },
            "special_characteristics": [{
                "characteristic": "string",
                "significance": "string"
            }]
        }
    },

    "extracted_information": {
        "text_elements": [{
            "content": "string",
            "type": "string",
            "location": "string",
            "language": "string",
            "purpose": "string"
        }],
        "data_points": [{
            "type": "string",
            "value": "string",
            "context": "string"
        }],
        "structured_data": {
            "type": "string",
            "schema": "string",
            "data": {}
        }
    },

    "insights": {
        "key_observations": ["string"],
        "unusual_elements": ["string"],
        "potential_implications": ["string"],
        "suggested_actions": ["string"]
    },

    "metadata": {
        "analysis_confidence": 0.0-1.0,
        "quality_indicators": {
            "image_quality": "string",
            "analysis_completeness": 0.0-1.0,
            "ambiguous_elements": ["string"]
        },
        "processing_notes": ["string"]
    },

    "dynamic_extensions": {
        // This section is for any additional structured data you discover
        // Feel free to add any new categories or analysis types that seem relevant
    }
}"#.to_string()
    }

    fn get_category_specific_instructions(category: &ContentCategory) -> String {
        match category {
            ContentCategory::Screenshot { platform } => {
                let mut instructions = r#"For this screenshot, perform deep UI/UX analysis:
- Identify UI patterns and components
- Map interaction flows and user journeys
- Extract all text content and labels
- Analyze information hierarchy
- Note accessibility considerations
- Identify platform-specific patterns
- Map navigation structure
- Flag any security/privacy concerns"#.to_string();

                if let Some(platform_name) = platform {
                    instructions.push_str(&format!("\n\nSpecific to {platform_name}:
- Check platform-specific design guidelines
- Identify standard platform components
- Note any platform-specific conventions"));
                }

                instructions
            },
            // ... [rest of the category matches from your original code]
            _ => r#"Perform comprehensive analysis considering:
- Primary purpose
- Key elements and relationships
- Content organization
- Technical aspects
- Cultural context
- Practical applications
- Quality indicators
- Notable patterns
- Unique characteristics"#.to_string(),
        }
    }

    fn add_dynamic_discovery_prompt(&self) -> String {
        r#"
Beyond these specific instructions, please also:

1. Pattern Recognition
- Identify any recurring patterns or motifs
- Note unusual or unexpected elements
- Recognize domain-specific conventions

2. Contextual Analysis
- Consider historical or cultural significance
- Note technological implications
- Identify industry-specific elements

3. Relationship Mapping
- Map connections between elements
- Identify hierarchical structures
- Note cause-and-effect relationships

4. Innovation Detection
- Flag unique or innovative approaches
- Identify emerging patterns
- Note creative solutions

5. Quality Assessment
- Evaluate technical execution
- Assess practical effectiveness
- Consider user experience aspects

Feel free to create new categories or analysis dimensions if you discover something interesting that doesn't fit the standard framework. Explain your reasoning for significant discoveries."#.to_string()
    }

    pub fn to_openai_content(&self) -> serde_json::Value {
        serde_json::json!([
            {
                "type": "text",
                "text": self.text
            },
            {
                "type": "image_url",
                "image_url": {
                    "url": "data:image/jpeg;base64,{}" // Placeholder for base64 image
                }
            }
        ])
    }

    pub fn to_ollama_prompt(&self) -> String {
        self.text.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_formats() {
        let formats = vec![
            PromptFormat::Concise,
            PromptFormat::Detailed,
            PromptFormat::Json,
            PromptFormat::List,
        ];

        for format in formats {
            let prompt = ImagePrompt::new(format.clone());
            assert!(!prompt.text.is_empty());
            
            // Test with specific configuration
            let config = AnalysisConfig {
                extract_text: true,
                detect_faces: false,
                identify_brands: true,
                analyze_layout: true,
                extract_data: true,
                color_analysis: true,
                spatial_analysis: false,
                semantic_analysis: true,
                detect_emotions: true,
                identify_patterns: true,
                historical_context: true,
                cultural_analysis: true,
                technical_details: true,
                accessibility_analysis: true,
                content_category: Some(ContentCategory::Screenshot { platform: Some("iOS".to_string()) }),
                custom_traits: vec![],
            };
            
            let prompt_with_config = ImagePrompt::with_config(format, config);
            assert!(!prompt_with_config.text.is_empty());
            
            // Test OpenAI content generation
            let openai_content = prompt.to_openai_content();
            assert!(openai_content.is_array());
            
            // Test Ollama prompt generation
            let ollama_prompt = prompt.to_ollama_prompt();
            assert!(!ollama_prompt.is_empty());
        }
    }

    #[test]
    fn test_category_specific_prompts() {
        let categories = vec![
            ContentCategory::Screenshot { platform: Some("iOS".to_string()) },
            ContentCategory::Recipe,
            ContentCategory::Document,
            ContentCategory::Map,
        ];

        for category in categories {
            let config = AnalysisConfig {
                content_category: Some(category),
                ..Default::default()
            };
            
            let prompt = ImagePrompt::with_config(PromptFormat::Json, config);
            assert!(prompt.text.contains("For this"));
            assert!(!prompt.text.is_empty());
            
            // Verify dynamic discovery prompt is included
            assert!(prompt.text.contains("Pattern Recognition"));
            assert!(prompt.text.contains("Innovation Detection"));
        }
    }

    #[test]
    fn test_dynamic_discovery() {
        let config = AnalysisConfig {
            identify_patterns: true,
            semantic_analysis: true,
            cultural_analysis: true,
            ..Default::default()
        };
        
        let prompt = ImagePrompt::with_config(PromptFormat::Json, config);
        
        // Check for dynamic analysis elements
        assert!(prompt.text.contains("dynamic_extensions"));
        assert!(prompt.text.contains("discovered_categories"));
        assert!(prompt.text.contains("pattern_type"));
    }

    #[test]
    fn test_custom_traits() {
        let config = AnalysisConfig {
            custom_traits: vec![
                "brand_safety".to_string(),
                "viral_potential".to_string(),
            ],
            ..Default::default()
        };
        
        let prompt = ImagePrompt::with_config(PromptFormat::Detailed, config);
        assert!(prompt.text.contains("brand_safety") || prompt.text.contains("viral_potential"));
    }

    #[test]
    fn test_platform_specific_screenshot() {
        let config = AnalysisConfig {
            content_category: Some(ContentCategory::Screenshot {
                platform: Some("iOS".to_string())
            }),
            ..Default::default()
        };
        
        let prompt = ImagePrompt::with_config(PromptFormat::Detailed, config);
        assert!(prompt.text.contains("iOS"));
        assert!(prompt.text.contains("platform-specific"));
    }

    #[test]
    fn test_prompt_serialization() {
        let prompt = ImagePrompt::new(PromptFormat::Json);
        let serialized = serde_json::to_string(&prompt).unwrap();
        assert!(!serialized.is_empty());
        
        // Test OpenAI format
        let openai_content = prompt.to_openai_content();
        assert!(openai_content.is_array());
        assert_eq!(openai_content.as_array().unwrap().len(), 2);
        
        // Test Ollama format
        let ollama_prompt = prompt.to_ollama_prompt();
        assert!(!ollama_prompt.is_empty());
    }
}