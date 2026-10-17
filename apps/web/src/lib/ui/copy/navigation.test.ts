import { describe, expect, it } from 'vitest';
import { MODULES, UPCOMING_MODULES } from './navigation';

const topics = MODULES.flatMap((module) => module.topics);
const pages = import.meta.glob<string>('/src/routes/**/+page.svelte', {
  query: '?raw',
  import: 'default',
  eager: true,
});

describe('Laboratory navigation contract', () => {
  it('links to seven distinct, implemented laboratory pages', () => {
    expect(topics).toHaveLength(7);
    expect(new Set(topics.map((topic) => topic.href)).size).toBe(7);
    for (const topic of topics) {
      expect(topic.href).toMatch(/^\/lab\/(calculo|vetores)\//);
      expect(pages[`/src/routes${topic.href}/+page.svelte`], topic.href).toBeDefined();
    }
  });

  it('keeps a static redirect entry for every former topic URL', () => {
    for (const topic of topics) {
      const legacyPath = topic.href.replace(/^\/lab/, '');
      expect(pages[`/src/routes${legacyPath}/+page.svelte`]).toContain(
        `<LegacyRedirect destination="${topic.href}" />`,
      );
    }
  });

  it('identifies future product modules without advertising routes', () => {
    expect(UPCOMING_MODULES.map((module) => module.title)).toEqual([
      'Exercícios',
      'Autoria',
      'Biblioteca',
      'Revisão',
      'Plano de estudos',
      'Progresso',
      'Conta e comunidade',
    ]);
    for (const module of UPCOMING_MODULES) {
      expect(module.phase).toMatch(/^F[1-4]/);
      expect(module).not.toHaveProperty('href');
    }
  });
});
