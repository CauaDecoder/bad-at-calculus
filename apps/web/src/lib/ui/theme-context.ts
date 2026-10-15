/** Shares the theme controller created in `+layout.svelte` with every page. */
import { getContext, setContext } from 'svelte';
import type { ThemeController } from '$lib/application/theme.svelte';

const THEME_KEY = Symbol('theme');

export function provideTheme(theme: ThemeController): void {
  setContext(THEME_KEY, theme);
}

export function useTheme(): ThemeController {
  return getContext<ThemeController>(THEME_KEY);
}
