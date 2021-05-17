import { writable } from 'svelte/store';

export enum Language {
  English = 'English',
  Chinese = '中文',
}

export const selectedLanguage = writable(localStorage.selectedLanguage);
selectedLanguage.subscribe(value => (localStorage.selectedLanguage = value));
