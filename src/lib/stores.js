import { writable } from 'svelte/store';

export const auth = writable({
  isLoggedIn: false,
  username: '',
});

export const curTab = writable('admin');