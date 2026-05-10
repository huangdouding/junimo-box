import { execFileSync } from "node:child_process";
import { platform } from "node:process";

const port = Number(process.argv[2] || "1420");

function findListenerPids(targetPort) {
  try {
    const output = execFileSync("netstat", ["-ano", "-p", "tcp"], {
      encoding: "utf8",
      windowsHide: true,
    });

    const pids = new Set();
    for (const line of output.split(/\r?\n/)) {
      if (!line.includes(`:${targetPort}`) || !line.includes("LISTENING")) {
        continue;
      }

      const match = line.trim().match(/\s(\d+)$/);
      if (match) {
        pids.add(match[1]);
      }
    }

    return [...pids];
  } catch {
    return [];
  }
}

function killPid(pid) {
  try {
    execFileSync("taskkill", ["/PID", pid, "/F"], {
      encoding: "utf8",
      windowsHide: true,
      stdio: "ignore",
    });
    console.log(`Stopped process ${pid} on port ${port}.`);
  } catch (error) {
    console.log(`Skipped process ${pid}: ${error instanceof Error ? error.message : String(error)}`);
  }
}

if (platform !== "win32") {
  console.log(`Port cleanup helper is intended for Windows. Skipping port ${port}.`);
  process.exit(0);
}

const pids = findListenerPids(port);

if (pids.length === 0) {
  console.log(`Port ${port} is free.`);
  process.exit(0);
}

for (const pid of pids) {
  killPid(pid);
}
