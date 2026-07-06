<script>
  import { startServer } from "./api.js";

  let name = "";
  let image = "steamcmd:latest";
  let cpuCores = 2;
  let memoryMb = 2048;
  let submitting = false;
  let error = "";
  let lastResult = null;

  async function handleSubmit() {
    submitting = true;
    error = "";
    lastResult = null;
    try {
      lastResult = await startServer({
        name,
        image,
        cpuCores: Number(cpuCores),
        memoryMb: Number(memoryMb),
      });
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
      <span>имя сервера</span>
      <input class="mono" type="text" bind:value={name} placeholder="my-cs2-server" required />
    </label>
    <label class="field">
      <span>образ</span>
      <input class="mono" type="text" bind:value={image} />
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
      {submitting ? "запускается..." : "запустить сервер"}
    </button>
  </form>

  {#if error}
    <p style="color: var(--danger); margin-top:10px;" class="mono">{error}</p>
  {/if}
  {#if lastResult}
    <p style="color: var(--success); margin-top:10px;" class="mono">
      запущено: {lastResult.server_id}, {lastResult.address}
    </p>
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
