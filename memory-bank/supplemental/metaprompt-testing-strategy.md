# Metaprompt Testing Strategy with GUI Harness + VCR

## Overview

The existing GUI test harness in `tests/gui/test_harness.rs` provides sophisticated testing infrastructure that can be adapted for the metaprompt architecture. By combining it with VCR-style API recording (e.g., `reqwest-vcr`), we can create deterministic, cost-effective tests for the entire generation flow.

## Existing Test Harness Capabilities

### GuiTestHarness Features
- **Process Management**: Launches app with custom database path
- **Screenshot Capture**: Timestamped screenshots for visual regression
- **Input Simulation**: Click positions, type text, key presses via `enigo`
- **State Verification**: Check if app is still running
- **Timeout Protection**: Configurable timeouts for long operations
- **Cleanup**: Automatic process termination and temp directory cleanup

### Current Test Coverage
- App launch without crash
- Wizard initial screen rendering
- Navigation between screens
- Database creation verification
- Crash debugging utilities

## Proposed Testing Architecture

### 1. VCR Integration for OpenAI Calls

#### Using `vcr-cassette` or `reqwest-vcr`
```toml
[dev-dependencies]
vcr-cassette = "0.1"
# or
reqwest-vcr = "0.3"
```

#### Modified OpenAI Client
```rust
// In openai_client.rs
#[cfg(test)]
pub fn create_test_client() -> OpenAIClient {
    use vcr_cassette::{VCR, Mode};
    
    let vcr = VCR::new("tests/cassettes/openai")
        .with_mode(Mode::Replay) // Use Mode::Record for initial recording
        .build();
    
    OpenAIClient::new_with_middleware(vcr)
}
```

### 2. Test Scenarios for Metaprompt Flow

#### Test 1: Complete Generation Flow
```rust
#[test]
#[serial]
fn test_complete_generation_flow_with_vcr() {
    // Set up VCR cassette
    let cassette_name = "complete_generation_flow";
    
    let runner = TestRunner::new("generation_flow")
        .with_timeout(Duration::from_secs(120)) // Longer for generation
        .expect("Failed to create test runner");
    
    runner.run(|harness| {
        // Launch app
        harness.launch()?;
        harness.wait(Duration::from_secs(3));
        
        // Navigate through wizard
        harness.screenshot("wizard_start")?;
        
        // Fill in game name
        harness.click(500, 300)?; // Click name field
        harness.type_text("Test RPG Game")?;
        harness.screenshot("name_entered")?;
        
        // Select genre
        harness.click(500, 400)?; // Fantasy option
        harness.screenshot("genre_selected")?;
        
        // Complete wizard
        harness.click(1100, 750)?; // Next button
        harness.wait(Duration::from_secs(1));
        
        // Start generation (this will use recorded API responses)
        harness.click(600, 500)?; // Generate button
        harness.screenshot("generation_started")?;
        
        // Monitor progress
        for i in 0..10 {
            harness.wait(Duration::from_secs(2));
            harness.screenshot(&format!("progress_{}", i))?;
            
            // Check if generation complete
            // (would need visual recognition or specific UI markers)
        }
        
        // Verify assets created
        let db_path = harness.db_path();
        verify_assets_in_database(db_path)?;
        
        Ok(())
    })
}
```

#### Test 2: Progress Event Display
```rust
#[test]
#[serial]
fn test_generation_progress_events() {
    // Test that progress events update the UI correctly
    // Uses pre-recorded API responses for deterministic progress
}
```

#### Test 3: Error Handling
```rust
#[test]
#[serial]
fn test_api_error_handling() {
    // Use VCR cassette with error responses
    // Verify UI shows appropriate error messages
}
```

### 3. Recording Strategy

#### Initial Recording Setup
```bash
# Environment variable to enable recording mode
OPENAI_TEST_MODE=record cargo test test_complete_generation_flow

# This will:
# 1. Make real API calls
# 2. Record requests/responses to cassettes
# 3. Cost money but only once
```

#### Cassette Structure
```
tests/cassettes/
├── openai/
│   ├── complete_generation_flow/
│   │   ├── phase1_style_guide.json
│   │   ├── phase2_architecture.json
│   │   ├── phase3_assets.json
│   │   ├── phase4_implementation.json
│   │   └── phase5_polish.json
│   ├── error_scenarios/
│   │   ├── rate_limit.json
│   │   ├── invalid_api_key.json
│   │   └── timeout.json
│   └── partial_generation/
│       └── cancelled_after_phase2.json
```

### 4. Visual Regression Testing

#### Screenshot Comparison
```rust
use image_compare::compare_images;

fn verify_screenshot_matches(
    actual: &Path,
    expected: &Path,
    threshold: f32,
) -> Result<()> {
    let result = compare_images(actual, expected)?;
    assert!(
        result.similarity > threshold,
        "Screenshot similarity {} below threshold {}",
        result.similarity,
        threshold
    );
    Ok(())
}
```

