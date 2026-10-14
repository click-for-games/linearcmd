#!/usr/bin/env bun

const args = process.argv.slice(2);
const command = args[0];
const subArgs = args.slice(1);

// Helper to execute git commands seamlessly
async function runGit(cmd: string[]): Promise<boolean> {
  const proc = Bun.spawn(cmd, {
    stdout: "inherit",
    stderr: "inherit",
    stdin: "inherit",
  });
  const exitCode = await proc.exited;
  return exitCode === 0;
}

// Helper to check if current folder is a git repo
async function isGitRepo(): Promise<boolean> {
  const proc = Bun.spawn(["git", "rev-parse", "--is-inside-work-tree"], { stdout: "pipe", stderr: "pipe" });
  const exitCode = await proc.exited;
  return exitCode === 0;
}

// Helper to parse repo URLs (handles github.com/user/repo or git@github.com:...)
function formatGitUrl(rawUrl: string): string {
  let url = rawUrl.trim();
  if (url.startsWith("git--")) url = url.replace("git--", "").trim();
  if (!url.startsWith("http://") && !url.startsWith("https://") && !url.startsWith("git@")) {
    url = `https://${url}`;
  }
  if (!url.endsWith(".git")) {
    url = `${url}.git`;
  }
  return url;
}

async function main() {
  switch (command) {
    case "init": {
      console.log("⚡ Initializing Git repository...");
      if (!await runGit(["git", "init"])) process.exit(1);
      if (!await runGit(["git", "branch", "-M", "main"])) process.exit(1);

      if (subArgs[0]) {
        const remoteUrl = formatGitUrl(subArgs[0]);
        console.log(`🔗 Setting remote origin to: ${remoteUrl}`);
        await runGit(["git", "remote", "add", "origin", remoteUrl]);
      }
      console.log("✅ Repository initialized!");
      break;
    }

    case "push": {
      console.log("🚀 Running Linear Push workflow...");

      // Automatically initialize git if not in a git repo
      if (!await isGitRepo()) {
        console.log("⚠️ Not a git repository. Initializing now...");
        if (!await runGit(["git", "init"])) process.exit(1);
        if (!await runGit(["git", "branch", "-M", "main"])) process.exit(1);
      }

      // If a URL or git-- option is passed as argument
      let remoteUrl = subArgs.find(a => a.includes("github.com") || a.startsWith("git--"));
      let commitMsg = subArgs.filter(a => a !== remoteUrl).join(" ");

      if (remoteUrl) {
        const cleanUrl = formatGitUrl(remoteUrl);
        console.log(`🔗 Adding/updating remote origin: ${cleanUrl}`);
        await runGit(["git", "remote", "remove", "origin"]).catch(() => {});
        await runGit(["git", "remote", "add", "origin", cleanUrl]);
      }

      console.log("📦 Staging all changes...");
      if (!await runGit(["git", "add", "."])) process.exit(1);

      const msg = commitMsg || `update: ${new Date().toLocaleString()}`;
      console.log(`💬 Committing with message: "${msg}"`);
      await runGit(["git", "commit", "-m", msg]);

      console.log("🌐 Pushing to origin...");
      const pushed = await runGit(["git", "push", "-u", "origin", "HEAD"]);
      if (pushed) console.log("🎉 Successfully pushed!");
      break;
    }

    case "fix": {
      if (!await isGitRepo()) {
        console.error("❌ Error: Current directory is not a Git repository.");
        process.exit(1);
      }

      console.log("🛠️ Executing fast hotfix commit...");
      await runGit(["git", "add", "-u"]);
      const fixMsg = subArgs.join(" ") || "fix: resolved issue";
      await runGit(["git", "commit", "-m", fixMsg]);
      await runGit(["git", "push"]);
      console.log("⚡ Fix deployed!");
      break;
    }

    case "sync": {
      if (!await isGitRepo()) {
        console.error("❌ Error: Not a Git repository.");
        process.exit(1);
      }
      console.log("🔄 Syncing with remote (Pull --rebase & Push)...");
      await runGit(["git", "pull", "--rebase"]);
      await runGit(["git", "push"]);
      console.log("✅ In sync!");
      break;
    }

    case "branch": {
      if (!await isGitRepo()) {
        console.error("❌ Not a Git repo.");
        process.exit(1);
      }
      const branchName = subArgs[0];
      if (!branchName) {
        console.log("📌 Active branches:");
        await runGit(["git", "branch", "-a"]);
      } else {
        console.log(`🔀 Switching to branch: ${branchName}`);
        await runGit(["git", "checkout", "-b", branchName]).catch(() => runGit(["git", "checkout", branchName]));
      }
      break;
    }

    case "status":
    case "st": {
      if (!await isGitRepo()) {
        console.error("❌ Not a Git repo.");
        process.exit(1);
      }
      await runGit(["git", "status"]);
      break;
    }

    case "undo": {
      if (!await isGitRepo()) process.exit(1);
      console.log("⏪ Undoing last unpushed commit (soft reset)...");
      await runGit(["git", "reset", "--soft", "HEAD~1"]);
      console.log("✅ Last commit undone. Your changes are still staged.");
      break;
    }

    default:
      console.log(`
  Linear Command Line Interface ⚡

  Usage:
    linear init [remote-url]        - Initialize a repo & set remote
    linear push [msg] [remote-url]  - Stage, commit, auto-link remote & push
    linear fix [msg]                - Quick stage-tracked & push hotfix
    linear sync                     - Pull rebase & push upstream
    linear branch [name]            - List or switch/create branches
    linear status                   - Clean status overview
    linear undo                     - Undo last commit keeping files staged
      `);
      break;
  }
}

main();