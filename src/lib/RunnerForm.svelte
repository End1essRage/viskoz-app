<script>
  import { startRunner } from "./api.js";

  let cpAddress="cp.viskoz.dev:9096";
  let hostDataPath = "/srv/runner-data";
  let cpuCores = 4;
  let memoryMb = 4096;
  let submitting = false;
  let logLines = [];
  let error = "";

  async function handleSubmit() {
    submitting = true;
    error = "";
    logLines = ["> запускаю раннер..."];
    try {
      const result = await startRunner({
        cpAddress,
        hostDataPath,
        cpuCores: Number(cpuCores),
        memoryMb: Number(memoryMb),
      });
      logLines = [...logLines, ...(result.log_lines || []), `> готово: ${result.runner_id}`];
    } catch (e) {
      error = String(e);
      logLines = [...logLines, `> ошибка: ${e}`];
    } finally {
      submitting = false;
    }
  }
</script>

<div class="panel" style="padding: 16px; max-width: 480px;">
  <h2 style="font-size:14px; font-weight:500; margin:0 0 12px;">Запустить раннер</h2>

  <form on:submit|preventDefault={handleSubmit} style="display:flex; flex-direction:column; gap:10px;">
    <label class="field">
      <span>адресc control-plane</span>
      <input class="mono" type="text" bind:value={cpAddress} />
    </label>
    <label class="field">
      <span>путь к данным на хосте</span>
      <input class="mono" type="text" bind:value={hostDataPath} />
    </label>
    <div style="display:flex; gap:10px;">
      <label class="field" style="flex:1;">
        <span>cpu cores</span>
        <input class="mono" type="number" min="1" bind:value={cpuCores} />
      </label>
      <label class="field" style="flex:1;">
        <span>memory (mb)</span>
        <input class="mono" type="number" min="256" step="256" bind:value={memoryMb} />
      </label>
    </div>
    <button type="submit" class="primary-btn" disabled={submitting}>
      {submitting ? "запускается..." : "запустить раннер"}
    </button>
  </form>

  {#if error}
    <p style="color: var(--danger); margin-top:10px;" class="mono">{error}</p>
  {/if}

  {#if logLines.length > 0}
    <pre class="mono" style="margin-top:12px; padding:10px; background: var(--bg); border:1px solid var(--border); border-radius: var(--radius); font-size:11px; color: var(--text-dim); max-height:160px; overflow:auto;">{logLines.join("\n")}</pre>
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
  .field input {
    background: var(--bg);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius);
    color: var(--text);
    padding: 6px 8px;
    font-size: 13px;
  }
  .field input:focus {
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
