<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import type { Scene, SceneDatabase } from '$lib/scenedatabase';
  import { EmptyScene } from '$lib/scenedatabase';
  import SceneInfo from './sceneinfo.svelte';

  let db: SceneDatabase | null = null,
    selected = -1,
    selectedScenes: Scene[] = [],
    selection: Scene;

  $: selection = selected >= 0 ? selectedScenes[selected] : EmptyScene;

  let filename = 'test.pr0';

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      invoke('play', {
        baseDir: db?.base_dir || '',
        directory: selection.directory,
        fileName: selection.file_name
      });
    }
  }

  getMatches().then((matches) => {
    const database = matches.args.database;
    if (database.occurrences > 0) {
      filename = database.value as string;
    }
    invoke('load', { path: filename }).then((r) => {
      db = r as SceneDatabase;
      selectedScenes = db.film;
      selectedScenes.sort((a, b) => {
        const aname = a.name || a.file_name;
        const bname = b.name || b.file_name;
        return aname.localeCompare(bname);
      });
    });
  });
</script>

<div style="width: 40%;">
  <ul on:keydown={onKeyDown}>
    {#each selectedScenes as scene, index}
      <li>
        <input
          type="radio"
          name="scenelist"
          value={index}
          id="sc{index}"
          bind:group={selected}
        /><label for="sc{index}">{scene.name || scene.file_name}</label>
      </li>
    {/each}
  </ul>
</div>
<div style="width: 60%; position: fixed; right: 0; top: 0;">
  <SceneInfo base_dir={db?.base_dir || ''} {selection} />
</div>

<style>
  ul {
    padding: 0;
  }
  li {
    list-style: none;
  }
  input[type='radio'] {
    appearance: none;
    -webkit-appearance: none;
  }
  input:checked + label {
    background: #888;
  }
</style>