#### Expected Screenshots
```
tests/expected_screenshots/
├── wizard_start.png
├── generation_progress_25.png
├── generation_progress_50.png
├── generation_complete.png
└── asset_gallery_populated.png
```

### 5. Database Verification

```rust
async fn verify_assets_in_database(db_path: &Path) -> Result<()> {
    let db = Database::connect(&format!("sqlite://{}", db_path.display())).await?;
    
    // Check project exists
    let projects = project::Entity::find().all(&db).await?;
    assert_eq!(projects.len(), 1);
    
    // Check assets were generated
    let assets = asset::Entity::find()
        .filter(asset::Column::ProjectId.eq(projects[0].id))
        .all(&db)
        .await?;
    
    // Should have at least: style guide, sprites, code files
    assert!(assets.len() >= 10, "Expected at least 10 assets, found {}", assets.len());
    
    // Verify asset types
    let asset_types: HashSet<_> = assets.iter()
        .map(|a| &a.asset_type)
        .collect();
    
    assert!(asset_types.contains("style_guide"));
    assert!(asset_types.contains("sprite"));
    assert!(asset_types.contains("code"));
    
    Ok(())
}
```

### 6. Test Data Management

#### Fixture System
```rust
pub struct TestFixtures {
    pub simple_game_config: GameConfiguration,
    pub complex_game_config: GameConfiguration,
    pub expected_style_guide: StyleGuide,
}

impl TestFixtures {
    pub fn load() -> Self {
        // Load from JSON files
        Self {
            simple_game_config: serde_json::from_str(
                include_str!("fixtures/simple_game.json")
            ).unwrap(),
            // ...
        }
    }
}
```

### 7. CI/CD Integration

```yaml
# .github/workflows/test.yml
test-metaprompt:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v3
    
    - name: Cache VCR Cassettes
      uses: actions/cache@v3
      with:
        path: tests/cassettes
        key: vcr-cassettes-${{ hashFiles('tests/cassettes/**') }}
    
    - name: Run Tests
      run: |
        # Run in replay mode (no API calls)
        OPENAI_TEST_MODE=replay cargo test --test gui_tests
```

## Implementation Steps

1. **Add VCR Dependency**
   ```toml
   [dev-dependencies]
   vcr-cassette = "0.1"
   serial_test = "2.0"
   image-compare = "0.3"
   ```

2. **Modify OpenAI Client**
   - Add test-specific client creation
   - Support VCR middleware injection
   - Environment variable for record/replay mode

3. **Create Test Fixtures**
   - Sample game configurations
   - Expected outputs for comparison
   - Error scenarios

4. **Record Initial Cassettes**
   - Run tests in record mode
   - Verify recordings are complete
   - Commit cassettes to repo

5. **Implement Test Cases**
   - Complete flow test
   - Progress monitoring test
   - Error handling test
   - Cancellation test

6. **Add Visual Regression**
   - Capture expected screenshots
   - Implement comparison logic
   - Set appropriate thresholds

## Benefits

1. **Deterministic Tests**: Same results every time
2. **No API Costs**: Replay recorded responses
3. **Fast Execution**: No network latency
4. **Comprehensive Coverage**: Test entire flow including UI
5. **Visual Verification**: Screenshots catch UI regressions
6. **Real Integration**: Tests actual user workflows

## Challenges & Solutions

### Challenge: Non-deterministic UI Animations
**Solution**: Wait for animations to complete before screenshots

### Challenge: Timing-dependent Progress Events  
**Solution**: Mock time in tests or use fixed intervals

### Challenge: Large Cassette Files
**Solution**: Git LFS for cassette storage

### Challenge: API Response Changes
**Solution**: Versioned cassettes, periodic re-recording

## Example Test Output

```
Running GUI test: test_complete_generation_flow_with_vcr
Launching application with database: /tmp/.tmpXYZ/test.db
Screenshot saved: tests/screenshots/generation_flow/wizard_start.png
Clicking at (500, 300)
Typing: Test RPG Game
Screenshot saved: tests/screenshots/generation_flow/name_entered.png
[... more output ...]
✓ Project created in database
✓ 15 assets generated
✓ Style guide consistent across assets
Test completed successfully in 45.2s
```

## Conclusion

By combining the existing GUI test harness with VCR-style API recording, we can create a robust testing strategy that:
- Validates the entire user journey
- Ensures UI responsiveness during generation
- Verifies correct data persistence
- Catches visual regressions
- Runs quickly and deterministically in CI

This approach leverages existing infrastructure while adapting it perfectly for the new metaprompt architecture.
