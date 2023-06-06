<script lang="ts">
  import type { Scene } from '$lib/scenedatabase';

  export let selectedScenes: Scene[];
  export let selected: number;

  let windowHeight: number; // bound to the height of the window

  // select another element
  function select(id: number) {
    if (id >= selectedScenes.length) id = selectedScenes.length - 1;
    else if (id < 0) id = 0;

    let target = <HTMLInputElement | null>document.getElementById('sc' + id);

    if (target) {
      target.checked = true;
      target.focus();
      selected = id;
    }
  }

  // how many elements should we scroll up/down if PgUp/PgDown is pressed
  function scrollStep(): number {
    return Math.round(windowHeight / document.getElementById('lsc0')!.offsetHeight);
  }

  function onKeyDown(e: KeyboardEvent) {
    switch (e.key) {
      case 'Home':
        select(0);
        break;
      case 'End':
        select(selectedScenes.length - 1);
        break;
      case 'ArrowUp':
      case 'Up':
        select(selected - 1);
        break;
      case 'ArrowDown':
      case 'Down':
        select(selected + 1);
        break;
      case 'PageDown':
        select(selected + scrollStep());
        break;
      case 'PageUp':
        select(selected - scrollStep());
        break;
      default:
        return; // no preventDefault
    }
    e.preventDefault();
  }
</script>

<svelte:window bind:innerHeight={windowHeight} />

<ul on:keydown={onKeyDown}>
  {#each selectedScenes as scene, index}
    <li id="lsc{index}">
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
