<!--
  Minimal inline icon set. Paths from Lucide (https://lucide.dev, ISC license),
  24×24 grid, stroke-based. Add new icons to ICON_PATHS.
-->
<script lang="ts" module>
  const ICON_PATHS = {
    plus: ['M5 12h14', 'M12 5v14'],
    minus: ['M5 12h14'],
    scan: [
      'M3 7V5a2 2 0 0 1 2-2h2',
      'M17 3h2a2 2 0 0 1 2 2v2',
      'M21 17v2a2 2 0 0 1-2 2h-2',
      'M7 21H5a2 2 0 0 1-2-2v-2',
    ],
    sun: [
      'M12 8a4 4 0 1 0 0 8a4 4 0 1 0 0-8',
      'M12 2v2',
      'M12 20v2',
      'm4.93 4.93 1.41 1.41',
      'm17.66 17.66 1.41 1.41',
      'M2 12h2',
      'M20 12h2',
      'm6.34 17.66-1.41 1.41',
      'm19.07 4.93-1.41 1.41',
    ],
    moon: ['M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z'],
    calculus: ['M3 17c3 0 4-10 9-10s6 10 9 10'],
    vectors: ['M5 19 19 5', 'M9 5h10v10'],
    geometry: ['M12 3 21 19H3Z'],
    matrix: ['M4 4h6v6H4z', 'M14 4h6v6h-6z', 'M4 14h6v6H4z', 'M14 14h6v6h-6z'],
    sigma: ['M18 7V4H6l6 8-6 8h12v-3'],
    atom: [
      'M12 12h.01',
      'M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9C11.17 3.78 5.83 1.76 3.8 3.8c-2.03 2.03-.02 7.36 4.5 11.9 4.53 4.52 9.86 6.54 11.9 4.5Z',
      'M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.53-6.54 9.86-4.5 11.9 2.03 2.03 7.36.02 11.9-4.5Z',
    ],
    alert: ['M12 8v4', 'M12 16h.01', 'M12 3a9 9 0 1 0 0 18a9 9 0 1 0 0-18'],
    check: ['M20 6 9 17l-5-5'],
    cross: ['M18 6 6 18', 'm6 6 12 12'],
    home: ['m3 10 9-7 9 7', 'M5 10v10h14V10', 'M10 20v-6h4v6'],
    lightbulb: ['M9 18h6', 'M10 22h4', 'M12 2a7 7 0 0 0-4 12.7V17h8v-2.3A7 7 0 0 0 12 2Z'],
    chevronRight: ['m9 18 6-6-6-6'],
    area: ['M3 20h18', 'M3 20c3-10 6-14 9-14s6 4 9 14'],
    limit: ['M3 12h7', 'M14 12h7', 'M12 12h.01'],
    derivative: ['M3 17c3 0 4-10 9-10s6 10 9 10', 'M4 4l16 8'],
    plane: ['M2 16 8 6h14l-6 10Z'],
    system: ['M4 6h10', 'M4 12h10', 'M4 18h10', 'M18 6v12'],
  } as const;

  export type IconName = keyof typeof ICON_PATHS;
</script>

<script lang="ts">
  interface Props {
    name: IconName;
    size?: number;
  }

  let { name, size = 16 }: Props = $props();
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.75"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {#each ICON_PATHS[name] as path (path)}
    <path d={path} />
  {/each}
</svg>
