<script>
  import { onMount } from 'svelte';
  import { invoke } from "@tauri-apps/api/core";
  import "./lib/theme.css";
  
  import RunnersTable from "./lib/RunnersTable.svelte";
  import ServersTable from "./lib/ServersTable.svelte";
  import RunnerForm from "./lib/RunnerForm.svelte";
  import ServerForm from "./lib/ServerForm.svelte";
  import MeshPanel from "./lib/MeshPanel.svelte";
  import TailscaleBadge from "./lib/TailscaleBadge.svelte";
  import AuthPage from "./lib/AuthPage.svelte";
  import UserMenu from "./lib/UserMenu.svelte";
  
  import { auth, curTab } from "./lib/stores.js";

  const tabs = [
    { id: "admin", label: "admin" },
    { id: "user", label: "user" },
    { id: "runner", label: "runner" },
  ];

  onMount(async () => {
    // При старте приложения проверяем, есть ли сохраненная сессия
    try {
      const status = await invoke("check_auth_status");
      if (status.isLoggedIn) {
        auth.set({ isLoggedIn: true, username: status.username });
      }
    } catch (e) {
      console.error("Failed to check auth status", e);
    }
  });
</script>

<main style="min-height:100vh; padding: 20px 24px; display:flex; flex-direction:column; gap:18px;">
  <header style="display:flex; justify-content:space-between; align-items:center;">
    <div style="display:flex; align-items:center; gap:16px;">
      <span style="font-size:14px; font-weight:500; letter-spacing:0.02em;">runner manager</span>
      
      <!-- UserMenu теперь сверху слева -->
      <UserMenu /> 
      
      {#if $auth.isLoggedIn}
        <nav style="display:flex; gap:4px;">
          {#each tabs as r}
            <button
              class="main-tab"
              class:active={$curTab === r.id}
              on:click={() => ($curTab = r.id)}
            >
              {r.label}
            </button>
          {/each}
        </nav>
      {/if}
    </div>
    
    <TailscaleBadge />
  </header>

  {#if !$auth.isLoggedIn || $curTab === "auth"}
    <AuthPage />
  {:else if $curTab === "admin"}
    <div style="display:flex; gap:18px; flex-wrap:wrap; align-items:flex-start;">
      <ServerForm />
      <div style="flex:1; min-width:420px; display:flex; flex-direction:column; gap:18px;">
        <ServersTable />
        <RunnersTable />
      </div>
    </div>
  {:else if $curTab === "user"}
    <MeshPanel />
  {:else if $curTab === "runner"}
    <div style="display:flex; gap:18px; flex-wrap:wrap; align-items:flex-start;">
      <RunnerForm />
      <div style="flex:1; min-width:420px;">
        <RunnersTable />
      </div>
    </div>
  {/if}
</main>

<style>
  .main-tab {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-dim);
    border-radius: var(--radius);
    padding: 5px 12px;
    font-size: 12px;
    cursor: pointer;
  }
  .main-tab.active {
    color: var(--accent);
    border-color: var(--accent);
    background: var(--accent-dim);
  }
</style>