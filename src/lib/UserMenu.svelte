<script>
  import { auth, curTab } from "./stores.js";
  import { invoke } from "@tauri-apps/api/core";

  let isOpen = false;

  async function handleLogout() {
    try {
      await invoke("logout_user");
    } catch (e) {
      console.error("Logout error:", e);
    }
    auth.set({ isLoggedIn: false, username: "" });
    $curTab = "admin";
    isOpen = false;
  }
</script>

<div 
  class="user-menu-container" 
  on:mouseenter={() => isOpen = true} 
  on:mouseleave={() => isOpen = false}
>
  {#if $auth.isLoggedIn}
    <button class="user-btn">
      <span class="avatar">{$auth.username.charAt(0).toUpperCase()}</span>
      <span class="username">{$auth.username}</span>
      <span class="arrow">▼</span>
    </button>
    
    {#if isOpen}
      <div class="dropdown">
        <button class="dropdown-item" on:click={() => { /* логика настроек */ }}>
          ⚙️ Настройки
        </button>
        <div class="divider"></div>
        <button class="dropdown-item danger" on:click={handleLogout}>
          🚪 Выйти
        </button>
      </div>
    {/if}
  {:else}
    <button class="login-btn" on:click={() => $curTab = "auth"}>
      <span>🔑</span> Войти
    </button>
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
  .user-btn:hover .arrow {
    transform: rotate(180deg);
  }
  .dropdown {
    position: absolute;
    top: calc(100% + 8px);
    left: 0;
    background: var(--bg, #1a1a1a);
    border: 1px solid var(--border, #333);
    border-radius: var(--radius, 6px);
    padding: 4px;
    min-width: 180px;
    box-shadow: 0 8px 24px rgba(0,0,0,0.4);
    z-index: 100;
    animation: fadeIn 0.15s ease-out;
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