<script>
  import { invoke } from "@tauri-apps/api/core";
  import { auth, curTab } from "./stores.js";

  let email = "";
  let username = "";
  let password = "";
  let submitting = false;
  let error = "";
  let logLines = [];

  const tabs = [
    { id: "signup", label: "signup" },
    { id: "login", label: "login" },
  ];
  let activeTab = "login";

  async function handleLogin() {
    submitting = true;
    error = "";
    logLines = ["> logging in..."];
    try {
      const result = await invoke("login_user", {
        req: { username, password }
      });
      logLines = [...logLines, `> готово: привет, ${result.username}`];
      
      auth.set({ isLoggedIn: true, username: result.username });
      $curTab = "admin";
    } catch (e) {
      error = String(e);
      logLines = [...logLines, `> ошибка: ${e}`];
    } finally {
      submitting = false;
    }
  }

  async function handleSignUp() {
    submitting = true;
    error = "";
    logLines = ["> signing up..."];
    try {
      // Убедитесь, что добавили команду sign_up_user в Rust аналогично login_user
      const result = await invoke("sign_up_user", {
        req: { email, username, password }
      });
      logLines = [...logLines, `> готово: аккаунт создан`];
      
      auth.set({ isLoggedIn: true, username: result.username });
      $curTab = "admin";
    } catch (e) {
      error = String(e);
      logLines = [...logLines, `> ошибка: ${e}`];
    } finally {
      submitting = false;
    }
  }
</script>

  <nav style="display:flex; gap:4px;">
    {#each tabs as r}
      <button
        class="auth-tab"
        class:active={curTab === r.id}
        on:click={() => (curTab = r.id)}
      >
        {r.label}
      </button>
    {/each}
  </nav>
  {#if curTab === "signup"}
  <div class="panel" style="padding: 16px; max-width: 480px;">
    <h2 style="font-size:14px; font-weight:500; margin:0 0 12px;">Sign Up</h2>
    <form on:submit|preventDefault={handleSignUp} style="display:flex; flex-direction:column; gap:10px;">
        <label class="field">
        <span>Email</span>
        <input class="mono" type="text" bind:value={email} />
        </label>
        <label class="field">
        <span>Username</span>
        <input class="mono" type="text" bind:value={username} />
        </label>
        <label class="field">
        <span>Password</span>
        <input class="mono" type="password" bind:value={password} />
        </label>
        <button type="submit" class="primary-btn" disabled={submitting}>
        {submitting ? "запускается..." : "запустить раннер"}
        </button>    
    </form>
  </div>
  {:else if curTab === "login"}
  <div class="panel" style="padding: 16px; max-width: 480px;">
  <h2 style="font-size:14px; font-weight:500; margin:0 0 12px;">Login</h2>
    <form on:submit|preventDefault={handleLogin} style="display:flex; flex-direction:column; gap:10px;">
    <label class="field">
      <span>Username</span>
      <input class="mono" type="text" bind:value={username} />
    </label>
    <label class="field">
      <span>Password</span>
      <input class="mono" type="password" bind:value={password} />
    </label>
    <button type="submit" class="primary-btn" disabled={submitting}>
      {submitting ? "запускается..." : "запустить раннер"}
    </button>
  </form>
  </div>
  {/if}


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
  .auth-tab {
    background: transparent;
    border: 1px solid var(--border);
    color: var(--text-dim);
    border-radius: var(--radius);
    padding: 5px 12px;
    font-size: 12px;
  }
  .auth-tab.active {
    color: var(--accent);
    border-color: var(--accent);
    background: var(--accent-dim);
  }
</style>
