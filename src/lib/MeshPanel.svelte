<script>
  import { joinMesh } from "./api.js";
  import TailscaleBadge from "./TailscaleBadge.svelte";

  let submitting = false;
  let error = "";

  async function handleJoin() {
    submitting = true;
    error = "";
    try {
      await joinMesh();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }
</script>

<div class="panel" style="padding: 16px; max-width: 420px;">
  <h2 style="font-size:14px; font-weight:500; margin:0 0 12px;">Подключение к сети</h2>
  <div style="margin-bottom: 14px;">
    <TailscaleBadge />
  </div>
  <button class="primary-btn" on:click={handleJoin} disabled={submitting}>
    {submitting ? "подключаюсь..." : "подключиться к mesh"}
  </button>
  {#if error}
    <p style="color: var(--danger); margin-top:10px;" class="mono">{error}</p>
  {/if}
</div>

<style>
  .primary-btn {
    background: var(--accent-dim);
    color: var(--accent);
    border: 1px solid var(--accent);
    border-radius: var(--radius);
    padding: 8px 12px;
    font-size: 13px;
  }
  .primary-btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
</style>
