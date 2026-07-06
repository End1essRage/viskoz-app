<script>
  import { onMount, onDestroy } from "svelte";
  import { tailscaleStatus } from "./api.js";

  let status = "checking";
  let ip = "";
  let interval;

  async function poll() {
    try {
      const res = await tailscaleStatus();
      status = res.connected ? "connected" : "disconnected";
      ip = res.tailscale_ip || "";
    } catch (e) {
      status = "error";
      ip = "";
    }
  }

  onMount(() => {
    poll();
    interval = setInterval(poll, 4000);
  });

  onDestroy(() => clearInterval(interval));

  const colors = {
    connected: "var(--success)",
    disconnected: "var(--text-faint)",
    checking: "var(--warning)",
    error: "var(--danger)",
  };

  const labels = {
    connected: "tailscale подключен",
    disconnected: "tailscale отключен",
    checking: "проверка...",
    error: "ошибка статуса",
  };
</script>

<div class="pill">
  <span class="pill-dot" style="background: {colors[status]}"></span>
  <span>{labels[status]}</span>
  {#if ip}
    <span style="color: var(--text-dim)">{ip}</span>
  {/if}
</div>
