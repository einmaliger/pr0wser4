<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import { realLength, type Scene, type SceneDatabase } from '$lib/scenedatabase';
  import { EmptyScene } from '$lib/scenedatabase';
  import SceneInfo from './sceneinfo.svelte';
  import { SceneFilter } from '$lib/scenefilter';

  let db: SceneDatabase | null = null,
    filter = new SceneFilter(),
    filterString = 'score>=73',
    selectedScenes: Scene[] = [];

  function filterChangeEvent() {
    if (db) {
      let oldSelection = selection;
      filter.parse(filterString);
      selected = -1;
      selectedScenes = db.film.filter((a) => filter.matches(a)); // TODO: understand why can we not simple pass filter.matches as argument?
      if (oldSelection) {
        selected = selectedScenes.indexOf(oldSelection);
      }
    }
  }

  let selected = -1;

  // Shortcut for selected scene
  $: selection = selected >= 0 ? selectedScenes[selected] : EmptyScene;

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      invoke('play', {
        baseDir: db?.base_dir || '',
        directory: selection.directory,
        fileName: selection.file_name,
        begin: selection.begin || -1,
        length: selection.end ? realLength(selection) : -1
      });
    }
  }

  getMatches().then((matches) => {
    let filename = '';
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
      filterChangeEvent();
    });
  });
</script>

<div style="width: 40%;">
  <input type="text" style="width:90%" bind:value={filterString} on:input={filterChangeEvent} />
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
