import type { PreferenceStore } from '$lib/ports/preference-store';

const KEY_PREFIX = 'math-view:';

/**
 * Browser `localStorage` adapter. Storage may be unavailable (private mode,
 * blocked cookies), so every access degrades to "no preference saved".
 */
export class LocalPreferenceStore implements PreferenceStore {
  read(key: string): string | null {
    try {
      return window.localStorage.getItem(KEY_PREFIX + key);
    } catch {
      return null;
    }
  }

  write(key: string, value: string): void {
    try {
      window.localStorage.setItem(KEY_PREFIX + key, value);
    } catch {
      // Preferences are a convenience; losing one is acceptable.
    }
  }
}
