/**
 * Requirement Validator Module
 * 
 * Modular validation for requirement expressivity.
 * Configure checks in CONFIG below, or extend by adding new methods.
 */

const CONFIG = {
  // Enable/disable checks
  checks: {
    acceptanceCriteria: true,
    boundaries: true,
    measurability: true,
    atomicity: true,
    testability: true
  },
  
  // Thresholds
  minAcceptanceCriteria: 1,
  maxRequirementLength: 2000
};

const requirementValidator = {
  /**
   * Main validation entry point
   * @param {string} requirement - The requirement text/body
   * @param {Object} options - Override default config
   * @returns {Object} validation result
   */
  validate(requirement, options = {}) {
    const config = { ...CONFIG, ...options };
    const checks = [];
    
    if (config.checks.acceptanceCriteria) {
      checks.push(this.checkAcceptanceCriteria(requirement));
    }
    
    if (config.checks.boundaries) {
      checks.push(this.checkBoundaries(requirement));
    }
    
    if (config.checks.measurability) {
      checks.push(this.checkMeasurability(requirement));
    }
    
    if (config.checks.atomicity) {
      checks.push(this.checkAtomicity(requirement));
    }
    
    if (config.checks.testability) {
      checks.push(this.checkTestability(requirement));
    }
    
    const failed = checks.filter(c => !c.passed);
    
    return {
      valid: failed.length === 0,
      score: checks.filter(c => c.passed).length / checks.length,
      checks: checks,
      feedback: failed.map(c => c.message),
      suggestions: this.generateSuggestions(failed)
    };
  },
  
  checkAcceptanceCriteria(req) {
    // Look for GWT pattern or checklist
    const patterns = [
      /given.*when.*then/i,
      /- \[.\]|\* \[.\]/, // Checkboxes
      /acceptance criteria:/i,
      /as a.*i want.*so that/i // User story format
    ];
    
    const found = patterns.some(p => p.test(req));
    
    return {
      name: 'Acceptance Criteria',
      passed: found,
      message: found 
        ? 'Has acceptance criteria'
        : 'Add acceptance criteria using Given-When-Then or checkbox format'
    };
  },
  
  checkBoundaries(req) {
    // Edge cases, limits, error conditions
    const patterns = [
      /edge case/i,
      /error|exception|failure/i,
      /null|undefined|empty/i,
      /maximum|minimum|limit/i,
      /invalid|unauthorized|forbidden/i
    ];
    
    const found = patterns.some(p => p.test(req));
    
    return {
      name: 'Boundaries',
      passed: found,
      message: found
        ? 'Defines boundaries and edge cases'
        : 'Describe error conditions and edge cases'
    };
  },
  
  checkMeasurability(req) {
    // Specific metrics or thresholds
    const patterns = [
      /\d+\s*(ms|s|mb|kb|%|users|rps)/i, // Units
      /within \d+/i,
      /at least \d+/i,
      /no more than \d+/i,
      /latency|throughput|availability/i
    ];
    
    const found = patterns.some(p => p.test(req));
    
    return {
      name: 'Measurability',
      passed: found,
      message: found
        ? 'Has measurable criteria'
        : 'Add specific metrics (latency < 100ms, throughput > 1000 rps, etc)'
    };
  },
  
  checkAtomicity(req) {
    // One requirement per issue
    const lines = req.split('\n').filter(l => l.trim());
    const tooLong = req.length > CONFIG.maxRequirementLength;
    const multiple = /and also|additionally|furthermore|, and /i.test(req);
    
    const passed = !tooLong && lines.length > 3 && !multiple;
    
    return {
      name: 'Atomicity',
      passed: passed,
      message: passed
        ? 'Requirement is atomic'
        : 'Split into smaller requirements (too long or contains multiple asks)'
    };
  },
  
  checkTestability(req) {
    // Can be verified
    const patterns = [
      /can be tested/i,
      /verify|validation/i,
      /demonstrate|show/i,
      /observable/i
    ];
    
    // Also check for vague terms
    const vague = /easy|fast|user-friendly|better|improved|enhanced/i;
    const hasVague = vague.test(req);
    
    return {
      name: 'Testability',
      passed: patterns.some(p => p.test(req)) || !hasVague,
      message: hasVague
        ? 'Replace vague terms (easy, fast, better) with specific criteria'
        : 'Requirement is testable'
    };
  },
  
  generateSuggestions(failed) {
    return failed.map(f => `- **${f.name}**: ${f.message}`).join('\n');
  },
  
  /**
   * Add custom validators
   */
  addValidator(name, validatorFn) {
    this[name] = validatorFn;
    CONFIG.checks[name] = true;
  }
};

module.exports = requirementValidator;