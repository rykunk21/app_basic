#!/usr/bin/env node
/**
 * Axiom Agent Daemon
 * 
 * This daemon runs 24/7 and manages the continuous Kanban workflow.
 * It responds to webhooks and polls for work when idle.
 */

const fs = require('fs');
const path = require('path');

const CONFIG = {
  repo: process.env.REPO || 'rykunk21/app_basic',
  projectNumber: process.env.PROJECT_NUMBER || '5',
  pollInterval: parseInt(process.env.POLL_INTERVAL) || 300000, // 5 min
  stateFile: '/tmp/axiom-agent-state.json'
};

const STATE = {
  currentIssue: null,
  phase: 'idle', // idle | planning | red | green | refactor | pr
  testSnapshot: null,
  lastWebhook: null
};

// Load previous state
function loadState() {
  try {
    if (fs.existsSync(CONFIG.stateFile)) {
      Object.assign(STATE, JSON.parse(fs.readFileSync(CONFIG.stateFile)));
    }
  } catch (e) {
    console.error('Failed to load state:', e);
  }
}

function saveState() {
  try {
    fs.writeFileSync(CONFIG.stateFile, JSON.stringify(STATE, null, 2));
  } catch (e) {
    console.error('Failed to save state:', e);
  }
}

// GitHub CLI wrapper
async function gh(args) {
  const { spawn } = require('child_process');
  return new Promise((resolve, reject) => {
    const proc = spawn('gh', args, { stdio: ['ignore', 'pipe', 'pipe'] });
    let stdout = '', stderr = '';
    proc.stdout.on('data', d => stdout += d);
    proc.stderr.on('data', d => stderr += d);
    proc.on('close', code => {
      if (code === 0) resolve(stdout);
      else reject(new Error(stderr || stdout));
    });
  });
}

// Requirement Validator (modular — enhance later)
const requirementValidator = {
  check(expressivity, requirement) {
    const checks = [
      this.hasAcceptanceCriteria(requirement),
      this.hasBoundaries(requirement),
      this.isMeasurable(requirement)
    ];
    
    return {
      valid: checks.every(c => c.passed),
      checks: checks,
      suggestions: checks.filter(c => !c.passed).map(c => c.message)
    };
  },
  
  hasAcceptanceCriteria(req) {
    // Check for Given-When-Then or bullet points
    const hasGWT = /given.*when.*then/i.test(req) || 
                   /- \[ \]|\* /.test(req);
    return { 
      passed: hasGWT, 
      message: hasGWT ? 'Has acceptance criteria' : 'Missing acceptance criteria (Given-When-Then format)'
    };
  },
  
  hasBoundaries(req) {
    // Check for edge cases or error conditions
    const hasBoundaries = /edge case|error|exception|invalid|null/i.test(req);
    return { 
      passed: hasBoundaries, 
      message: hasBoundaries ? 'Defines boundaries' : 'Missing edge cases or error conditions'
    };
  },
  
  isMeasurable(req) {
    // Check for specific metrics or completion criteria
    const isMeasurable = /\d+|minimum|maximum|within|less than|greater than/i.test(req);
    return { 
      passed: isMeasurable, 
      message: isMeasurable ? 'Has measurable criteria' : 'Add specific metrics or thresholds'
    };
  }
};

// Test Fingerprinting
const testFingerprint = {
  async capture(repo) {
    try {
      // Hash all test files
      const { execSync } = require('child_process');
      const tests = execSync('find . -name "*.rs" -path "*/test*" -o -name "*test*.rs" 2>/dev/null || true', { cwd: `/tmp/${repo}` });
      const hash = crypto.createHash('sha256').update(tests).digest('hex');
      return hash;
    } catch {
      return null;
    }
  },
  
  async verify(repo, originalHash) {
    const currentHash = await this.capture(repo);
    return currentHash === originalHash;
  }
};

