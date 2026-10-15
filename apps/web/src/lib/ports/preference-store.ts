/** Port: small per-user preferences (theme, panel state) that survive reloads. */
export interface PreferenceStore {
  read(key: string): string | null;
  write(key: string, value: string): void;
}
