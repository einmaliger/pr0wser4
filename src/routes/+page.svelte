<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import type { Scene, SceneDatabase } from '$lib/scenedatabase';
  import { EmptyScene } from '$lib/scenedatabase';
  import SceneInfo from './sceneinfo.svelte';
  import { AtomicSceneFilter } from '$lib/scenefilter';

  let db: SceneDatabase | null = null,
    filter = new AtomicSceneFilter(),
    filterString = 'score>=73',
    selectedScenes: Scene[] = [];

  $: {
    if (db) {
      filter.parse(filterString);
      selectedScenes = db.film.filter((a) => filter.matches(a)); // TODO: understand why can we not simple pass filter.matches as argument?
    }
  }

  let selected = -1,
    selection: Scene;

  $: selection = selected >= 0 ? selectedScenes[selected] : EmptyScene;

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
    let filename = 'test.pr0';
    const database = matches.args.database;
    if (database.occurrences > 0) {
      filename = database.value as string;
    }
    invoke('load', { path: filename }).then((r) => {
      db = r as SceneDatabase;
      db.film.sort((a, b) => {
        const aname = a.name || a.file_name;
        const bname = b.name || b.file_name;
        return aname.localeCompare(bname);
      });
    });
  });
</script>

<div style="width: 40%;">
  <input type="text" style="width:90%" bind:value={filterString} />
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
