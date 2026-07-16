<script>
  import { createEventDispatcher } from "svelte";
  import { auth } from "./stores.js";
  import { login, signUp } from "./api.js";

  const dispatch = createEventDispatcher();

  let mode = "login"; // "login" | "signup"
  let email = "";
  let username = "";
  let password = "";
  let submitting = false;
  let error = "";

  async function handleLogin() {
    submitting = true;
    error = "";
    try {
      const result = await login(email, password);
      auth.set({ isLoggedIn: true, username: result.username });
      dispatch("success");
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function handleSignUp() {
    submitting = true;
    error = "";
    try {
      await signUp(email, username, password);
      // после регистрации сразу логинимся, чтобы не заставлять юзера
      // вводить пароль второй раз
      const result = await login(email, password);
      auth.set({ isLoggedIn: true, username: result.username });
      dispatch("success");
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }
</script>

<div class="auth-panel">
  <nav class="auth-tabs">
    <button class="auth-tab" class:active={mode === "login"} on:click={() => (mode = "login")}>
      login
    </button>
    <button class="auth-tab" class:active={mode === "signup"} on:click={() => (mode = "signup")}>
      signup
    </button>
  </nav>

  {#if mode === "login"}
    <form on:submit|preventDefault={handleLogin} class="auth-form">
      <label class="field">
        <span>Email</span>
        <input class="mono" type="text" bind:value={email} autocomplete="username" />
      </label>
      <label class="field">
        <span>Password</span>
        <input class="mono" type="password" bind:value={password} autocomplete="current-password" />
      </label>
      {#if error}<p class="error">{error}</p>{/if}
      <button type="submit" class="primary-btn" disabled={submitting}>
        {submitting ? "вход..." : "войти"}
      </button>
    </form>
  {:else}
    <form on:submit|preventDefault={handleSignUp} class="auth-form">
      <label class="field">
        <span>Email</span>
        <input class="mono" type="text" bind:value={email} autocomplete="email" />
      </label>
      <label class="field">
        <span>Username</span>
        <input class="mono" type="text" bind:value={username} autocomplete="username" />
      </label>
      <label class="field">
        <span>Password</span>
        <input class="mono" type="password" bind:value={password} autocomplete="new-password" />
      </label>
      {#if error}<p class="error">{error}</p>{/if}
      <button type="submit" class="primary-btn" disabled={submitting}>
        {submitting ? "создаём..." : "зарегистрироваться"}
      </button>
    </form>
  {/if}
</div>

<style>
  .auth-panel {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 12px;
    min-width: 240px;
  }
  .auth-tabs {
    display: flex;
    gap: 4px;
    margin-bottom: 2px;
  }
  .auth-tab {
    flex: 1;
    background: transparent;
    border: 1px solid var(--border, #333);
    color: var(--text-dim, #888);
    border-radius: var(--radius, 6px);
    padding: 5px 0;
    font-size: 12px;
    cursor: pointer;
  }
  .auth-tab.active {
    color: var(--accent, #4f8ef7);
    border-color: var(--accent, #4f8ef7);
    background: var(--accent-dim, rgba(79, 142, 247, 0.1));
  }
  .auth-form {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 12px;
    color: var(--text-dim, #888);
  }
  .field input {
    background: var(--bg, #111);
    border: 1px solid var(--border-strong, #444);
    border-radius: var(--radius, 6px);
    color: var(--text, #eee);
    padding: 6px 8px;
    font-size: 13px;
  }
  .field input:focus {
    outline: none;
    border-color: var(--accent, #4f8ef7);
  }
  .primary-btn {
    background: var(--accent-dim, rgba(79, 142, 247, 0.1));
    color: var(--accent, #4f8ef7);
    border: 1px solid var(--accent, #4f8ef7);
    border-radius: var(--radius, 6px);
    padding: 8px 12px;
    font-size: 13px;
  }
  .primary-btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
  .error {
    font-size: 12px;
    color: #ff5555;
    margin: 0;
  }
</style>