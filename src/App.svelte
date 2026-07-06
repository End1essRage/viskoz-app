<script>
  import "./lib/theme.css";
  import RunnersTable from "./lib/RunnersTable.svelte";
  import ServersTable from "./lib/ServersTable.svelte";
  import RunnerForm from "./lib/RunnerForm.svelte";
  import ServerForm from "./lib/ServerForm.svelte";
  import MeshPanel from "./lib/MeshPanel.svelte";
  import TailscaleBadge from "./lib/TailscaleBadge.svelte";

  const roles = [
    { id: "admin", label: "admin" },
    { id: "user", label: "user" },
    { id: "runner", label: "runner" },
  ];

  let role = "admin";
</script>

<main style="min-height:100vh; padding: 20px 24px; display:flex; flex-direction:column; gap:18px;">
  <header style="display:flex; justify-content:space-between; align-items:center;">
    <div style="display:flex; align-items:center; gap:16px;">
      <span style="font-size:14px; font-weight:500; letter-spacing:0.02em;">runner manager</span>
      <nav style="display:flex; gap:4px;">
        {#each roles as r}
          <button
            class="role-tab"
            class:active={role === r.id}
            on:click={() => (role = r.id)}
          >
            {r.label}
          </button>
        {/each}
      </nav>
    </div>
    <TailscaleBadge />
  </header>

  {#if role === "admin"}
    <div style="display:flex; gap:18px; flex-wrap:wrap; align-items:flex-start;">
      <ServerForm />
      <div style="flex:1; min-width:420px; display:flex; flex-direction:column; gap:18px;">
        <ServersTable />
        <RunnersTable />
      </div>
    </div>
  {:else if role === "user"}
    <MeshPanel />
  {:else if role === "runner"}
    <div style="display:flex; gap:18px; flex-wrap:wrap; align-items:flex-start;">
      <RunnerForm />
      <div style="flex:1; min-width:420px;">
        <RunnersTable />
      </div>
    </div>
  {/if}
</main>

<style>
  .role-tab {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-dim);
    border-radius: var(--radius);
    padding: 5px 12px;
    font-size: 12px;
  }
  .role-tab.active {
    color: var(--accent);
    border-color: var(--accent);
    background: var(--accent-dim);
  }
</style>
