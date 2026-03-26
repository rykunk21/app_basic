const express = require('express');
const crypto = require('crypto');
const { spawn } = require('child_process');

const app = express();
const PORT = process.env.PORT || 3000;
const WEBHOOK_SECRET = process.env.WEBHOOK_SECRET;
const OPENCLAW_GATEWAY_URL = process.env.OPENCLAW_GATEWAY_URL;
const OPENCLAW_TOKEN = process.env.OPENCLAW_TOKEN;

app.use(express.raw({ type: 'application/json' }));

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: 'axiom-webhook-server' });
});

// Verify GitHub webhook signature
function verifySignature(signature, body) {
  if (!WEBHOOK_SECRET) return true; // Skip in dev
  const hmac = crypto.createHmac('sha256', WEBHOOK_SECRET);
  const digest = 'sha256=' + hmac.update(body).digest('hex');
  return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(digest));
}

// Forward event to OpenClaw agent
async function notifyAgent(eventType, payload) {
  try {
    const response = await fetch(`${OPENCLAW_GATEWAY_URL}/cron/wake`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${OPENCLAW_TOKEN}`
      },
      body: JSON.stringify({
        text: `[GitHub Webhook] ${eventType}\n\n${JSON.stringify(payload, null, 2)}`,
        mode: 'now'
      })
    });
    
    if (!response.ok) {
      console.error('Failed to notify agent:', response.statusText);
      return false;
    }
    return true;
  } catch (error) {
    console.error('Error notifying agent:', error);
    return false;
  }
}

// Webhook handler
app.post('/webhook/github', async (req, res) => {
  const signature = req.headers['x-hub-signature-256'];
  const event = req.headers['x-github-event'];
  const delivery = req.headers['x-github-delivery'];
  
  if (!verifySignature(signature, req.body)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }
  
  const payload = JSON.parse(req.body);
  
  console.log(`Received ${event} (delivery: ${delivery})`);
  
  // Process specific events
  const handlers = {
    'issues': handleIssueEvent,
    'issue_comment': handleIssueCommentEvent,
    'pull_request': handlePullRequestEvent,
    'push': handlePushEvent,
    'workflow_run': handleWorkflowRunEvent
  };
  
  if (handlers[event]) {
    await handlers[event](payload);
  }
  
  res.status(200).json({ received: true });
});

// Event handlers
async function handleIssueEvent(payload) {
  const { action, issue } = payload;
  
  // Trigger agent when:
  // - Issue opened (new backlog item)
  // - Issue labeled (column transition)
  // - Issue closed
  
  if (['opened', 'labeled', 'closed', 'reopened'].includes(action)) {
    await notifyAgent('issues', {
      action,
      issue_number: issue.number,
      title: issue.title,
      labels: issue.labels.map(l => l.name),
      state: issue.state
    });
  }
}

async function handlePullRequestEvent(payload) {
  const { action, pull_request, repository } = payload;
  
  // Trigger on PR open, sync, merge, close
  if (['opened', 'synchronize', 'closed', 'merged'].includes(action)) {
    await notifyAgent('pull_request', {
      action,
      pr_number: pull_request.number,
      title: pull_request.title,
      state: pull_request.state,
      merged: pull_request.merged,
      head_branch: pull_request.head.ref,
      base_branch: pull_request.base.ref
    });
  }
}

async function handlePushEvent(payload) {
  const { ref, commits, repository } = payload;
  
  // Only care about main/master pushes
  if (ref === 'refs/heads/main' || ref === 'refs/heads/master') {
    await notifyAgent('push', {
      ref,
      commits: commits.map(c => ({
        message: c.message,
        id: c.id
      })),
      repository: repository.full_name
    });
  }
}

async function handleWorkflowRunEvent(payload) {
  const { action, workflow_run } = payload;
  
  // CI completion status
  if (action === 'completed') {
    await notifyAgent('workflow_run', {
      name: workflow_run.name,
      conclusion: workflow_run.conclusion,
      head_branch: workflow_run.head_branch,
      head_sha: workflow_run.head_sha
    });
  }
}

async function handleIssueCommentEvent(payload) {
  // Handle @axiom mentions or commands in comments
  const { comment, issue } = payload;
  
  if (comment.body.includes('@axiom') || comment.body.includes('@openclaw')) {
    await notifyAgent('issue_comment', {
      issue_number: issue.number,
      comment_body: comment.body,
      author: comment.user.login
    });
  }
}

app.listen(PORT, () => {
  console.log(`Axiom webhook server listening on port ${PORT}`);
  console.log(`Webhook URL: https://your-domain.com/webhook/github`);
});