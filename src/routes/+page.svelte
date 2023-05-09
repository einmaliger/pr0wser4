<script lang="ts">
  import { getMatches } from '@tauri-apps/api/cli';
  import { invoke } from '@tauri-apps/api/tauri';
  import { realLength, type Scene, type SceneDatabase } from '$lib/scenedatabase';
  import { EmptyScene } from '$lib/scenedatabase';
  import SceneInfo from './sceneinfo.svelte';
  import SceneSelector from './sceneselector.svelte';
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

  // global key events
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

<svelte:window on:keydown={onKeyDown} />

<input type="text" bind:value={filterString} on:input={filterChangeEvent} />

<div style="width: 40%; padding-top: 1rem;">
  <SceneSelector {selectedScenes} bind:selected />
</div>
<div style="width: 60%; position: fixed; right: 0; top: 0;">
  <SceneInfo base_dir={db?.base_dir || ''} {selection} />
</div>

<style>
  input {
    position: fixed;
    width: 35%;
  }
</style>