// Phase handlers
const phases = {
  async idle() {
    console.log('[Idle] Checking for backlog work...');
    
    try {
      // Find issues in backlog
      const output = await gh([
        'issue', 'list', '--repo', CONFIG.repo,
        '--label', 'backlog,story',
        '--limit', '1',
        '--json', 'number,title,body'
      ]);
      
      const issues = JSON.parse(output);
      if (issues.length > 0) {
        STATE.currentIssue = issues[0].number;
        STATE.phase = 'planning';
        console.log(`[Idle] Found issue #${STATE.currentIssue}: ${issues[0].title}`);
      }
    } catch (e) {
      console.log('[Idle] No backlog stories found');
    }
  },
  
  async planning() {
    console.log(`[Planning] Issue #${STATE.currentIssue}: Validating requirement...`);
    
    // Fetch issue details
    const output = await gh([
      'issue', 'view', STATE.currentIssue.toString(),
      '--repo', CONFIG.repo,
      '--json', 'title,body'
    ]);
    
    const issue = JSON.parse(output);
    
    // Validate requirement
    const validation = requirementValidator.check('simple', issue.body || '');
    
    if (!validation.valid) {
      console.log('[Planning] Requirement needs refinement:', validation.suggestions);
      // Add comment to issue
      await gh([
        'issue', 'comment', STATE.currentIssue.toString(),
        '--repo', CONFIG.repo,
        '--body', `⚠️ Requirement needs refinement:\n\n${validation.suggestions.join('\n- ')}`
      ]);
      STATE.currentIssue = null;
      STATE.phase = 'idle';
      return;
    }
    
    // Auto-promote to Red
    await gh([
      'issue', 'edit', STATE.currentIssue.toString(),
      '--repo', CONFIG.repo,
      '--add-label', 'red',
      '--remove-label', 'backlog'
    ]);
    
    console.log(`[Planning] Moved issue #${STATE.currentIssue} to Red`);
    STATE.phase = 'red';
  },
  
  async red() {
    console.log(`[Red] Issue #${STATE.currentIssue}: Writing failing test...`);
    // TODO: Generate test from requirement
    // For now, wait for developer
    STATE.phase = 'green';
  },
  
  async green() {
    console.log(`[Green] Issue #${STATE.currentIssue}: Implementing...`);
    
    // Snapshot tests
    STATE.testSnapshot = await testFingerprint.capture(CONFIG.repo);
    
    // TODO: Generate implementation
    // For now, wait for developer
    STATE.phase = 'refactor';
  },
  
  async refactor() {
    console.log(`[Refactor] Issue #${STATE.currentIssue}: Verifying...`);
    
    // Verify tests unchanged
    const testsValid = await testFingerprint.verify(CONFIG.repo, STATE.testSnapshot);
    
    if (!testsValid) {
      console.log('[Refactor] WARNING: Tests changed during green phase!');
      // Revert to red
      await gh([
        'issue', 'edit', STATE.currentIssue.toString(),
        '--repo', CONFIG.repo,
        '--add-label', 'red',
        '--remove-label', 'green,refactor'
      ]);
      STATE.phase = 'red';
      return;
    }
    
    // Open PR
    console.log(`[Refactor] Opening PR for issue #${STATE.currentIssue}`);
    // TODO: Open PR via gh pr create
    STATE.phase = 'pr';
  },
  
  async pr() {
    console.log(`[PR] Issue #${STATE.currentIssue}: Waiting for merge...`);
    // Webhook handler will advance this when PR merges
    // For now, poll PR status
    try {
      const output = await gh([
        'pr', 'list', '--repo', CONFIG.repo,
        '--state', 'merged',
        '--search', `closes #${STATE.currentIssue}`,
        '--limit', '1',
        '--json', 'number'
      ]);
      const prs = JSON.parse(output);
      if (prs.length > 0) {
        // PR merged! Move to done
        await gh([
          'issue', 'edit', STATE.currentIssue.toString(),
          '--repo', CONFIG.repo,
          '--add-label', 'done',
          '--remove-label', 'refactor'
        ]);
        await gh([
          'issue', 'close', STATE.currentIssue.toString(),
          '--repo', CONFIG.repo
        ]);
        
        console.log(`[PR] Issue #${STATE.currentIssue} completed!`);
        STATE.currentIssue = null;
        STATE.phase = 'idle';
      }
    } catch (e) {
      // PR not merged yet
    }
  }
};

// Webhook processor
function processWebhook(event, payload) {
  console.log(`[Webhook] Received ${event}`);
  STATE.lastWebhook = { event, payload, time: Date.now() };
  
  switch (event) {
    case 'issues':
      if (payload.action === 'opened' && payload.labels.includes('backlog')) {
        console.log('[Webhook] New backlog item - will process on next cycle');
      }
      break;
      
    case 'pull_request':
      if (payload.action === 'merged') {
        console.log('[Webhook] PR merged - checking if our issue');
      }
      break;
      
    case 'workflow_run':
      if (payload.conclusion === 'success') {
        console.log('[Webhook] CI passed - can advance to next phase');
      }
      break;
  }
  
  saveState();
}

// Main loop
async function tick() {
  console.log(`\n[${new Date().toISOString()}] Tick: ${STATE.phase}`);
  
  if (phases[STATE.phase]) {
    await phases[STATE.phase]();
  }
  
  saveState();
}

// HTTP server for webhook reception
const http = require('http');

const server = http.createServer((req, res) => {
  if (req.method === 'POST' && req.url === '/webhook') {
    let body = '';
    req.on('data', chunk => body += chunk);
    req.on('end', () => {
      try {
        const payload = JSON.parse(body);
        const event = req.headers['x-github-event'] || payload.event_type;
        processWebhook(event, payload);
        res.writeHead(200); res.end('OK');
      } catch (e) {
        res.writeHead(400); res.end('Invalid JSON');
      }
    });
  } else if (req.method === 'GET' && req.url === '/health') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ 
      status: 'ok', 
      phase: STATE.phase,
      currentIssue: STATE.currentIssue
    }));
  } else {
    res.writeHead(404); res.end('Not found');
  }
});

// Start up
console.log('=== Axiom Agent Daemon ===');
console.log(`Repository: ${CONFIG.repo}`);
console.log(`Poll interval: ${CONFIG.pollInterval}ms`);

loadState();

// Start HTTP server for webhooks
server.listen(8080, () => {
  console.log('Webhook listener on :8080');
});

// Start work loop
setInterval(tick, CONFIG.pollInterval);
tick(); // Initial tick