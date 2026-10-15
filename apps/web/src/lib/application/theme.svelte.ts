import type { PreferenceStore } from '$lib/ports/preference-store';

export type Theme = 'light' | 'dark';

const THEME_KEY = 'theme';

/**
 * Light/dark preference. Starts from the saved choice, or from the system
 * preference when nothing was saved. Applying it to the DOM is the UI's job.
 */
export class ThemeController {
  theme = $state<Theme>('light');

  private readonly store: PreferenceStore;

  constructor(store: PreferenceStore, systemPrefersDark: boolean) {
    this.store = store;
    const saved = store.read(THEME_KEY);
    this.theme = isTheme(saved) ? saved : systemPrefersDark ? 'dark' : 'light';
  }

  toggle(): void {
    this.theme = this.theme === 'dark' ? 'light' : 'dark';
    this.store.write(THEME_KEY, this.theme);
  }
}

function isTheme(value: string | null): value is Theme {
  return value === 'light' || value === 'dark';
}
