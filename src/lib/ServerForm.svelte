<script>
  import { startServer } from "./api.js";
  import { onMount } from "svelte";
  import { listRunners } from "./api.js";

  let runners = [];
  let runnerId = "";

  let serverId = "server-333";
  let gameId = "22";
  let gameName = "tml";

  let runnerImage = "runner-tml:latest";
  let gameSavesMountPath = "/tModLoader";
  let supportsMods = true;
  let modsMountPath = "";
  let forceSaveCmd = "save";
  let preinstallerType = "PREINSTALLER_TYPE_IMAGE";
  let preinstallerImage = "preinst-tml:latest";
  let downloaderType = "DOWNLOADER_TYPE_NONE";

  let startScript = "";
  let isNewWorld = true;
  let modSetId = "";
  let modSetPath = "";

  // одна переменная на строку, вида KEY=VALUE
  let envText = "";
  let preinstallerEnvText = [
    "WORLD_NAME=pivo1",
    "WORLD_SIZE=2",
    "WORLD_SEED=MYSEED",
    "DIFFICULTY=1",
    "MOD_IDS=",
    "PORT=7777",
    "PASSWORD=",
    "MAX_PLAYERS=10",
    "MOTD=Welcome!",
  ].join("\n");

  let submitting = false;
  let error = "";
  let lastResult = null;

  onMount(async () => {
    try {
      runners = await listRunners();
      if (runners.length > 0) runnerId = runners[0].runnerId;
    } catch (e) {
      error = String(e);
    }
  });

  function linesToArray(text) {
    return text
      .split("\n")
      .map((l) => l.trim())
      .filter((l) => l.length > 0);
  }

  async function handleSubmit() {
    submitting = true;
    error = "";
    lastResult = null;
    try {
      const req = {
        serverId: { serverId },
        game: {
          gameId,
          gameName,
          spec: {
            downloaderType,
            runnerImage,
            gameSavesMountPath,
            supportsMods,
            modsMountPath,
            forceSaveCmd,
            preinstallerType,
            preinstallerImage,
          },
        },
        startScript,
        env: linesToArray(envText),
        modSetId,
        modSetPath,
        isNewWorld,
        s3PresignedGetUrl: "",
        s3PresignedPutUrl: "",
        preinstaller_env: linesToArray(preinstallerEnvText),
      };
      lastResult = await startServer(runnerId, req);
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }
</script>

<div class="panel" style="padding: 16px; max-width: 480px;">
  <h2 style="font-size:14px; font-weight:500; margin:0 0 12px;">Запустить сервер</h2>

  <form on:submit|preventDefault={handleSubmit} style="display:flex; flex-direction:column; gap:10px;">
    <label class="field">
      <span>раннер</span>
      <select class="mono" bind:value={runnerId}>
        {#each runners as r}
          <option value={r.runnerId}>{r.nodeName || r.runnerId.slice(0, 8)} ({r.ip})</option>
        {/each}
      </select>
    </label>

    <label class="field">
      <span>server id</span>
      <input class="mono" type="text" bind:value={serverId} required />
    </label>

    <div style="display:flex; gap:10px;">
      <label class="field" style="flex:1;">
        <span>game id</span>
        <input class="mono" type="text" bind:value={gameId} />
      </label>
      <label class="field" style="flex:1;">
        <span>game name</span>
        <input class="mono" type="text" bind:value={gameName} />
      </label>
    </div>

    <label class="field">
      <span>runner image</span>
      <input class="mono" type="text" bind:value={runnerImage} />
    </label>

    <div style="display:flex; gap:10px;">
      <label class="field" style="flex:1;">
        <span>preinstaller image</span>
        <input class="mono" type="text" bind:value={preinstallerImage} />
      </label>
      <label class="field" style="flex:1;">
        <span>saves mount path</span>
        <input class="mono" type="text" bind:value={gameSavesMountPath} />
      </label>
    </div>

    <label class="field" style="flex-direction:row; align-items:center; gap:6px;">
      <input type="checkbox" bind:checked={isNewWorld} />
      <span>новый мир (isNewWorld)</span>
    </label>

    <label class="field">
      <span>preinstaller env (KEY=VALUE, по строке)</span>
      <textarea class="mono" rows="6" bind:value={preinstallerEnvText}></textarea>
    </label>

    <label class="field">
      <span>env (KEY=VALUE, по строке)</span>
      <textarea class="mono" rows="3" bind:value={envText}></textarea>
    </label>

    <button type="submit" class="primary-btn" disabled={submitting || !runnerId}>
      {submitting ? "запускается..." : "запустить сервер"}
    </button>
  </form>

  {#if error}
    <p style="color: var(--danger); margin-top:10px;" class="mono">{error}</p>
  {/if}
  {#if lastResult}
    <pre class="mono" style="margin-top:10px; color: var(--success); font-size:11px; white-space:pre-wrap;">{JSON.stringify(lastResult, null, 2)}</pre>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 12px;
    color: var(--text-dim);
  }
  .field input, .field select, .field textarea {
    background: var(--bg);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius);
    color: var(--text);
    padding: 6px 8px;
    font-size: 13px;
  }
  .field input:focus, .field select:focus, .field textarea:focus {
    outline: none;
    border-color: var(--accent);
  }
  .primary-btn {
    background: var(--accent-dim);
    color: var(--accent);
    border: 1px solid var(--accent);
    border-radius: var(--radius);
    padding: 8px 12px;
    font-size: 13px;
    margin-top: 4px;
  }
  .primary-btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
</style>