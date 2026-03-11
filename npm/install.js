#!/usr/bin/env node

"use strict";

const https = require("https");
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

const REPO = "0xdecaf/ynab-cli";
const BINARY_NAME = "ynab";

const PLATFORM_MAP = {
  "darwin-x64": "x86_64-apple-darwin",
  "darwin-arm64": "aarch64-apple-darwin",
  "linux-x64": "x86_64-unknown-linux-gnu",
  "linux-arm64": "aarch64-unknown-linux-gnu",
};

function getTarget() {
  const key = `${process.platform}-${process.arch}`;
  const target = PLATFORM_MAP[key];
  if (!target) {
    console.error(
      `Unsupported platform: ${process.platform} ${process.arch}\n` +
        `Supported: macOS (x64, arm64), Linux (x64, arm64)`
    );
    process.exit(1);
  }
  return target;
}

function getVersion() {
  const pkg = JSON.parse(
    fs.readFileSync(path.join(__dirname, "package.json"), "utf8")
  );
  return pkg.version;
}

function download(url) {
  return new Promise((resolve, reject) => {
    https
      .get(url, (res) => {
        // Follow redirects (GitHub releases redirect to S3)
        if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
          return download(res.headers.location).then(resolve, reject);
        }
        if (res.statusCode !== 200) {
          reject(new Error(`Download failed: HTTP ${res.statusCode} from ${url}`));
          return;
        }
        const chunks = [];
        res.on("data", (chunk) => chunks.push(chunk));
        res.on("end", () => resolve(Buffer.concat(chunks)));
        res.on("error", reject);
      })
      .on("error", reject);
  });
}

async function install() {
  const target = getTarget();
  const version = getVersion();
  const tarball = `ynab-cli-${target}.tar.gz`;
  const url = `https://github.com/${REPO}/releases/download/v${version}/${tarball}`;

  const binDir = path.join(__dirname, "bin");
  const binPath = path.join(binDir, BINARY_NAME);
  const tmpTarball = path.join(__dirname, tarball);

  console.log(`Downloading ynab v${version} for ${target}...`);

  try {
    const data = await download(url);

    // Write tarball to temp file
    fs.writeFileSync(tmpTarball, data);

    // Ensure bin directory exists
    fs.mkdirSync(binDir, { recursive: true });

    // Extract binary from tarball
    execSync(`tar xzf "${tmpTarball}" -C "${binDir}"`, { stdio: "pipe" });

    // Make binary executable
    fs.chmodSync(binPath, 0o755);

    // Clean up tarball
    fs.unlinkSync(tmpTarball);

    console.log(`Installed ynab to ${binPath}`);
  } catch (err) {
    console.error(`Failed to install ynab: ${err.message}`);
    console.error(
      `\nYou can manually download from:\n  https://github.com/${REPO}/releases/tag/v${version}`
    );
    process.exit(1);
  }
}

install();
