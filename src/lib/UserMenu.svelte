<script>
  import { auth } from "./stores.js";
  import { logout } from "./api.js";
  import AuthPage from "./AuthPage.svelte";

  let isOpen = false;

  function toggle() {
    isOpen = !isOpen;
  }
  function close() {
    isOpen = false;
  }

  // клик вне контейнера закрывает меню — надёжнее, чем hover, когда внутри форма
  function clickOutside(node) {
    function onClick(event) {
      if (!node.contains(event.target)) close();
    }
    document.addEventListener("click", onClick, true);
    return { destroy: () => document.removeEventListener("click", onClick, true) };
  }

  async function handleLogout() {
    try {
      await logout();
    } catch (e) {
      console.error("Logout error:", e);
    }
    auth.set({ isLoggedIn: false, username: "" });
    close();
  }
</script>

<div class="user-menu-container" use:clickOutside>
  {#if $auth.isLoggedIn}
    <button class="user-btn" on:click={toggle}>
      <span class="avatar">{$auth.username.charAt(0).toUpperCase()}</span>
      <span class="username">{$auth.username}</span>
      <span class="arrow" class:up={isOpen}>▼</span>
    </button>
    {#if isOpen}
      <div class="dropdown">
        <button class="dropdown-item" on:click={close}>
          ⚙️ Настройки
        </button>
        <div class="divider"></div>
        <button class="dropdown-item danger" on:click={handleLogout}>
          🚪 Выйти
        </button>
      </div>
    {/if}
  {:else}
    <button class="login-btn" on:click={toggle}>
      <span>🔑</span> Войти
    </button>
    {#if isOpen}
      <div class="dropdown auth-dropdown">
        <AuthPage on:success={close} />
      </div>
    {/if}
  {/if}
</div>

<style>
  .user-menu-container {
    position: relative;
  }
  .user-btn, .login-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    background: transparent;
    border: 1px solid var(--border, #333);
    color: var(--text, #eee);
    border-radius: var(--radius, 6px);
    padding: 6px 12px;
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s;
  }
  .user-btn:hover, .login-btn:hover {
    border-color: var(--accent, #4f8ef7);
    background: var(--accent-dim, rgba(79, 142, 247, 0.1));
    color: var(--accent, #4f8ef7);
  }
  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: var(--accent, #4f8ef7);
    color: white;
    border-radius: 50%;
    font-size: 12px;
    font-weight: 600;
  }
  .arrow {
    font-size: 10px;
    color: var(--text-dim, #888);
    transition: transform 0.2s;
  }
  .arrow.up {
    transform: rotate(180deg);
  }
  .dropdown {
    position: absolute;
    top: calc(100% + 8px);
    right: 0; /* меню теперь справа, поэтому раскрывается влево от края кнопки */
    background: var(--bg, #1a1a1a);
    border: 1px solid var(--border, #333);
    border-radius: var(--radius, 6px);
    padding: 4px;
    min-width: 180px;
    box-shadow: 0 8px 24px rgba(0,0,0,0.4);
    z-index: 100;
    animation: fadeIn 0.15s ease-out;
  }
  .auth-dropdown {
    padding: 0;
    min-width: 260px;
  }
  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }
  .dropdown-item {
    width: 100%;
    text-align: left;
    background: transparent;
    border: none;
    color: var(--text, #eee);
    padding: 8px 12px;
    border-radius: var(--radius, 6px);
    font-size: 13px;
    cursor: pointer;
  }
  .dropdown-item:hover {
    background: var(--accent-dim, rgba(79, 142, 247, 0.1));
    color: var(--accent, #4f8ef7);
  }
  .dropdown-item.danger:hover {
    background: rgba(255, 50, 50, 0.1);
    color: #ff5555;
  }
  .divider {
    height: 1px;
    background: var(--border, #333);
    margin: 4px 8px;
  }
</style>